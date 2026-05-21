mod app;
mod exercises;
mod input;
mod theory;
mod ui;

use app::PianoApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([950.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Piano Simple",
        options,
        Box::new(|_cc| Ok(Box::new(PianoApp::new()))),
    )
}
