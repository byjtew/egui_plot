#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::StackedHorizontalBarExample;

impl PlotExample for StackedHorizontalBarExample {
    fn name(&self) -> &'static str {
        "stacked_horizontal_bar"
    }

    fn title(&self) -> &'static str {
        "Stacked Normalized Horizontal Bar Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to create a stacked normalized (100%) horizontal bar chart, inspired by D3's stacked-normalized-horizontal-bar. Each category is a full-width horizontal bar whose stacked segments show each part's share of the total, with a percentage axis."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["bar_chart", "stacked_bar", "horizontal", "normalized"]
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
