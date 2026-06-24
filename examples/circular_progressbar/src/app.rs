use eframe::egui;
use eframe::egui::Color32;
use eframe::egui::Response;
use egui_plot::Line;
use egui_plot::MarkerShape;
use egui_plot::Plot;
use egui_plot::PlotBounds;
use egui_plot::PlotPoint;
use egui_plot::Points;
use egui_plot::Text;

const RADIUS: f64 = 1.0;
/// Arc geometry: the gauge sweeps clockwise from the lower-left (`225°`) to the
/// lower-right (`-45°`), leaving a 90° gap at the bottom.
const START_DEG: f64 = 225.0;
const SWEEP_DEG: f64 = 270.0;
/// Ring thickness in ui points; the end-cap circles use half of it as radius.
const STROKE: f32 = 26.0;

const TRACK_COLOR: Color32 = Color32::from_rgb(64, 64, 68);
const PROGRESS_COLOR: Color32 = Color32::from_rgb(138, 201, 124);

pub struct CircularProgressbarExample {
    /// Value the progress animates toward.
    target: f64,
    /// Current animated value, rising from 0 to `target`.
    current: f64,
    /// Percentage added per frame while animating.
    speed: f64,
}

impl Default for CircularProgressbarExample {
    fn default() -> Self {
        Self {
            target: 89.0,
            current: 0.0,
            speed: 4.0,
        }
    }
}

/// Points sampled along the arc from `from_deg` to `to_deg`.
fn arc(from_deg: f64, to_deg: f64) -> Vec<[f64; 2]> {
    const STEPS: usize = 128;
    (0..=STEPS)
        .map(|i| {
            let deg = from_deg + (to_deg - from_deg) * (i as f64 / STEPS as f64);
            let r = deg.to_radians();
            [RADIUS * r.cos(), RADIUS * r.sin()]
        })
        .collect()
}

fn point_at(deg: f64) -> [f64; 2] {
    let r = deg.to_radians();
    [RADIUS * r.cos(), RADIUS * r.sin()]
}

/// A filled circle marker of the given `radius` (ui points). Used both for the
/// rounded arc end-caps (radius = half the ring thickness) and the info badge.
fn disc(pos: [f64; 2], radius: f32, color: Color32) -> Points<'static> {
    Points::new("", vec![pos])
        .shape(MarkerShape::Circle)
        .radius(radius)
        .filled(true)
        .color(color)
}

impl CircularProgressbarExample {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Target:");
            ui.add(egui::Slider::new(&mut self.target, 0.0..=100.0).suffix(" %"));
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut self.speed, 1.0..=15.0));
            if ui.button("Restart").clicked() {
                self.current = 0.0;
            }
        })
        .response
    }

    pub fn show_plot(&mut self, ui: &mut egui::Ui) -> Response {
        // Animate the displayed value from 0 up to the target.
        if self.current < self.target {
            self.current = (self.current + self.speed).min(self.target);
            ui.ctx().request_repaint();
        } else if self.current > self.target {
            self.current = self.target;
        }

        let fraction = (self.current / 100.0).clamp(0.0, 1.0);
        let progress_end = START_DEG - fraction * SWEEP_DEG;

        Plot::new("circular_progressbar")
            .data_aspect(1.0)
            .show_axes(false)
            .show_grid(false)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([-1.35, -1.4], [1.35, 1.3]));

                // Track: the full gray arc with rounded ends.
                let cap = STROKE / 2.0;
                plot_ui.line(
                    Line::new("", arc(START_DEG, START_DEG - SWEEP_DEG))
                        .width(STROKE)
                        .color(TRACK_COLOR),
                );
                plot_ui.points(disc(point_at(START_DEG), cap, TRACK_COLOR));
                plot_ui.points(disc(point_at(START_DEG - SWEEP_DEG), cap, TRACK_COLOR));

                // Progress: the green arc drawn on top, from the start up to the
                // current fraction.
                if fraction > 0.0 {
                    plot_ui.line(
                        Line::new("", arc(START_DEG, progress_end))
                            .width(STROKE)
                            .color(PROGRESS_COLOR),
                    );
                    plot_ui.points(disc(point_at(START_DEG), cap, PROGRESS_COLOR));
                    plot_ui.points(disc(point_at(progress_end), cap, PROGRESS_COLOR));
                }

                // Center percentage.
                plot_ui.add(
                    Text::new(
                        "",
                        PlotPoint::new(0.0, 0.12),
                        egui::RichText::new(format!("{:.0}%", self.current))
                            .size(52.0)
                            .strong()
                            .color(Color32::from_rgb(236, 236, 236)),
                    )
                    .anchor(egui::Align2::CENTER_CENTER),
                );

                // Info badge: white ring, dark core, an "i" glyph.
                plot_ui.points(disc([0.0, -0.5], 19.0, Color32::from_rgb(238, 238, 238)));
                plot_ui.points(disc([0.0, -0.5], 15.0, Color32::from_rgb(30, 30, 32)));
                plot_ui.add(
                    Text::new(
                        "",
                        PlotPoint::new(0.0, -0.5),
                        egui::RichText::new("i")
                            .size(17.0)
                            .strong()
                            .color(Color32::from_rgb(238, 238, 238)),
                    )
                    .anchor(egui::Align2::CENTER_CENTER),
                );

                // Caption below the gauge.
                plot_ui.add(
                    Text::new(
                        "",
                        PlotPoint::new(0.0, -1.12),
                        egui::RichText::new("Carbon-free")
                            .size(24.0)
                            .color(Color32::from_rgb(150, 150, 154)),
                    )
                    .anchor(egui::Align2::CENTER_CENTER),
                );
            })
            .response
    }
}
