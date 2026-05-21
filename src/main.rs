mod app;
mod exercises;
mod input;
mod theory;
mod ui;

use std::sync::Arc;
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
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            app::setup_visuals(&cc.egui_ctx);
            Ok(Box::new(PianoApp::new()))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let music_fonts = [
        "/System/Library/Fonts/Apple Symbols.ttf",
    ];

    for path in &music_fonts {
        if let Ok(data) = std::fs::read(path) {
            fonts
                .font_data
                .insert("music".into(), Arc::new(egui::FontData::from_owned(data)));
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .push("music".into());
            break;
        }
    }

    ctx.set_fonts(fonts);
}
