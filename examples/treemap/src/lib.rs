#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::TreemapExample;

impl PlotExample for TreemapExample {
    fn name(&self) -> &'static str {
        "treemap"
    }

    fn title(&self) -> &'static str {
        "Treemap Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates a treemap, ported from D3's treemap example. A treemap tiles a rectangle with one cell per leaf of a hierarchy, where each cell's area is proportional to the leaf's value. Cells are colored by their top-level category. The layout uses the squarified tiling algorithm for nice aspect ratios."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["treemap", "hierarchy", "polygon"]
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
