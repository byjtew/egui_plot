use std::time::Duration;

use eframe::egui;
use eframe::egui::Response;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotBounds;
use egui_plot::PlotPoints;

const X_MAX: f64 = 100.0;

const N_POINTS: usize = 600;

/// Per-frame scroll distance in x-units; real-world speed is this times the FPS.
const SCROLL_STEP: f64 = 0.7;

pub struct AnimatedSignalExample {
    phase: f64,

    /// Also throttles repaint frequency.
    fps: f32,
}

impl Default for AnimatedSignalExample {
    fn default() -> Self {
        Self { phase: 0.0, fps: 30.0 }
    }
}

/// Deterministic pure function of `x` so the animation and screenshot test stay reproducible.
fn hash_noise(x: f64) -> f64 {
    let v = (x * 12.9898).sin() * 43758.5453;
    2.0 * (v - v.floor()) - 1.0
}

/// Noisy trace centered on 1.0, clamped to `0.8..=1.2`.
fn signal(s: f64) -> f64 {
    let noise = 0.55 * hash_noise(s) + 0.30 * hash_noise(s * 2.7 + 11.0) + 0.15 * hash_noise(s * 6.1 + 5.0);
    let carrier = 0.05 * (s * 0.6).sin();
    (1.0 + 0.18 * noise + carrier).clamp(0.8, 1.2)
}

impl AnimatedSignalExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Animation speed:");
            ui.add(egui::Slider::new(&mut self.fps, 1.0..=120.0).text("FPS"));
        })
        .response
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        self.phase += SCROLL_STEP;
        ui.ctx().request_repaint_after(Duration::from_secs_f32(1.0 / self.fps));

        // A feature at sample coord sits at `x = coord - phase`, so it drifts left over time.
        let phase = self.phase;
        let points: PlotPoints<'_> = (0..N_POINTS)
            .map(|k| {
                let x = k as f64 / (N_POINTS - 1) as f64 * X_MAX;
                [x, signal(x + phase)]
            })
            .collect();

        let line = Line::new("signal", points).color(egui::Color32::from_rgb(100, 230, 150));

        Plot::new("Animated Signal Demo")
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [X_MAX, 2.0]));
                plot_ui.line(line);
            })
            .response
    }
}
