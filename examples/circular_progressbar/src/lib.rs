#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::CircularProgressbarExample;

impl PlotExample for CircularProgressbarExample {
    fn name(&self) -> &'static str {
        "circular_progressbar"
    }

    fn title(&self) -> &'static str {
        "Circular Progressbar Demo"
    }

    fn description(&self) -> &'static str {
        "This example draws a circular (gauge-style) progress bar: a 270° arc with a gap at the bottom, a colored progress arc with rounded caps over a gray track, a percentage in the center, and a label below. The progress animates up from zero, with a Restart button and an animation-speed slider. It shows how to compose arcs from thick polylines plus circular end-caps."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["progressbar", "gauge", "arc", "circular"]
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
