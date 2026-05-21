use egui::{self, Color32, Rect, Sense, Stroke, StrokeKind, Vec2, pos2};
use crate::theory::note::NoteName;

const WHITE_WIDTH: f32 = 40.0;
const WHITE_HEIGHT: f32 = 160.0;
const BLACK_WIDTH: f32 = 26.0;
const BLACK_HEIGHT: f32 = 100.0;

const WHITE_OFFSETS: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const BLACK_OFFSETS: [u8; 5] = [1, 3, 6, 8, 10];
const BLACK_CENTERS: [f32; 5] = [1.0, 2.0, 4.0, 5.0, 6.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Normal,
    Correct,
    Wrong,
    Highlight,
}

pub struct PianoKeyboard {
    pub start_note: u8,
    pub num_octaves: usize,
    key_states: [KeyState; 128],
}

impl PianoKeyboard {
    pub fn new(start_note: u8, num_octaves: usize) -> Self {
        Self {
            start_note,
            num_octaves,
            key_states: [KeyState::Normal; 128],
        }
    }

    pub fn set_key_state(&mut self, midi: u8, state: KeyState) {
        if let Some(k) = self.key_states.get_mut(midi as usize) {
            *k = state;
        }
    }

    pub fn reset_states(&mut self) {
        self.key_states = [KeyState::Normal; 128];
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Option<u8> {
        let total_width = self.num_octaves as f32 * 7.0 * WHITE_WIDTH;
        let (response, painter) = ui.allocate_painter(
            Vec2::new(total_width, WHITE_HEIGHT),
            Sense::click(),
        );

        let origin = response.rect.min;
        let mut clicked_note = None;

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                for octave in 0..self.num_octaves {
                    let base = self.start_note + (octave * 12) as u8;
                    for (&offset, &cx) in BLACK_OFFSETS.iter().zip(BLACK_CENTERS.iter()) {
                        let midi = base + offset;
                        let kx = origin.x
                            + octave as f32 * 7.0 * WHITE_WIDTH
                            + cx * WHITE_WIDTH
                            - BLACK_WIDTH / 2.0;
                        let rect = Rect::from_min_size(
                            pos2(kx, origin.y),
                            Vec2::new(BLACK_WIDTH, BLACK_HEIGHT),
                        );
                        if rect.contains(pos) {
                            clicked_note = Some(midi);
                        }
                    }
                }

                if clicked_note.is_none() {
                    for octave in 0..self.num_octaves {
                        let base = self.start_note + (octave * 12) as u8;
                        for (wi, &offset) in WHITE_OFFSETS.iter().enumerate() {
                            let midi = base + offset;
                            let kx = origin.x
                                + (octave as f32 * 7.0 + wi as f32) * WHITE_WIDTH;
                            let rect = Rect::from_min_size(
                                pos2(kx, origin.y),
                                Vec2::new(WHITE_WIDTH, WHITE_HEIGHT),
                            );
                            if rect.contains(pos) {
                                clicked_note = Some(midi);
                            }
                        }
                    }
                }
            }
        }

        for octave in 0..self.num_octaves {
            let base = self.start_note + (octave * 12) as u8;
            for (wi, &offset) in WHITE_OFFSETS.iter().enumerate() {
                let midi = base + offset;
                let x = origin.x + (octave as f32 * 7.0 + wi as f32) * WHITE_WIDTH;
                let rect = Rect::from_min_size(
                    pos2(x, origin.y),
                    Vec2::new(WHITE_WIDTH, WHITE_HEIGHT),
                );

                let state = self.key_states[midi as usize];
                let (fill, stroke) = match state {
                    KeyState::Correct => (
                        Color32::from_rgb(144, 238, 144),
                        Stroke::new(2.0, Color32::GREEN),
                    ),
                    KeyState::Wrong => (
                        Color32::from_rgb(255, 160, 160),
                        Stroke::new(2.0, Color32::RED),
                    ),
                    KeyState::Highlight => (
                        Color32::from_rgb(173, 216, 230),
                        Stroke::new(1.0, Color32::GRAY),
                    ),
                    KeyState::Normal => (
                        Color32::WHITE,
                        Stroke::new(1.0, Color32::GRAY),
                    ),
                };

                painter.rect_filled(rect, 2.0, fill);
                painter.rect_stroke(rect, 2.0, stroke, StrokeKind::Inside);
            }
        }

        for octave in 0..self.num_octaves {
            let base = self.start_note + (octave * 12) as u8;
            for (&offset, &cx) in BLACK_OFFSETS.iter().zip(BLACK_CENTERS.iter()) {
                let midi = base + offset;
                let x = origin.x
                    + octave as f32 * 7.0 * WHITE_WIDTH
                    + cx * WHITE_WIDTH
                    - BLACK_WIDTH / 2.0;
                let rect = Rect::from_min_size(
                    pos2(x, origin.y),
                    Vec2::new(BLACK_WIDTH, BLACK_HEIGHT),
                );

                let state = self.key_states[midi as usize];
                let (fill, stroke) = match state {
                    KeyState::Correct => (
                        Color32::from_rgb(80, 180, 80),
                        Stroke::new(2.0, Color32::GREEN),
                    ),
                    KeyState::Wrong => (
                        Color32::from_rgb(200, 80, 80),
                        Stroke::new(2.0, Color32::RED),
                    ),
                    KeyState::Highlight => (
                        Color32::from_rgb(100, 150, 200),
                        Stroke::new(1.0, Color32::DARK_GRAY),
                    ),
                    KeyState::Normal => (
                        Color32::from_rgb(30, 30, 30),
                        Stroke::new(1.0, Color32::DARK_GRAY),
                    ),
                };

                painter.rect_filled(rect, 2.0, fill);
                painter.rect_stroke(rect, 2.0, stroke, StrokeKind::Inside);
            }
        }

        self.draw_labels(&painter, origin);

        clicked_note
    }

    fn draw_labels(&self, painter: &egui::Painter, origin: egui::Pos2) {
        let label_notes = [NoteName::C, NoteName::F];
        for octave in 0..self.num_octaves {
            let base = self.start_note + (octave * 12) as u8;
            for (wi, &offset) in WHITE_OFFSETS.iter().enumerate() {
                let midi = base + offset;
                let name = NoteName::from_semitone(midi);
                if label_notes.contains(&name) {
                    let x = origin.x + (octave as f32 * 7.0 + wi as f32) * WHITE_WIDTH;
                    let cx = x + WHITE_WIDTH / 2.0;
                    let cy = origin.y + WHITE_HEIGHT - 16.0;
                    painter.text(
                        pos2(cx, cy),
                        egui::Align2::CENTER_BOTTOM,
                        name.to_str(),
                        egui::FontId::proportional(14.0),
                        Color32::GRAY,
                    );
                }
            }
        }
    }
}
