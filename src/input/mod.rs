pub mod midi;
pub mod qwerty;

use crate::theory::note::Note;

#[derive(Debug, Clone)]
pub struct NoteEvent {
    pub midi_note: u8,
    #[allow(dead_code)]
    pub velocity: u8,
}

#[allow(dead_code)]
impl NoteEvent {
    pub fn note(&self) -> Note {
        Note::from_midi(self.midi_note)
    }
}

pub struct InputManager {
    qwerty: qwerty::QwertyInput,
    _midi: Option<midi::MidiInput>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            qwerty: qwerty::QwertyInput::new(),
            _midi: None,
        }
    }

    pub fn poll_event(&mut self, ctx: &egui::Context) -> Option<NoteEvent> {
        self.qwerty.poll_event(ctx).or_else(|| {
            #[cfg(feature = "midi")]
            if let Some(midi) = &mut self._midi {
                return midi.poll_event();
            }
            None
        })
    }
}
