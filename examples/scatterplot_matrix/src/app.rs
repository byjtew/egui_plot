use eframe::egui;
use eframe::egui::Response;
use egui_plot::Plot;
use egui_plot::PlotBounds;
use egui_plot::PlotPoint;
use egui_plot::Points;
use egui_plot::Text;

const VARIABLES: [&str; 4] = ["Bill length", "Bill depth", "Flipper length", "Body mass"];

const SPECIES: [(&str, egui::Color32); 3] = [
    ("Adelie", egui::Color32::from_rgb(230, 126, 34)),
    ("Gentoo", egui::Color32::from_rgb(46, 134, 193)),
    ("Chinstrap", egui::Color32::from_rgb(155, 89, 182)),
];

struct Penguin {
    /// `[bill_length, bill_depth, flipper_length, body_mass]` in (mm, mm, mm, grams).
    values: [f64; 4],
    species: usize,
}

/// Hand-picked subset of the Palmer Penguins dataset, not the full data.
const PENGUINS: &[Penguin] = &[
    Penguin {
        values: [39.1, 18.7, 181.0, 3750.0],
        species: 0,
    },
    Penguin {
        values: [39.5, 17.4, 186.0, 3800.0],
        species: 0,
    },
    Penguin {
        values: [40.3, 18.0, 195.0, 3250.0],
        species: 0,
    },
    Penguin {
        values: [36.7, 19.3, 193.0, 3450.0],
        species: 0,
    },
    Penguin {
        values: [39.3, 20.6, 190.0, 3650.0],
        species: 0,
    },
    Penguin {
        values: [38.9, 17.8, 181.0, 3625.0],
        species: 0,
    },
    Penguin {
        values: [39.2, 19.6, 195.0, 4675.0],
        species: 0,
    },
    Penguin {
        values: [41.1, 17.6, 182.0, 3200.0],
        species: 0,
    },
    Penguin {
        values: [38.6, 21.2, 191.0, 3800.0],
        species: 0,
    },
    Penguin {
        values: [34.6, 21.1, 198.0, 4400.0],
        species: 0,
    },
    Penguin {
        values: [36.6, 17.8, 185.0, 3700.0],
        species: 0,
    },
    Penguin {
        values: [38.7, 19.0, 195.0, 3450.0],
        species: 0,
    },
    Penguin {
        values: [42.5, 20.7, 197.0, 4500.0],
        species: 0,
    },
    Penguin {
        values: [34.4, 18.4, 184.0, 3325.0],
        species: 0,
    },
    Penguin {
        values: [46.1, 13.2, 211.0, 4500.0],
        species: 1,
    },
    Penguin {
        values: [50.0, 16.3, 230.0, 5700.0],
        species: 1,
    },
    Penguin {
        values: [48.7, 14.1, 210.0, 4450.0],
        species: 1,
    },
    Penguin {
        values: [50.0, 15.2, 218.0, 5700.0],
        species: 1,
    },
    Penguin {
        values: [47.6, 14.5, 215.0, 5400.0],
        species: 1,
    },
    Penguin {
        values: [46.5, 13.5, 210.0, 4550.0],
        species: 1,
    },
    Penguin {
        values: [45.4, 14.6, 211.0, 4800.0],
        species: 1,
    },
    Penguin {
        values: [46.7, 15.3, 219.0, 5200.0],
        species: 1,
    },
    Penguin {
        values: [43.3, 13.4, 209.0, 4400.0],
        species: 1,
    },
    Penguin {
        values: [49.3, 15.7, 217.0, 5850.0],
        species: 1,
    },
    Penguin {
        values: [50.2, 14.3, 218.0, 5700.0],
        species: 1,
    },
    Penguin {
        values: [45.1, 14.5, 215.0, 5000.0],
        species: 1,
    },
    Penguin {
        values: [49.0, 16.1, 216.0, 5550.0],
        species: 1,
    },
    Penguin {
        values: [48.4, 14.6, 213.0, 5850.0],
        species: 1,
    },
    Penguin {
        values: [46.5, 17.9, 192.0, 3500.0],
        species: 2,
    },
    Penguin {
        values: [50.0, 19.5, 196.0, 3900.0],
        species: 2,
    },
    Penguin {
        values: [51.3, 19.2, 193.0, 3650.0],
        species: 2,
    },
    Penguin {
        values: [45.4, 18.7, 188.0, 3525.0],
        species: 2,
    },
    Penguin {
        values: [52.7, 19.8, 197.0, 3725.0],
        species: 2,
    },
    Penguin {
        values: [45.2, 17.8, 198.0, 3950.0],
        species: 2,
    },
    Penguin {
        values: [46.1, 18.2, 178.0, 3250.0],
        species: 2,
    },
    Penguin {
        values: [51.3, 18.2, 197.0, 3750.0],
        species: 2,
    },
    Penguin {
        values: [46.0, 18.9, 195.0, 4150.0],
        species: 2,
    },
    Penguin {
        values: [51.3, 19.9, 198.0, 3700.0],
        species: 2,
    },
    Penguin {
        values: [46.6, 17.8, 193.0, 3800.0],
        species: 2,
    },
    Penguin {
        values: [51.7, 20.3, 194.0, 3775.0],
        species: 2,
    },
    Penguin {
        values: [47.0, 17.3, 185.0, 3700.0],
        species: 2,
    },
    Penguin {
        values: [52.0, 18.1, 201.0, 4050.0],
        species: 2,
    },
];

