use egui::{self, Color32, Pos2, Stroke, Vec2};
use crate::songs::data::SongNote;
use crate::theory::note::{Note, NoteName};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clef {
    Treble,
    Bass,
}

pub struct Staff {
    pub clef: Clef,
    spacing: f32,
}

impl Staff {
    pub fn new(clef: Clef) -> Self {
        Self { clef, spacing: 14.0 }
    }

    pub fn height(&self) -> f32 {
        self.spacing * 4.0 + 80.0
    }

    pub fn draw(&self, ui: &mut egui::Ui, note: &Note, highlighted: Option<bool>) {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(ui.available_width(), self.height()),
            egui::Sense::hover(),
        );

        let r = response.rect;
        let center_x = r.center().x;
        let staff_top = r.top() + 30.0;
        let staff_bot = staff_top + 4.0 * self.spacing;

        Self::draw_staff_lines(&painter, r, staff_top, self.spacing, r.left() + 40.0, r.right() - 20.0);
        Self::draw_clef(&painter, r, staff_top, self.spacing, self.clef);

        let diatonic = Self::diatonic_fn();
        let ref_diatonic = match self.clef {
            Clef::Treble => 30,
            Clef::Bass => 18,
        };
        let pos = diatonic(note) - ref_diatonic;
        let note_y = staff_bot - pos as f32 * self.spacing / 2.0;
        let note_x = center_x;

        Self::draw_ledger_lines(&painter, note_y, staff_top, staff_bot, self.spacing, pos, note_x);
        self.draw_note(&painter, note, note_x, note_y, highlighted);
    }

    pub fn draw_multi(&self, ui: &mut egui::Ui, notes: &[SongNote], current_idx: usize) {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(ui.available_width(), self.height()),
            egui::Sense::hover(),
        );

        let r = response.rect;
        let left_x = r.left() + 40.0;
        let right_x = r.right() - 20.0;
        let staff_top = r.top() + 30.0;
        let staff_bot = staff_top + 4.0 * self.spacing;

        Self::draw_staff_lines(&painter, r, staff_top, self.spacing, left_x, right_x);
        Self::draw_clef(&painter, r, staff_top, self.spacing, self.clef);

        let avail_w = right_x - left_x;
        let total = notes.len().max(1);
        let spacing_x = avail_w / (total as f32 + 1.0);
        let start_x = left_x + spacing_x;

        let diatonic = Self::diatonic_fn();
        let ref_diatonic = match self.clef {
            Clef::Treble => 30,
            Clef::Bass => 18,
        };

        let bar_width = spacing_x * 0.6;
        let mut bar_x = start_x + spacing_x * 0.2;

        for (i, sn) in notes.iter().enumerate() {
            let note = Note::from_midi(sn.midi);
            let pos = diatonic(&note) - ref_diatonic;
            let note_y = staff_bot - pos as f32 * self.spacing / 2.0;
            let note_x = bar_x + bar_width / 2.0;

            Self::draw_ledger_lines(&painter, note_y, staff_top, staff_bot, self.spacing, pos, note_x);

            let highlighted = if i == current_idx {
                Some(true)
            } else if i < current_idx {
                Some(false)
            } else {
                None
            };
            self.draw_note(&painter, &note, note_x, note_y, highlighted);

            bar_x += spacing_x;
        }
    }

    fn draw_staff_lines(painter: &egui::Painter, _r: egui::Rect, staff_top: f32, spacing: f32, left: f32, right: f32) {
        for i in 0..5 {
            let y = staff_top + i as f32 * spacing;
            painter.line_segment(
                [Pos2::new(left, y), Pos2::new(right, y)],
                Stroke::new(1.0, Color32::from_gray(140)),
            );
        }
    }

    fn draw_clef(painter: &egui::Painter, r: egui::Rect, staff_top: f32, spacing: f32, clef: Clef) {
        let glyph = match clef {
            Clef::Treble => "\u{1D11E}",
            Clef::Bass => "\u{1D122}",
        };
        painter.text(
            Pos2::new(r.left() + 28.0, staff_top + 2.0 * spacing),
            egui::Align2::CENTER_CENTER,
            glyph,
            egui::FontId::proportional(36.0),
            Color32::from_gray(60),
        );
    }

    fn diatonic_fn() -> impl Fn(&Note) -> i32 {
        |n: &Note| -> i32 {
            n.octave as i32 * 7
                + match n.name {
                    NoteName::C | NoteName::Cs => 0,
                    NoteName::D | NoteName::Ds => 1,
                    NoteName::E => 2,
                    NoteName::F | NoteName::Fs => 3,
                    NoteName::G | NoteName::Gs => 4,
                    NoteName::A | NoteName::As => 5,
                    NoteName::B => 6,
                }
        }
    }

    fn draw_ledger_lines(painter: &egui::Painter, note_y: f32, staff_top: f32, staff_bot: f32, spacing: f32, pos: i32, note_x: f32) {
        let line_ext = 16.0;
        if note_y < staff_top - spacing / 4.0 || note_y > staff_bot + spacing / 4.0 {
            if pos < 0 {
                let mut ly = staff_bot + spacing;
                while ly <= note_y + spacing / 4.0 {
                    painter.line_segment(
                        [Pos2::new(note_x - line_ext, ly), Pos2::new(note_x + line_ext, ly)],
                        Stroke::new(1.0, Color32::from_gray(140)),
                    );
                    ly += spacing;
                }
            } else if pos > 8 {
                let mut ly = staff_top - spacing;
                while ly >= note_y - spacing / 4.0 {
                    painter.line_segment(
                        [Pos2::new(note_x - line_ext, ly), Pos2::new(note_x + line_ext, ly)],
                        Stroke::new(1.0, Color32::from_gray(140)),
                    );
                    ly -= spacing;
                }
            }
        }
    }

    fn draw_note(&self, painter: &egui::Painter, note: &Note, note_x: f32, note_y: f32, highlighted: Option<bool>) {
        if note.name.is_sharp() {
            painter.text(
                Pos2::new(note_x - 22.0, note_y),
                egui::Align2::CENTER_CENTER,
                "\u{266F}",
                egui::FontId::proportional(16.0),
                Color32::from_gray(40),
            );
        }

        let note_color = match highlighted {
            Some(true) => Color32::from_rgb(60, 160, 60),
            Some(false) => Color32::from_rgb(200, 50, 50),
            None => Color32::BLACK,
        };

        painter.add(egui::Shape::ellipse_filled(
            Pos2::new(note_x, note_y),
            Vec2::new(8.0, 6.0),
            note_color,
        ));
    }
}
