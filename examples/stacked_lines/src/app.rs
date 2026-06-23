use std::f64::consts::PI;

use eframe::egui;
use eframe::egui::Response;
use egui_plot::FilledArea;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

/// Each series is stacked on the cumulative total of the series below it.
struct Series {
    name: &'static str,
    color: egui::Color32,
    f: fn(f64) -> f64,
}

const SERIES: &[Series] = &[
    Series {
        name: "1 + sin(x)",
        color: egui::Color32::from_rgb(200, 100, 100),
        f: |x| 1.0 + x.sin(),
    },
    Series {
        name: "1 + cos(x / 2)",
        color: egui::Color32::from_rgb(100, 170, 100),
        f: |x| 1.0 + (x / 2.0).cos(),
    },
    Series {
        name: "1 + 0.5 sin(2x)",
        color: egui::Color32::from_rgb(100, 130, 200),
        f: |x| 1.0 + 0.5 * (2.0 * x).sin(),
    },
];

pub struct StackedLinesExample {
    num_points: usize,
    fill_alpha: u8,
    show_boundaries: bool,
}

impl Default for StackedLinesExample {
    fn default() -> Self {
        Self {
            num_points: 100,
            fill_alpha: 80,
            show_boundaries: true,
        }
    }
}

impl StackedLinesExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Number of points:");
                ui.add(egui::Slider::new(&mut self.num_points, 10..=500).text("points"));
            });
            ui.vertical(|ui| {
                ui.label("Fill opacity:");
                ui.add(egui::Slider::new(&mut self.fill_alpha, 0..=255).text("alpha"));
            });
            ui.vertical(|ui| {
                ui.label("Boundaries:");
                ui.checkbox(&mut self.show_boundaries, "show top lines");
            });
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let xs: Vec<f64> = (0..self.num_points)
            .map(|i| i as f64 * 4.0 * PI / self.num_points as f64)
            .collect();

        Plot::new("Stacked Lines Demo")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                let mut cumulative = vec![0.0; xs.len()];

                for series in SERIES {
                    // Lower boundary is the previous stack top; upper adds this series on top.
                    let ys_min = cumulative.clone();
                    let ys_max: Vec<f64> = xs
                        .iter()
                        .zip(&cumulative)
                        .map(|(&x, &base)| base + (series.f)(x))
                        .collect();

                    let fill_color = egui::Color32::from_rgba_unmultiplied(
                        series.color.r(),
                        series.color.g(),
                        series.color.b(),
                        self.fill_alpha,
                    );

                    plot_ui.add(FilledArea::new(series.name, &xs, &ys_min, &ys_max).fill_color(fill_color));

                    if self.show_boundaries {
                        plot_ui.line(
                            Line::new(
                                series.name,
                                xs.iter()
                                    .zip(&ys_max)
                                    .map(|(&x, &y)| [x, y])
                                    .collect::<PlotPoints<'_>>(),
                            )
                            .color(series.color),
                        );
                    }

                    cumulative = ys_max;
                }
            })
            .response
    }
}
