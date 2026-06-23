#![cfg_attr(doc, doc = include_str!("../README.md"))]

use eframe::egui;
use examples_utils::PlotExample;

mod app;
pub use app::CandlestickChartExample;

impl PlotExample for CandlestickChartExample {
    fn name(&self) -> &'static str {
        "candlestick_chart"
    }

    fn title(&self) -> &'static str {
        "Candlestick Chart Demo"
    }

    fn description(&self) -> &'static str {
        "This example demonstrates a financial OHLC candlestick chart built on top of box plots. Each candle's high/low becomes the whiskers and its open/close becomes the body, colored green when the price rose over the period and red when it fell, with an optional simple moving average overlaid."
    }

    fn tags(&self) -> &'static [&'static str] {
        &["candlestick", "finance", "ohlc", "box_plot"]
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
