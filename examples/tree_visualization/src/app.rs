use std::collections::HashMap;
use std::f64::consts::FRAC_PI_2;
use std::f64::consts::PI;

use eframe::egui;
use eframe::egui::Response;
use egui_plot::Line;
use egui_plot::MarkerShape;
use egui_plot::Plot;
use egui_plot::PlotPoint;
use egui_plot::Points;
use egui_plot::Text;

struct NodeDef {
    id: &'static str,
    parents: &'static [&'static str],
}

/// Programming-language influence DAG; multi-parent nodes are what make it "tangled".
const LEVELS: &[&[NodeDef]] = &[
    &[
        NodeDef {
            id: "Lisp",
            parents: &[],
        },
        NodeDef {
            id: "ALGOL",
            parents: &[],
        },
        NodeDef {
            id: "Fortran",
            parents: &[],
        },
    ],
    &[
        NodeDef {
            id: "Smalltalk",
            parents: &["Lisp", "ALGOL"],
        },
        NodeDef {
            id: "C",
            parents: &["ALGOL", "Fortran"],
        },
        NodeDef {
            id: "Simula",
            parents: &["ALGOL"],
        },
    ],
    &[
        NodeDef {
            id: "C++",
            parents: &["C", "Simula", "Smalltalk"],
        },
        NodeDef {
            id: "Scheme",
            parents: &["Lisp"],
        },
        NodeDef {
            id: "Objective-C",
            parents: &["C", "Smalltalk"],
        },
    ],
    &[
        NodeDef {
            id: "Java",
            parents: &["C++", "Objective-C"],
        },
        NodeDef {
            id: "Python",
            parents: &["C", "Scheme"],
        },
    ],
    &[
        NodeDef {
            id: "C#",
            parents: &["C++", "Java"],
        },
        NodeDef {
            id: "Rust",
            parents: &["C++", "Python"],
        },
        NodeDef {
            id: "Go",
            parents: &["C", "Python"],
        },
    ],
];

// Layout constants, ported from Nitaku's "Tangled Tree Visualization II".
const NODE_HEIGHT: f64 = 22.0;
const NODE_WIDTH: f64 = 90.0;
const BUNDLE_WIDTH: f64 = 14.0;
const LEVEL_Y_PADDING: f64 = 16.0;
const METRO_D: f64 = 4.0;
const PADDING: f64 = 8.0;

// Screen-space points; background core is `NODE_OUTLINE - 2 * NODE_BORDER` wide.
const NODE_OUTLINE: f32 = 8.0;
const NODE_BORDER: f32 = 2.0;

const LABEL_SIZE: f32 = 11.0;

const PALETTE: &[egui::Color32] = &[
    egui::Color32::from_rgb(31, 119, 180),
    egui::Color32::from_rgb(255, 127, 14),
    egui::Color32::from_rgb(44, 160, 44),
    egui::Color32::from_rgb(214, 39, 40),
    egui::Color32::from_rgb(148, 103, 189),
    egui::Color32::from_rgb(140, 86, 75),
    egui::Color32::from_rgb(227, 119, 194),
    egui::Color32::from_rgb(188, 189, 34),
    egui::Color32::from_rgb(23, 190, 207),
];

/// A node after layout, in screen-like (y-down) coordinates.
struct LaidNode {
    label: &'static str,
    x: f64,
    y: f64,
    /// Extra vertical room reserved for the bundles this node parents.
    height: f64,
    parents: Vec<usize>,
    bundle: Option<usize>,
    /// Bundles this node parents, sorted; list index sets each link's attach offset.
    parent_bundles: Vec<usize>,
}

/// A bundle groups all children that share the exact same set of parents.
struct Bundle {
    parents: Vec<usize>,
    level: usize,
    i_in_level: usize,
    x: f64,
}

/// A routed link as a plot-coordinate polyline (y already flipped), tagged with its bundle.
struct LinkPath {
    points: Vec<[f64; 2]>,
    bundle: usize,
}

pub struct TreeVisualizationExample {
    curve: f64,
    line_width: f32,
    color_bundles: bool,
    show_labels: bool,
}

impl Default for TreeVisualizationExample {
    fn default() -> Self {
        Self {
            curve: 16.0,
            line_width: 2.0,
            color_bundles: true,
            show_labels: true,
        }
    }
}

