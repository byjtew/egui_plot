#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::StackedLinesExample;

impl PlotExample for StackedLinesExample {
    fn name(&self) -> &'static str {
        "stacked_lines"
    }

    fn title(&self) -> &'static str {
        "Stacked Lines Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create a stacked area chart by stacking several series on top of each other. Each series is drawn as a filled band between its cumulative lower and upper boundaries, which is useful for visualizing how parts contribute to a whole over time."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["stacked_lines", "filled_area", "stacked_area"]
    }

    fn thumbnail_bytes(&self) -> &'static [u8] {
        include_bytes!("../screenshot_thumb.png")
    }

    fn code_bytes(&self) -> &'static [u8] {
        include_bytes!("./app.rs")
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        self.show_plot(ui)
    }

    fn show_controls(&mut self, ui: &mut egui::Ui) -> egui::Response {
        self.show_controls(ui)
    }
}
