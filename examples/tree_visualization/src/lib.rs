#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::TreeVisualizationExample;

impl PlotExample for TreeVisualizationExample {
    fn name(&self) -> &'static str {
        "tree_visualization"
    }

    fn title(&self) -> &'static str {
        "Tangled Tree Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates how to draw a tangled tree visualization, a node-link diagram for directed acyclic graphs where a node can have several parents. Nodes are laid out level by level and the edges are bundled into metro-map-style links. It is based on Nitaku's 'Tangled Tree Visualization II'."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["tree", "graph", "tangled_tree", "lines"]
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