impl TreeVisualizationExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Corner radius:");
                ui.add(egui::Slider::new(&mut self.curve, 4.0..=28.0).text("curve"));
            });
            ui.vertical(|ui| {
                ui.label("Link width:");
                ui.add(egui::Slider::new(&mut self.line_width, 1.0..=5.0).text("width"));
            });
            ui.vertical(|ui| {
                ui.label("Appearance:");
                ui.checkbox(&mut self.color_bundles, "color bundles");
                ui.checkbox(&mut self.show_labels, "show labels");
            });
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let (nodes, links) = compute_layout(self.curve);

        let bg = ui.visuals().extreme_bg_color;
        let fg = ui.visuals().text_color();

        Plot::new("Tangled Tree Demo")
            .data_aspect(1.0)
            .show_axes(false)
            .show_grid(false)
            .show(ui, |plot_ui| {
                // Background halo under each link so crossings stay readable (metro-map look).
                for link in &links {
                    plot_ui.line(
                        Line::new("", link.points.clone())
                            .color(bg)
                            .width(self.line_width + 2.0),
                    );
                }
                for link in &links {
                    let color = if self.color_bundles {
                        PALETTE[link.bundle % PALETTE.len()]
                    } else {
                        fg
                    };
                    plot_ui.line(Line::new("", link.points.clone()).color(color).width(self.line_width));
                }
                // One-to-many junctions render as a capsule spanning the bundle height, simpler nodes as a hollow dot; circular caps round the otherwise flat-capped plot lines.
                for node in &nodes {
                    let top = -(node.y - node.height / 2.0);
                    let bottom = -(node.y + node.height / 2.0);
                    let ends = vec![[node.x, top], [node.x, bottom]];
                    for (width, color) in [(NODE_OUTLINE, fg), (NODE_OUTLINE - 2.0 * NODE_BORDER, bg)] {
                        if node.height > 0.0 {
                            plot_ui.line(Line::new("", ends.clone()).color(color).width(width));
                        }
                        plot_ui.points(
                            Points::new("", ends.clone())
                                .shape(MarkerShape::Circle)
                                .filled(true)
                                .radius(width / 2.0)
                                .color(color),
                        );
                    }
                    if self.show_labels {
                        plot_ui.text(
                            Text::new(
                                "",
                                PlotPoint::new(node.x + 4.0, top),
                                egui::RichText::new(node.label).size(LABEL_SIZE),
                            )
                            .anchor(egui::Align2::LEFT_BOTTOM)
                            .color(fg),
                        );
                    }
                }
            })
            .response
    }
}

fn compute_layout(curve: f64) -> (Vec<LaidNode>, Vec<LinkPath>) {
    let (mut nodes, levels) = build_nodes();
    let mut bundles = build_bundles(&mut nodes, &levels);
    assign_positions(&mut nodes, &mut bundles, &levels);
    let links = build_links(&nodes, &bundles, curve);
    (nodes, links)
}

/// Returns the flattened nodes plus the node indices grouped by level.
fn build_nodes() -> (Vec<LaidNode>, Vec<Vec<usize>>) {
    let mut index = HashMap::new();
    let mut nodes = Vec::new();
    let mut levels = Vec::with_capacity(LEVELS.len());

    for defs in LEVELS {
        let mut level_nodes = Vec::with_capacity(defs.len());
        for def in *defs {
            index.insert(def.id, nodes.len());
            level_nodes.push(nodes.len());
            nodes.push(LaidNode {
                label: def.id,
                x: 0.0,
                y: 0.0,
                height: 0.0,
                parents: Vec::new(),
                bundle: None,
                parent_bundles: Vec::new(),
            });
        }
        levels.push(level_nodes);
    }

    // Resolve parent ids to indices now that every node is known.
    for (level, defs) in LEVELS.iter().enumerate() {
        for (i, def) in defs.iter().enumerate() {
            let node = levels[level][i];
            nodes[node].parents = def.parents.iter().filter_map(|p| index.get(p).copied()).collect();
        }
    }

    (nodes, levels)
}

