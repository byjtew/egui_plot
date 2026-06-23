#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::ElectricityGridExample;

impl PlotExample for ElectricityGridExample {
    fn name(&self) -> &'static str {
        "electricity_grid"
    }

    fn title(&self) -> &'static str {
        "Electricity Grid Demo"
    }

    fn description(&self) -> &'static str {
        "This example reproduces Mike Bostock's \"Electricity Usage, 2019\" as a calendar heatmap. Each tile is one hour of the year: the x-axis is the hour of day, the y-axis the day of year, and a diverging color scale shows net power (kW) — red when drawing from the grid, blue when solar panels export back to it. The solar capacity animates, sweeping from zero up to the slider's value so the midday band breathes between grid draw and solar export."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["heatmap", "calendar", "color", "visualization"]
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

    fn animated(&self) -> bool {
        true
    }
}
