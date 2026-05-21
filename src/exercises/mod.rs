pub mod settings;
pub mod sight_read;
pub mod song;

use crate::input::NoteEvent;

#[allow(dead_code)]
pub trait Exercise {
    fn name(&self) -> &str;
    fn handle_input(&mut self, event: &NoteEvent);
    fn render(&mut self, ui: &mut egui::Ui) -> Option<NoteEvent>;
}