pub struct ScatterplotMatrixExample {
    /// Side length in points (not pixels).
    cell_size: f32,
    point_radius: f32,
    show_diagonal_labels: bool,
}

impl Default for ScatterplotMatrixExample {
    fn default() -> Self {
        Self {
            cell_size: 130.0,
            point_radius: 2.5,
            show_diagonal_labels: true,
        }
    }
}

impl ScatterplotMatrixExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Cell size:");
                ui.add(egui::Slider::new(&mut self.cell_size, 80.0..=200.0).text("px"));
            });
            ui.vertical(|ui| {
                ui.label("Point radius:");
                ui.add(egui::Slider::new(&mut self.point_radius, 1.0..=6.0).text("radius"));
            });
            ui.vertical(|ui| {
                ui.label("Diagonal:");
                ui.checkbox(&mut self.show_diagonal_labels, "show labels");
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.label("Species:");
                ui.horizontal(|ui| {
                    for (name, color) in SPECIES {
                        ui.colored_label(color, format!("⏺ {name}"));
                    }
                });
            });
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let n = VARIABLES.len();

        egui::ScrollArea::both()
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
                ui.vertical(|ui| {
                    for row in 0..n {
                        ui.horizontal(|ui| {
                            for col in 0..n {
                                self.show_cell(ui, row, col);
                            }
                        });
                    }
                })
                .response
            })
            .inner
    }

    /// Cell `(row, col)` plots variable `col` (x) against variable `row` (y).
    fn show_cell(&self, ui: &mut egui::Ui, row: usize, col: usize) {
        let is_diagonal = row == col;

        // No per-cell axes: axis ticks reserve variable space and break grid alignment.
        let plot = Plot::new(("splom_cell", row, col))
            .width(self.cell_size)
            .height(self.cell_size)
            .show_axes(false)
            .show_grid(true)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false);

        plot.show(ui, |plot_ui| {
            // Pin identical bounds in every cell so scatter regions line up across the matrix.
            plot_ui.set_plot_bounds(PlotBounds::from_min_max([-0.05, -0.05], [1.05, 1.05]));

            if is_diagonal {
                if self.show_diagonal_labels {
                    let text_color = plot_ui.ctx().global_style().visuals.text_color();
                    plot_ui.add(
                        Text::new(
                            "label",
                            PlotPoint::new(0.5, 0.5),
                            egui::RichText::new(VARIABLES[col]).strong(),
                        )
                        .color(text_color),
                    );
                }
                return;
            }

            for (species_idx, (name, color)) in SPECIES.iter().enumerate() {
                let pts: Vec<[f64; 2]> = PENGUINS
                    .iter()
                    .filter(|p| p.species == species_idx)
                    .map(|p| [normalize(col, p.values[col]), normalize(row, p.values[row])])
                    .collect();

                plot_ui.points(
                    Points::new(*name, pts)
                        .color(*color)
                        .radius(self.point_radius)
                        .filled(true),
                );
            }
        });
    }
}

/// Normalize to `0..1` via the column's dataset min/max so cells share a scale.
fn normalize(var: usize, value: f64) -> f64 {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for p in PENGUINS {
        min = min.min(p.values[var]);
        max = max.max(p.values[var]);
    }
    if (max - min).abs() < f64::EPSILON {
        0.5
    } else {
        (value - min) / (max - min)
    }
}
