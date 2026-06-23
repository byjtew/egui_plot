use eframe::egui;
use eframe::egui::Response;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use egui_plot::PlotPoints;
use egui_plot::Polygon;
use egui_plot::Text;

struct Node {
    name: &'static str,
    value: f64,
    children: &'static [Self],
}

impl Node {
    const fn leaf(name: &'static str, value: f64) -> Self {
        Self {
            name,
            value,
            children: &[],
        }
    }

    const fn branch(name: &'static str, children: &'static [Self]) -> Self {
        Self {
            name,
            value: 0.0,
            children,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn weight(&self) -> f64 {
        if self.is_leaf() {
            self.value
        } else {
            self.children.iter().map(Self::weight).sum()
        }
    }
}

/// Sample hierarchy analogous to D3's flare software-package dataset.
const HIERARCHY: &[Node] = &[
    Node::branch(
        "analytics",
        &[
            Node::leaf("cluster", 35.0),
            Node::leaf("graph", 28.0),
            Node::leaf("optimization", 19.0),
            Node::leaf("regression", 14.0),
        ],
    ),
    Node::branch(
        "graphics",
        &[
            Node::leaf("render", 42.0),
            Node::leaf("shader", 22.0),
            Node::leaf("texture", 16.0),
            Node::leaf("sprite", 11.0),
            Node::leaf("camera", 9.0),
        ],
    ),
    Node::branch(
        "io",
        &[
            Node::leaf("network", 31.0),
            Node::leaf("filesystem", 24.0),
            Node::leaf("serialize", 18.0),
        ],
    ),
    Node::branch(
        "ui",
        &[
            Node::leaf("layout", 26.0),
            Node::leaf("widgets", 33.0),
            Node::leaf("events", 15.0),
            Node::leaf("theme", 12.0),
            Node::leaf("animation", 20.0),
        ],
    ),
];

const CATEGORY_COLORS: &[egui::Color32] = &[
    egui::Color32::from_rgb(31, 119, 180),
    egui::Color32::from_rgb(255, 127, 14),
    egui::Color32::from_rgb(44, 160, 44),
    egui::Color32::from_rgb(148, 103, 189),
];

/// Layout-space rectangle where `y` grows downward, like D3.
#[derive(Clone, Copy)]
struct Rect {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

impl Rect {
    fn width(&self) -> f64 {
        self.x1 - self.x0
    }

    fn height(&self) -> f64 {
        self.y1 - self.y0
    }
}

struct Cell {
    rect: Rect,
    name: &'static str,
    value: f64,
    category: usize,
}

#[derive(Clone, Copy)]
struct Item {
    weight: f64,
    name: &'static str,
    value: f64,
    category: usize,
}

/// Worst (largest) aspect ratio of `row` laid out along `side`; the metric squarify minimizes.
fn worst_ratio(row: &[Item], row_sum: f64, side: f64) -> f64 {
    if row.is_empty() || row_sum <= 0.0 {
        return f64::INFINITY;
    }
    let s2 = row_sum * row_sum;
    let side2 = side * side;
    let max = row.iter().map(|it| it.weight).fold(0.0_f64, f64::max);
    let min = row.iter().map(|it| it.weight).fold(f64::INFINITY, f64::min);
    let a = (side2 * max) / s2;
    let b = s2 / (side2 * min);
    a.max(b)
}

/// Squarified treemap layout (Bruls, Huizing & van Wijk); `items` weights must already sum to the area of `bounds`.
fn squarify(mut items: &[Item], mut bounds: Rect, out: &mut Vec<Cell>) {
    while !items.is_empty() {
        // Lay rows along the shorter side for better aspect ratios.
        let side = bounds.width().min(bounds.height());

        let mut row_len = 1usize;
        let mut row_sum = items[0].weight;
        let mut best = worst_ratio(&items[0..1], row_sum, side);
        while row_len < items.len() {
            let next_sum = row_sum + items[row_len].weight;
            let next = worst_ratio(&items[0..=row_len], next_sum, side);
            if next > best {
                break;
            }
            best = next;
            row_sum = next_sum;
            row_len += 1;
        }

        let row = &items[0..row_len];
        place_row(row, row_sum, &mut bounds, out);
        items = &items[row_len..];
    }
}

fn place_row(row: &[Item], row_sum: f64, bounds: &mut Rect, out: &mut Vec<Cell>) {
    let w = bounds.width();
    let h = bounds.height();
    if w >= h {
        let strip_w = if h > 0.0 { row_sum / h } else { 0.0 };
        let mut y = bounds.y0;
        for it in row {
            let cell_h = if row_sum > 0.0 { h * (it.weight / row_sum) } else { 0.0 };
            push_cell(out, it, bounds.x0, y, bounds.x0 + strip_w, y + cell_h);
            y += cell_h;
        }
        bounds.x0 += strip_w;
    } else {
        let strip_h = if w > 0.0 { row_sum / w } else { 0.0 };
        let mut x = bounds.x0;
        for it in row {
            let cell_w = if row_sum > 0.0 { w * (it.weight / row_sum) } else { 0.0 };
            push_cell(out, it, x, bounds.y0, x + cell_w, bounds.y0 + strip_h);
            x += cell_w;
        }
        bounds.y0 += strip_h;
    }
}

fn push_cell(out: &mut Vec<Cell>, it: &Item, x0: f64, y0: f64, x1: f64, y1: f64) {
    out.push(Cell {
        rect: Rect { x0, y0, x1, y1 },
        name: it.name,
        value: it.value,
        category: it.category,
    });
}

fn layout() -> Vec<Cell> {
    let total: f64 = HIERARCHY.iter().map(Node::weight).sum();
    let area = 1.0;

    let level1: Vec<Item> = HIERARCHY
        .iter()
        .enumerate()
        .map(|(category, node)| Item {
            weight: node.weight() / total * area,
            name: node.name,
            value: node.weight(),
            category,
        })
        .collect();

    let mut category_rects = Vec::new();
    squarify(
        &level1,
        Rect {
            x0: 0.0,
            y0: 0.0,
            x1: 1.0,
            y1: 1.0,
        },
        &mut category_rects,
    );

    let mut cells = Vec::new();
    for cat_cell in &category_rects {
        let node = &HIERARCHY[cat_cell.category];
        let rect = cat_cell.rect;
        let cat_area = rect.width() * rect.height();
        let cat_total: f64 = node.children.iter().map(Node::weight).sum();
        if cat_total <= 0.0 {
            continue;
        }

        let leaves: Vec<Item> = node
            .children
            .iter()
            .map(|leaf| Item {
                weight: leaf.weight() / cat_total * cat_area,
                name: leaf.name,
                value: leaf.weight(),
                category: cat_cell.category,
            })
            .collect();

        squarify(&leaves, rect, &mut cells);
    }

    cells
}

pub struct TreemapExample {
    show_labels: bool,
    label_size: f32,
    fill_alpha: u8,
    border_width: f32,
}

impl Default for TreemapExample {
    fn default() -> Self {
        Self {
            show_labels: true,
            label_size: 12.0,
            fill_alpha: 160,
            border_width: 1.5,
        }
    }
}

impl TreemapExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Labels:");
                ui.checkbox(&mut self.show_labels, "show labels");
            });
            ui.vertical(|ui| {
                ui.label("Label size:");
                ui.add(egui::Slider::new(&mut self.label_size, 6.0..=24.0).text("pt"));
            });
            ui.vertical(|ui| {
                ui.label("Fill opacity:");
                ui.add(egui::Slider::new(&mut self.fill_alpha, 0..=255).text("alpha"));
            });
            ui.vertical(|ui| {
                ui.label("Border:");
                ui.add(egui::Slider::new(&mut self.border_width, 0.0..=4.0).text("width"));
            });
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let cells = layout();

