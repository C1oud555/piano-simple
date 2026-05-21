use crate::exercises::{sight_read::SightReadExercise, Exercise};
use crate::input::InputManager;

pub struct PianoApp {
    input_manager: InputManager,
    exercise: Box<dyn Exercise>,
}

impl PianoApp {
    pub fn new() -> Self {
        Self {
            input_manager: InputManager::new(),
            exercise: Box::new(SightReadExercise::new()),
        }
    }
}

impl eframe::App for PianoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(event) = self.input_manager.poll_event(ctx) {
            self.exercise.handle_input(&event);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.exercise.render(ui);
        });
    }
}

pub fn setup_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    visuals.panel_fill = egui::Color32::from_rgb(248, 244, 236);
    visuals.window_fill = egui::Color32::from_rgb(255, 252, 245);
    ctx.set_visuals(visuals);
}