/// Groups each level's children into bundles by their shared parent set.
fn build_bundles(nodes: &mut [LaidNode], levels: &[Vec<usize>]) -> Vec<Bundle> {
    let mut bundles: Vec<Bundle> = Vec::new();

    for (level, level_nodes) in levels.iter().enumerate() {
        let mut key_to_bundle: HashMap<String, usize> = HashMap::new();
        for &node in level_nodes {
            if nodes[node].parents.is_empty() {
                continue;
            }
            let mut ids: Vec<&str> = nodes[node].parents.iter().map(|&p| nodes[p].label).collect();
            ids.sort_unstable();
            let key = ids.join("-X-");

            let bundle = *key_to_bundle.entry(key).or_insert_with(|| {
                let i_in_level = bundles.iter().filter(|b| b.level == level).count();
                bundles.push(Bundle {
                    parents: nodes[node].parents.clone(),
                    level,
                    i_in_level,
                    x: 0.0,
                });
                bundles.len() - 1
            });
            nodes[node].bundle = Some(bundle);
        }
    }

    for (b, bundle) in bundles.iter().enumerate() {
        for &parent in &bundle.parents {
            nodes[parent].parent_bundles.push(b);
        }
    }
    for node in nodes.iter_mut() {
        // Deeper bundles attach first, so their links fan out predictably.
        node.parent_bundles
            .sort_by(|&a, &b| bundles[b].level.cmp(&bundles[a].level));
        node.height = (node.parent_bundles.len().max(1) - 1) as f64 * METRO_D;
    }

    bundles
}

/// Nodes stack vertically across all levels; each bundle's x sits just right of its parents.
fn assign_positions(nodes: &mut [LaidNode], bundles: &mut [Bundle], levels: &[Vec<usize>]) {
    let mut x_offset = PADDING;
    let mut y_offset = PADDING;
    for (level, level_nodes) in levels.iter().enumerate() {
        let bundle_count = bundles.iter().filter(|b| b.level == level).count();
        x_offset += bundle_count as f64 * BUNDLE_WIDTH;
        y_offset += LEVEL_Y_PADDING;
        for &node in level_nodes {
            nodes[node].x = level as f64 * NODE_WIDTH + x_offset;
            nodes[node].y = NODE_HEIGHT + y_offset + nodes[node].height / 2.0;
            y_offset += NODE_HEIGHT + nodes[node].height;
        }
    }

    for b in 0..bundles.len() {
        let max_parent_x = bundles[b].parents.iter().map(|&p| nodes[p].x).fold(f64::MIN, f64::max);
        let count = bundles.iter().filter(|x| x.level == bundles[b].level).count();
        let slot = count as f64 - 1.0 - bundles[b].i_in_level as f64;
        bundles[b].x = max_parent_x + NODE_WIDTH + slot * BUNDLE_WIDTH;
    }
}

/// One polyline per (child, parent) edge, sharing the bundle's vertical trunk.
fn build_links(nodes: &[LaidNode], bundles: &[Bundle], curve: f64) -> Vec<LinkPath> {
    let mut links = Vec::new();
    for child in nodes {
        let Some(bundle) = child.bundle else { continue };
        for &parent in &child.parents {
            let slot = nodes[parent]
                .parent_bundles
                .iter()
                .position(|&b| b == bundle)
                .unwrap_or(0);
            let count = nodes[parent].parent_bundles.len();
            let yt = nodes[parent].y + slot as f64 * METRO_D - count as f64 * METRO_D / 2.0 + METRO_D / 2.0;
            links.push(LinkPath {
                points: route(nodes[parent].x, yt, bundles[bundle].x, child.x, child.y, curve),
                bundle,
            });
        }
    }
    links
}

/// Rounded metro-style polyline for one link; flips y to read top-to-bottom in the plot's y-up space.
fn route(xt: f64, yt: f64, xb: f64, xs: f64, ys: f64, curve: f64) -> Vec<[f64; 2]> {
    // Clamp the radius so the two corners never overlap on short links.
    let half = (ys - yt) / 2.0;
    let mut c = if curve > half { half } else { curve };
    if c < 1.0 {
        c = 1.0;
    }

    let mut p = vec![[xt, yt], [xb - c, yt]];
    push_arc(&mut p, xb - c, yt + c, c, -FRAC_PI_2, 0.0);
    p.push([xb, ys - c]);
    push_arc(&mut p, xb + c, ys - c, c, PI, FRAC_PI_2);
    p.push([xs, ys]);

    p.iter().map(|&[x, y]| [x, -y]).collect()
}

/// Sample a quarter-circle arc from angle `a0` to `a1` into `out`.
fn push_arc(out: &mut Vec<[f64; 2]>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64) {
    const SEGMENTS: usize = 10;
    for k in 0..=SEGMENTS {
        let t = k as f64 / SEGMENTS as f64;
        let a = a0 + (a1 - a0) * t;
        out.push([cx + r * a.cos(), cy + r * a.sin()]);
    }
}
