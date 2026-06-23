use eframe::egui;
use eframe::egui::Response;
use egui_plot::BoxElem;
use egui_plot::BoxPlot;
use egui_plot::BoxSpread;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

const BULLISH: egui::Color32 = egui::Color32::from_rgb(38, 166, 91);
const BEARISH: egui::Color32 = egui::Color32::from_rgb(217, 83, 79);

/// One OHLC period.
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

pub struct CandlestickChartExample {
    num_candles: usize,
    ma_window: usize,
    show_ma: bool,
}

impl Default for CandlestickChartExample {
    fn default() -> Self {
        Self {
            num_candles: 40,
            ma_window: 5,
            show_ma: true,
        }
    }
}

/// Deterministic pseudo-random value in `-1..=1`, so the synthetic series (and
/// the screenshot) are reproducible.
fn hash_noise(x: f64) -> f64 {
    let v = (x * 12.9898).sin() * 43758.5453;
    2.0 * (v - v.floor()) - 1.0
}

/// Synthetic OHLC series: a random walk where each open is the previous close.
fn candles(n: usize) -> Vec<Candle> {
    let mut out = Vec::with_capacity(n);
    let mut price = 100.0;
    for i in 0..n {
        let i = i as f64;
        let open = price;
        let close = (open + 3.0 * hash_noise(i * 1.7)).max(1.0);
        let high = open.max(close) + 2.0 * hash_noise(i * 3.1 + 1.0).abs();
        let low = (open.min(close) - 2.0 * hash_noise(i * 5.3 + 2.0).abs()).max(0.5);
        out.push(Candle { open, high, low, close });
        price = close;
    }
    out
}

impl CandlestickChartExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Candles:");
                ui.add(egui::Slider::new(&mut self.num_candles, 5..=1000).text("count"));
            });
            ui.vertical(|ui| {
                ui.label("Moving average:");
                ui.checkbox(&mut self.show_ma, "show overlay");
                ui.add(egui::Slider::new(&mut self.ma_window, 2..=20).text("window"));
            });
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        let data = candles(self.num_candles);

        let elem = |i: usize, c: Candle| {
            // Body spans open..close; median sits on the body's lower edge so it
            // reads as a candlestick rather than a box-and-whisker.
            let (lo_body, hi_body) = (c.open.min(c.close), c.open.max(c.close));
            BoxElem::new(i as f64, BoxSpread::new(c.low, lo_body, lo_body, hi_body, c.high))
                .box_width(0.6)
                .whisker_width(0.0)
        };

        let bullish: Vec<BoxElem> = data
            .iter()
            .enumerate()
            .filter(|(_, c)| c.close >= c.open)
            .map(|(i, &c)| elem(i, c))
            .collect();
        let bearish: Vec<BoxElem> = data
            .iter()
            .enumerate()
            .filter(|(_, c)| c.close < c.open)
            .map(|(i, &c)| elem(i, c))
            .collect();

        let bull_plot = BoxPlot::new("Bullish", bullish).color(BULLISH);
        let bear_plot = BoxPlot::new("Bearish", bearish).color(BEARISH);

        Plot::new("Candlestick Chart Demo")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.box_plot(bull_plot);
                plot_ui.box_plot(bear_plot);

                if self.show_ma && data.len() >= self.ma_window {
                    let window = self.ma_window;
                    let ma: PlotPoints<'_> = data
                        .windows(window)
                        .enumerate()
                        .map(|(i, w)| {
                            let mean = w.iter().map(|c| c.close).sum::<f64>() / window as f64;
                            [(i + window - 1) as f64, mean]
                        })
                        .collect();
                    plot_ui.line(Line::new(format!("MA({window})"), ma).color(egui::Color32::from_rgb(240, 200, 80)));
                }
            })
            .response
    }
}
