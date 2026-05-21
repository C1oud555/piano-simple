use crate::input::NoteEvent;

/// Placeholder for future MIDI input support.
/// To enable: `cargo add midir` and implement the connection logic.
pub struct MidiInput;

#[allow(dead_code)]
impl MidiInput {
    pub fn new() -> Self {
        Self
    }

    pub fn poll_event(&mut self) -> Option<NoteEvent> {
        None
    }
}
