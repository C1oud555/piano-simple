use egui::{self, Color32, Rect, Sense, Stroke, StrokeKind, Vec2, pos2};
use crate::theory::note::{Note, NoteName};

const WHITE_HEIGHT: f32 = 130.0;

#[derive(Debug, Clone, Copy)]
pub struct KeyboardRange {
    pub start: u8,
    pub end: u8,
}

impl KeyboardRange {
    pub fn new(start: u8, end: u8) -> Self {
        Self { start, end }
    }

    pub fn white_count(&self) -> usize {
        (self.start..=self.end)
            .filter(|m| !NoteName::from_semitone(*m).is_sharp())
            .count()
    }

    pub fn label(&self) -> String {
        format!("{} keys ({}–{})",
            self.end.saturating_sub(self.start) as usize + 1,
            Note::from_midi(self.start),
            Note::from_midi(self.end),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Normal,
    Correct,
    Wrong,
    Highlight,
}

pub struct PianoKeyboard {
    range: KeyboardRange,
    key_states: [KeyState; 128],
}

fn midi_to_key_label(midi: u8) -> Option<&'static str> {
    match midi {
        48 => Some("Z"), 49 => Some("S"), 50 => Some("X"), 51 => Some("D"),
        52 => Some("C"), 53 => Some("V"), 54 => Some("G"), 55 => Some("B"),
        56 => Some("H"), 57 => Some("N"), 58 => Some("J"), 59 => Some("M"),
        60 => Some("Q"), 61 => Some("2"), 62 => Some("W"), 63 => Some("3"),
        64 => Some("E"), 65 => Some("R"), 66 => Some("5"), 67 => Some("T"),
        68 => Some("6"), 69 => Some("Y"), 70 => Some("7"), 71 => Some("U"),
        72 => Some("I"), 73 => Some("9"), 74 => Some("O"), 75 => Some("0"),
        76 => Some("P"),
        _ => None,
    }
}

impl PianoKeyboard {
    pub fn new(range: KeyboardRange) -> Self {
        Self {
            range,
            key_states: [KeyState::Normal; 128],
        }
    }

    pub fn range(&self) -> &KeyboardRange {
        &self.range
    }

    pub fn set_range(&mut self, range: KeyboardRange) {
        self.range = range;
        self.reset_states();
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
        let w = ui.available_width().max(100.0);
        let white_count = self.range.white_count() as f32;
        if white_count == 0.0 {
            return None;
        }
        let white_w = w / white_count;
        let black_w = white_w * 0.625;
        let black_h = WHITE_HEIGHT * 0.635;

        let (response, painter) = ui.allocate_painter(
            Vec2::new(w, WHITE_HEIGHT),
            Sense::click(),
        );
        let origin = response.rect.min;
        let mut clicked = None;

        // click detection: black keys first
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                for midi in self.range.start..=self.range.end {
                    if !NoteName::from_semitone(midi).is_sharp() {
                        continue;
                    }
                    if let Some(bx) = self.black_abs_x(midi, white_w) {
                        let r = Rect::from_min_size(
                            pos2(bx, origin.y),
                            Vec2::new(black_w, black_h),
                        );
                        if r.contains(pos) {
                            clicked = Some(midi);
                            break;
                        }
                    }
                }

                if clicked.is_none() {
                    let mut wi = 0usize;
                    for midi in self.range.start..=self.range.end {
                        if NoteName::from_semitone(midi).is_sharp() {
                            continue;
                        }
                        let r = Rect::from_min_size(
                            pos2(origin.x + wi as f32 * white_w, origin.y),
                            Vec2::new(white_w, WHITE_HEIGHT),
                        );
                        if r.contains(pos) {
                            clicked = Some(midi);
                            break;
                        }
                        wi += 1;
                    }
                }
            }
        }

        // draw white keys
        let mut wi = 0usize;
        for midi in self.range.start..=self.range.end {
            if NoteName::from_semitone(midi).is_sharp() {
                continue;
            }
            let x = origin.x + wi as f32 * white_w;
            let rect = Rect::from_min_size(pos2(x, origin.y), Vec2::new(white_w, WHITE_HEIGHT));

            let s = self.key_states[midi as usize];
            let (fill, stroke) = match s {
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
                KeyState::Normal => (Color32::WHITE, Stroke::new(1.0, Color32::GRAY)),
            };

            painter.rect_filled(rect, 2.0, fill);
            painter.rect_stroke(rect, 2.0, stroke, StrokeKind::Inside);

            if let Some(lbl) = midi_to_key_label(midi) {
                painter.text(
                    pos2(rect.center().x, rect.bottom() - 6.0),
                    egui::Align2::CENTER_BOTTOM,
                    lbl,
                    egui::FontId::proportional(12.0),
                    Color32::from_gray(120),
                );
            }
            wi += 1;
        }

        // draw black keys
        for midi in self.range.start..=self.range.end {
            if !NoteName::from_semitone(midi).is_sharp() {
                continue;
            }
            if let Some(bx) = self.black_abs_x(midi, white_w) {
                let rect = Rect::from_min_size(
                    pos2(bx, origin.y),
                    Vec2::new(black_w, black_h),
                );
                let s = self.key_states[midi as usize];
                let (fill, stroke) = match s {
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

                if let Some(lbl) = midi_to_key_label(midi) {
                    painter.text(
                        pos2(rect.center().x, rect.top() + 10.0),
                        egui::Align2::CENTER_CENTER,
                        lbl,
                        egui::FontId::proportional(white_w.max(9.0).min(12.0)),
                        Color32::from_gray(180),
                    );
                }
            }
        }

        clicked
    }

    fn black_abs_x(&self, midi: u8, white_w: f32) -> Option<f32> {
        let semitone = midi % 12;
        let cde_block = 3.0 * white_w / 5.0;
        let fgab_block = 4.0 * white_w / 7.0;

        let offset_from_prev = match semitone {
            1 => Some(1.0 * cde_block),
            3 => Some(3.0 * cde_block - white_w),
            6 => Some(1.0 * fgab_block),
            8 => Some(3.0 * fgab_block - white_w),
            10 => Some(5.0 * fgab_block - 2.0 * white_w),
            _ => None,
        }?;

        let mut wi = 0i32;
        for m in self.range.start..midi {
            if !NoteName::from_semitone(m).is_sharp() {
                wi += 1;
            }
        }
        Some(wi as f32 * white_w + offset_from_prev)
    }
}
