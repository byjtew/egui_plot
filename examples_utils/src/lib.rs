use eframe::egui;

/// Trait for examples that can be displayed in the demo gallery.
pub trait PlotExample {
    /// The name of the example. Should match directory name.
    fn name(&self) -> &'static str;

    /// The title of the example.
    fn title(&self) -> &'static str;

    /// The description of the example.
    fn description(&self) -> &'static str;

    /// The tags of the example.
    fn tags(&self) -> &'static [&'static str];

    /// The thumbnail image of the example.
    /// Should be 192x192 pixels. It is automatically generated from the
    /// screenshot of the example.
    fn thumbnail_bytes(&self) -> &'static [u8];

    /// The code of the example.
    fn code_bytes(&self) -> &'static [u8];

    /// The UI of the example.
    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response;

    /// The controls for the example.
    fn show_controls(&mut self, ui: &mut egui::Ui) -> egui::Response;

    /// Whether the example animates over time. When `true`, the screenshot test
    /// also captures a short looping `animation.gif` (only when regenerating
    /// snapshots, i.e. with `UPDATE_SNAPSHOTS` set).
    fn animated(&self) -> bool {
        false
    }
}

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub mod internal {
    use std::path::PathBuf;

    use egui_kittest::Harness;
    use egui_kittest::SnapshotOptions;

    pub fn run_screenshot_test<State>(
        builder: impl Fn(&mut eframe::CreationContext<'_>) -> State,
        manifest_dir: &str,
        animated: bool,
    ) where
        State: eframe::App,
    {
        let output_path = PathBuf::from(manifest_dir);
        let options = SnapshotOptions::new()
            .threshold(2.0)
            .failed_pixel_count_threshold(5)
            .output_path(output_path.clone());

        // Generate main screenshot
        let mut harness = Harness::builder()
            .with_size(egui::Vec2::new(800.0, 800.0))
            .build_eframe(&builder);
        // `run_ok` (not `run`) so a continuously-repainting example settles to a
        // frame instead of panicking on the step limit.
        harness.run_ok();
        harness.snapshot_options("screenshot", &options);

        // Generate thumbnail
        let mut thumb_harness = Harness::builder()
            .with_size(egui::Vec2::new(192.0, 192.0))
            .build_eframe(&builder);
        thumb_harness.run_ok();
        let _ = thumb_harness.try_snapshot_options("screenshot_thumb", &options);

        // Capturing every frame is wasted work on a normal test run, so the gif
        // is only (re)generated alongside the other snapshots.
        if animated && std::env::var("UPDATE_SNAPSHOTS").is_ok() {
            capture_gif(&builder, &output_path.join("animation.gif"));
        }
    }

    /// Step a fresh harness frame-by-frame into a looping ~1s gif. Each step
    /// advances the app's own animation clock, so the gif reproduces the live
    /// motion deterministically.
    fn capture_gif<State>(builder: impl Fn(&mut eframe::CreationContext<'_>) -> State, path: &std::path::Path)
    where
        State: eframe::App,
    {
        const FRAMES: usize = 20;
        const FPS: u32 = 20;
        const SIZE: f32 = 400.0;

        let mut harness = Harness::builder().with_size(egui::Vec2::splat(SIZE)).build_eframe(&builder);
        let mut frames = Vec::with_capacity(FRAMES);
        for _ in 0..FRAMES {
            harness.step();
            if let Ok(image) = harness.render() {
                frames.push(image);
            }
        }

        if let Ok(file) = std::fs::File::create(path) {
            let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(std::io::BufWriter::new(file), 10);
            let _ = encoder.set_repeat(image::codecs::gif::Repeat::Infinite);
            for image in frames {
                let delay = image::Delay::from_numer_denom_ms(1000, FPS);
                let _ = encoder.encode_frame(image::Frame::from_parts(image, 0, 0, delay));
            }
        }
    }
}

/// Macro to generate a simple native `main` function for an `eframe` example
/// and a corresponding screenshot test. Intended to be used for [`PlotExample`]
/// implementations.
///
/// # Example
///
/// ```no_run,ignore
/// use examples_utils::make_main;
/// use my_example::MyExample;
///
/// make_main!(MyExample);
/// ```
#[macro_export]
macro_rules! make_main {
    ($inner:ident) => {
        use eframe::egui;

        // Generate wrapper struct
        #[derive(Default)]
        pub struct AppWrapper {
            pub inner: $inner,
            pub plot_only: bool,
        }

        impl eframe::App for AppWrapper {
            fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if self.plot_only {
                        self.inner.show_plot(ui);
                    } else {
                        ui.vertical(|ui| {
                            self.inner.show_controls(ui);
                            ui.separator();
                            self.inner.show_plot(ui);
                        });
                    }
                });
            }
        }

        /// Native entry-point for the example.
        fn main() -> eframe::Result {
            use $crate::PlotExample as _;

            env_logger::init();

            // Derive the application title from the `PlotExample` implementation.
            let app_name: &'static str = <$inner as $crate::PlotExample>::title(&<$inner as Default>::default());

            let options = eframe::NativeOptions::default();
            eframe::run_native(
                app_name,
                options,
                Box::new(|_cc| Ok(Box::new(AppWrapper::default()))),
            )
        }

        /// Screenshot tests for the example.
        ///
        /// This uses `egui_kittest` under the hood and is only compiled for
        /// non-WASM targets.
        #[cfg(all(test, not(target_arch = "wasm32")))]
        mod screenshot_tests {
            use super::AppWrapper;
            use ::examples_utils::PlotExample as _;

            #[allow(non_snake_case)]
            #[test]
            fn $inner() {
                let animated = AppWrapper::default().inner.animated();
                ::examples_utils::internal::run_screenshot_test(
                    |_cc| AppWrapper {
                        plot_only: true,
                        ..Default::default()
                    },
                    env!("CARGO_MANIFEST_DIR"),
                    animated,
                );
            }
        }
    };
}
