#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::AnimatedSignalExample;

impl PlotExample for AnimatedSignalExample {
    fn name(&self) -> &'static str {
        "animated_signal"
    }

    fn title(&self) -> &'static str {
        "Animated Signal Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates a continuously animated plot: a very noisy electric-style signal that scrolls to the left as new samples arrive on the right. The y-axis is pinned to 0..2 and the signal fluctuates around 1.0, while an FPS slider controls how fast the trace scrolls by throttling the repaint rate."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["animation", "line", "realtime", "signal"]
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
