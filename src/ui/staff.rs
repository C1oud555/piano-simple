use egui::{self, Color32, Pos2, Stroke, Vec2};
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

        for i in 0..5 {
            let y = staff_top + i as f32 * self.spacing;
            painter.line_segment(
                [Pos2::new(r.left() + 40.0, y), Pos2::new(r.right() - 20.0, y)],
                Stroke::new(1.0, Color32::from_gray(140)),
            );
        }

        let clef = match self.clef {
            Clef::Treble => "\u{1D11E}",
            Clef::Bass => "\u{1D122}",
        };
        painter.text(
            Pos2::new(r.left() + 28.0, staff_top + 2.0 * self.spacing),
            egui::Align2::CENTER_CENTER,
            clef,
            egui::FontId::proportional(36.0),
            Color32::from_gray(60),
        );

        let diatonic = |n: &Note| -> i32 {
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
        };

        let ref_diatonic = match self.clef {
            Clef::Treble => 30,
            Clef::Bass => 18,
        };

        let pos = diatonic(note) - ref_diatonic;
        let note_y = staff_bot - pos as f32 * self.spacing / 2.0;
        let note_x = center_x;

        let line_ext = 16.0;
        if note_y < staff_top - self.spacing / 4.0 || note_y > staff_bot + self.spacing / 4.0 {
            if pos < 0 {
                let mut ly = staff_bot + self.spacing;
                while ly <= note_y + self.spacing / 4.0 {
                    painter.line_segment(
                        [Pos2::new(note_x - line_ext, ly), Pos2::new(note_x + line_ext, ly)],
                        Stroke::new(1.0, Color32::from_gray(140)),
                    );
                    ly += self.spacing;
                }
            } else if pos > 8 {
                let mut ly = staff_top - self.spacing;
                while ly >= note_y - self.spacing / 4.0 {
                    painter.line_segment(
                        [Pos2::new(note_x - line_ext, ly), Pos2::new(note_x + line_ext, ly)],
                        Stroke::new(1.0, Color32::from_gray(140)),
                    );
                    ly -= self.spacing;
                }
            }
        }

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
