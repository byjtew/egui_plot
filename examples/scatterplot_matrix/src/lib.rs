#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::ScatterplotMatrixExample;

impl PlotExample for ScatterplotMatrixExample {
    fn name(&self) -> &'static str {
        "scatterplot_matrix"
    }

    fn title(&self) -> &'static str {
        "Scatterplot Matrix (SPLOM)"
    }

    fn description(&self) -> &'static str {
        "A scatterplot matrix (SPLOM) showing every pairwise combination of several numeric variables. Each off-diagonal cell plots one variable against another, while the diagonal labels the variable. Points are colored by category, making clusters and correlations easy to spot. Ported from the classic D3 SPLOM example using a grid of egui_plot `Plot` widgets."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["scatterplot_matrix", "splom", "scatter", "points"]
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
