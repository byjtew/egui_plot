use std::ops::RangeInclusive;

use eframe::egui;
use eframe::egui::Response;
use egui_plot::AxisHints;
use egui_plot::Bar;
use egui_plot::BarChart;
use egui_plot::GridInput;
use egui_plot::GridMark;
use egui_plot::Legend;
use egui_plot::Plot;
use egui_plot::PlotPoint;

struct Segment {
    name: &'static str,
    color: egui::Color32,
    /// Raw population in millions, per category.
    values: [f64; 5],
}

const CATEGORIES: [&str; 5] = ["California", "Texas", "Florida", "New York", "Illinois"];

/// Values are normalized per category to 100% (D3 stacked-normalized style).
const SEGMENTS: &[Segment] = &[
    Segment {
        name: "<20",
        color: egui::Color32::from_rgb(99, 138, 198),
        values: [10.0, 8.0, 4.5, 4.5, 3.2],
    },
    Segment {
        name: "20-39",
        color: egui::Color32::from_rgb(96, 173, 110),
        values: [11.5, 8.5, 5.5, 5.5, 3.6],
    },
    Segment {
        name: "40-59",
        color: egui::Color32::from_rgb(214, 180, 92),
        values: [10.0, 7.0, 5.5, 5.0, 3.4],
    },
    Segment {
        name: "≥60",
        color: egui::Color32::from_rgb(214, 110, 92),
        values: [7.5, 5.0, 6.0, 4.5, 2.6],
    },
];

pub struct StackedHorizontalBarExample {
    bar_width: f64,
}

impl Default for StackedHorizontalBarExample {
    fn default() -> Self {
        Self { bar_width: 0.8 }
    }
}

impl StackedHorizontalBarExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Bar width:");
            ui.add(egui::Slider::new(&mut self.bar_width, 0.1..=1.0).step_by(0.05));
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let totals: Vec<f64> = (0..CATEGORIES.len())
            .map(|i| SEGMENTS.iter().map(|s| s.values[i]).sum())
            .collect();

        let mut charts: Vec<BarChart> = Vec::with_capacity(SEGMENTS.len());

        for segment in SEGMENTS {
            let bars = CATEGORIES
                .iter()
                .enumerate()
                .zip(segment.values)
                .map(|((index, &category), value)| {
                    // First arg is the category position; second is bar length (normalized value).
                    let fraction = value / totals[index];
                    Bar::new(index as f64 + 0.5, fraction).name(category)
                })
                .collect();

            let mut chart = BarChart::new(segment.name, bars)
                .width(self.bar_width)
                .color(segment.color)
                .name(segment.name)
                .horizontal();

            let previous: Vec<&BarChart> = charts.iter().collect();
            if !previous.is_empty() {
                chart = chart.stack_on(&previous);
            }

            charts.push(chart);
        }

        let percent_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| format!("{:.0}%", mark.value * 100.0);

        let category_grid = |_input: GridInput| -> Vec<GridMark> {
            (0..CATEGORIES.len())
                .map(|i| GridMark {
                    value: i as f64 + 0.5,
                    step_size: 1.0,
                })
                .collect()
        };

        let category_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let index = (mark.value - 0.5).round() as usize;
            CATEGORIES.get(index).copied().unwrap_or("").to_owned()
        };

        let label_formatter = |name: &str, point: &PlotPoint| {
            if name.is_empty() {
                String::new()
            } else {
                format!("{name}\n{:.1}%", point.x * 100.0)
            }
        };

        Plot::new("Stacked Normalized Horizontal Bar Chart Demo")
            .legend(Legend::default())
            .custom_x_axes(vec![AxisHints::new_x().label("Share").formatter(percent_formatter)])
            .custom_y_axes(vec![AxisHints::new_y().formatter(category_formatter)])
            .y_grid_spacer(category_grid)
            .label_formatter(label_formatter)
            .show(ui, |plot_ui| {
                for chart in charts {
                    plot_ui.bar_chart(chart);
                }
            })
            .response
    }
}
