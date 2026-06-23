use std::f64::consts::PI;
use std::ops::RangeInclusive;

use eframe::egui;
use eframe::egui::Color32;
use eframe::egui::Response;
use eframe::egui::vec2;
use egui_plot::AxisHints;
use egui_plot::GridInput;
use egui_plot::GridMark;
use egui_plot::Heatmap;
use egui_plot::Plot;

// Reproduces Mike Bostock's "Electricity Usage, 2019" (https://observablehq.com/@mbostock/electric-usage-2019)
// as a synthetic calendar heatmap: x = hour of day, y = day of year, diverging color = net power kW (red = grid draw, blue = solar export).

const HOURS_PER_DAY: usize = 24;
const DAYS: usize = 365;

/// Day-of-year (0-based) on which each month starts, for a non-leap year.
const MONTH_STARTS: [(usize, &str); 12] = [
    (0, "Jan"),
    (31, "Feb"),
    (59, "Mar"),
    (90, "Apr"),
    (120, "May"),
    (151, "Jun"),
    (181, "Jul"),
    (212, "Aug"),
    (243, "Sep"),
    (273, "Oct"),
    (304, "Nov"),
    (334, "Dec"),
];

/// Diverging blue→white→red scale: low (solar export) blue, high (grid draw) red.
const DIVERGING_PALETTE: [Color32; 11] = [
    Color32::from_rgb(5, 48, 97),
    Color32::from_rgb(33, 102, 172),
    Color32::from_rgb(67, 147, 195),
    Color32::from_rgb(146, 197, 222),
    Color32::from_rgb(209, 229, 240),
    Color32::from_rgb(247, 247, 247),
    Color32::from_rgb(253, 219, 199),
    Color32::from_rgb(244, 165, 130),
    Color32::from_rgb(214, 96, 77),
    Color32::from_rgb(178, 24, 43),
    Color32::from_rgb(103, 0, 31),
];

fn month_and_day(day_of_year: usize) -> (&'static str, usize) {
    let (start, name) = MONTH_STARTS
        .iter()
        .rev()
        .find(|(start_day, _)| *start_day <= day_of_year)
        .copied()
        .unwrap_or((0, "Jan"));
    (name, day_of_year - start + 1)
}

/// Phase advanced per frame. One full 0→peak→0 sweep spans 20 frames, matching
/// the gif capture length so the exported gif loops seamlessly.
const SWEEP_STEP: f64 = PI / 10.0;

pub struct ElectricityGridExample {
    /// Peak solar capacity (kW) the animation sweeps up to; set by the slider.
    solar_capacity: f64,
    /// Animation phase driving the capacity sweep.
    phase: f64,
}

impl Default for ElectricityGridExample {
    fn default() -> Self {
        Self {
            solar_capacity: 6.0,
            phase: 0.0,
        }
    }
}

/// Gaussian bell curve centered at `mu`, width `sigma`, peaking at 1.0.
fn bell(x: f64, mu: f64, sigma: f64) -> f64 {
    (-(((x - mu) / sigma).powi(2))).exp()
}

impl ElectricityGridExample {
    /// Synthetic net power in kW; positive = grid draw, negative = solar export.
    fn net_usage(solar_capacity: f64, day: usize, hour: usize) -> f64 {
        let h = hour as f64;

        // +1 at mid-summer (~late July, day 201), -1 at mid-winter.
        let season = (2.0 * PI * (day as f64 - 201.0) / DAYS as f64).cos();
        let summer = season.max(0.0);
        let winter = (-season).max(0.0);

        let consumption = 0.4
            + 0.9 * bell(h, 7.5, 1.8)
            + 1.4 * bell(h, 19.5, 2.5)
            + 0.8 * summer * bell(h, 15.0, 3.0)
            + 0.5 * winter * (bell(h, 7.0, 2.0) + bell(h, 20.0, 3.0));

        let daylight = (PI * (h - 6.0) / 12.0).sin().max(0.0).powf(1.3);
        let sun_season = 0.35 + 0.65 * (0.5 + 0.5 * season);
        let solar = solar_capacity * sun_season * daylight;

        consumption - solar
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Peak solar capacity:");
            ui.add(
                egui::Slider::new(&mut self.solar_capacity, 0.0..=8.0)
                    .text("kW")
                    .step_by(0.5),
            );
        })
        .response
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        self.phase += SWEEP_STEP;
        ui.ctx().request_repaint();

        // Sweep the effective capacity over 0..=peak so the midday band breathes.
        let solar = self.solar_capacity * 0.5 * (1.0 - self.phase.cos());

        // Row 0 renders at the bottom, so flip day `d` to row `DAYS - 1 - d` to put January on top.
        let mut values = Vec::with_capacity(DAYS * HOURS_PER_DAY);
        let mut max_abs = 0.0_f64;
        for row in 0..DAYS {
            let day = DAYS - 1 - row;
            for hour in 0..HOURS_PER_DAY {
                let v = Self::net_usage(solar, day, hour);
                max_abs = max_abs.max(v.abs());
                values.push(v);
            }
        }
        // Symmetric range so zero maps to the middle (white) of the diverging palette.
        let max_abs = max_abs.max(0.1);

        let heatmap = Heatmap::new(values, HOURS_PER_DAY)
            .palette(&DIVERGING_PALETTE)
            .range(-max_abs, max_abs)
            .show_labels(false)
            .name("Net power (kW)");

        let hour_spacer = |input: GridInput| -> Vec<GridMark> {
            let (min, max) = input.bounds;
            (min.floor() as i64..=max.ceil() as i64)
                .filter(|h| h.rem_euclid(6) == 0)
                .map(|h| GridMark {
                    value: h as f64,
                    step_size: 6.0,
                })
                .collect()
        };
        let hour_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| match mark.value as i64 {
            0 | 24 => "12 AM".to_owned(),
            6 => "6 AM".to_owned(),
            12 => "12 PM".to_owned(),
            18 => "6 PM".to_owned(),
            _ => String::new(),
        };

        let month_spacer = |_input: GridInput| -> Vec<GridMark> {
            MONTH_STARTS
                .iter()
                .map(|(start_day, _)| GridMark {
                    value: (DAYS - 1 - start_day) as f64,
                    step_size: 30.0,
                })
                .collect()
        };
        let month_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let day = (DAYS as f64 - 1.0 - mark.value).round();
            if (0.0..DAYS as f64).contains(&day) {
                month_and_day(day as usize).0.to_owned()
            } else {
                String::new()
            }
        };

        Plot::new("Electricity Grid Demo")
            .custom_x_axes(vec![AxisHints::new_x().label("Hour of day").formatter(hour_formatter)])
            .custom_y_axes(vec![AxisHints::new_y().label("Month").formatter(month_formatter)])
            .x_grid_spacer(hour_spacer)
            .y_grid_spacer(month_spacer)
            .set_margin_fraction(vec2(0.0, 0.0))
            .show_grid(false)
            .label_formatter(|_name, point| {
                let hour = (point.x.floor() as i64).clamp(0, 23);
                let day = (DAYS as f64 - 1.0 - point.y.floor()).round();
                if !(0.0..DAYS as f64).contains(&day) {
                    return String::new();
                }
                let (month, dom) = month_and_day(day as usize);
                format!("{month} {dom}\n{hour:02}:00")
            })
            .show(ui, |plot_ui| {
                plot_ui.heatmap(heatmap);
            })
            .response
    }
}