        Plot::new("Treemap Demo")
            .data_aspect(1.0)
            .show_axes(false)
            .show_grid(false)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show_x(false)
            .show_y(false)
            .show(ui, |plot_ui| {
                for cell in &cells {
                    let base = CATEGORY_COLORS[cell.category % CATEGORY_COLORS.len()];
                    let fill = egui::Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), self.fill_alpha);

                    // Treemap y grows downward but the plot's y grows upward, so negate y to read top-down.
                    let r = cell.rect;
                    let corners = vec![[r.x0, -r.y0], [r.x1, -r.y0], [r.x1, -r.y1], [r.x0, -r.y1]];

                    plot_ui.add(
                        Polygon::new(cell.name, PlotPoints::new(corners))
                            .fill_color(fill)
                            .stroke(egui::Stroke::new(self.border_width, egui::Color32::WHITE))
                            .allow_hover(true),
                    );

                    if self.show_labels {
                        let pad = 0.004;
                        let pos = PlotPoint::new(r.x0 + pad, -(r.y0 + pad));
                        let label =
                            egui::RichText::new(format!("{} ({})", cell.name, cell.value as i64)).size(self.label_size);
                        plot_ui.add(
                            Text::new(cell.name, pos, label)
                                .color(egui::Color32::WHITE)
                                .anchor(egui::Align2::LEFT_TOP),
                        );
                    }
                }
            })
            .response
    }
}
