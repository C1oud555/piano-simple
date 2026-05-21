use std::collections::HashMap;
use egui::Key;
use crate::input::NoteEvent;

pub struct QwertyInput {
    mapping: HashMap<Key, u8>,
}

impl QwertyInput {
    pub fn new() -> Self {
        let mut mapping = HashMap::new();

        mapping.insert(Key::Z, 48);  // C3
        mapping.insert(Key::S, 49);  // C#3
        mapping.insert(Key::X, 50);  // D3
        mapping.insert(Key::D, 51);  // D#3
        mapping.insert(Key::C, 52);  // E3
        mapping.insert(Key::V, 53);  // F3
        mapping.insert(Key::G, 54);  // F#3
        mapping.insert(Key::B, 55);  // G3
        mapping.insert(Key::H, 56);  // G#3
        mapping.insert(Key::N, 57);  // A3
        mapping.insert(Key::J, 58);  // A#3
        mapping.insert(Key::M, 59);  // B3

        mapping.insert(Key::Q, 60);  // C4
        mapping.insert(Key::Num2, 61); // C#4
        mapping.insert(Key::W, 62);  // D4
        mapping.insert(Key::Num3, 63); // D#4
        mapping.insert(Key::E, 64);  // E4
        mapping.insert(Key::R, 65);  // F4
        mapping.insert(Key::Num5, 66); // F#4
        mapping.insert(Key::T, 67);  // G4
        mapping.insert(Key::Num6, 68); // G#4
        mapping.insert(Key::Y, 69);  // A4
        mapping.insert(Key::Num7, 70); // A#4
        mapping.insert(Key::U, 71);  // B4

        mapping.insert(Key::I, 72);  // C5
        mapping.insert(Key::Num9, 73); // C#5
        mapping.insert(Key::O, 74);  // D5
        mapping.insert(Key::Num0, 75); // D#5
        mapping.insert(Key::P, 76);  // E5

        Self { mapping }
    }

    pub fn poll_event(&mut self, ctx: &egui::Context) -> Option<NoteEvent> {
        ctx.input(|i| {
            for (key, &midi_note) in &self.mapping {
                if i.key_pressed(*key) {
                    return Some(NoteEvent { midi_note, velocity: 100 });
                }
            }
            None
        })
    }
}
