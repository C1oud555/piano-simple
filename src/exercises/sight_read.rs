use egui::{Color32, Vec2};
use rand::Rng;
use crate::exercises::Exercise;
use crate::input::NoteEvent;
use crate::theory::note::Note;
use crate::ui::piano::{KeyState, KeyboardRange, PianoKeyboard};
use crate::ui::staff::{Clef, Staff};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuestionState {
    Waiting,
    Correct,
    Wrong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SightReadMode {
    Single,
}

pub struct SightReadExercise {
    clef: Clef,
    #[allow(dead_code)]
    mode: SightReadMode,
    current_note: Note,
    state: QuestionState,
    total: u32,
    correct: u32,
    keyboard: PianoKeyboard,
    staff: Staff,
    wrong_note: Option<u8>,
}

impl SightReadExercise {
    pub fn new() -> Self {
        let mut ex = Self {
            clef: Clef::Treble,
            mode: SightReadMode::Single,
            current_note: Note::from_midi(60),
            state: QuestionState::Waiting,
            total: 0,
            correct: 0,
            keyboard: PianoKeyboard::new(KeyboardRange::new(48, 76)),
            staff: Staff::new(Clef::Treble),
            wrong_note: None,
        };
        ex.next_question();
        ex
    }

    fn next_question(&mut self) {
        let (lo, hi) = match self.clef {
            Clef::Treble => (64, 81), // E4 – C6
            Clef::Bass => (43, 60),   // G2 – C4
        };
        let midi = rand::thread_rng().gen_range(lo..=hi);
        self.current_note = Note::from_midi(midi);
        self.state = QuestionState::Waiting;
        self.wrong_note = None;
        self.keyboard.reset_states();
    }

    fn check_answer(&mut self, midi: u8) {
        self.total += 1;
        if midi == self.current_note.to_midi() {
            self.correct += 1;
            self.state = QuestionState::Correct;
            self.keyboard.set_key_state(midi, KeyState::Correct);
        } else {
            self.state = QuestionState::Wrong;
            self.wrong_note = Some(midi);
            self.keyboard.set_key_state(midi, KeyState::Wrong);
            self.keyboard
                .set_key_state(self.current_note.to_midi(), KeyState::Highlight);
        }
    }
}

impl Exercise for SightReadExercise {
    fn name(&self) -> &str {
        "Sight Read"
    }

    fn handle_input(&mut self, event: &NoteEvent) {
        match self.state {
            QuestionState::Waiting => {
                self.check_answer(event.midi_note);
            }
            QuestionState::Correct | QuestionState::Wrong => {
                self.next_question();
            }
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) -> Option<NoteEvent> {
        let avail_h = ui.available_height();
        let piano_h = 170.0;

        let mut out = None;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Piano Simple");
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        let pct = if self.total > 0 {
                            self.correct as f32 / self.total as f32 * 100.0
                        } else {
                            0.0
                        };
                        ui.label(format!(
                            "Score: {}/{} ({:.0}%)",
                            self.correct, self.total, pct
                        ));
                    },
                );
            });
            ui.separator();

            ui.horizontal(|ui| {
                let labels = ["Treble", "Bass", "Mixed"];
                let vals = [Clef::Treble, Clef::Bass, Clef::Treble];
                let mut sel = Clef::Treble;
                for (i, &label) in labels.iter().enumerate() {
                    if ui
                        .selectable_label(self.clef == vals[i], label)
                        .clicked()
                    {
                        sel = vals[i];
                    }
                }
                if sel != self.clef {
                    self.clef = sel;
                    self.staff = Staff::new(self.clef);
                    self.next_question();
                }
            });

            ui.add_space(8.0);

            let content_h = avail_h - piano_h - 60.0;
            ui.allocate_ui(
                Vec2::new(ui.available_width(), content_h),
                |ui| {
                    ui.vertical_centered(|ui| {
                        let highlighted = match self.state {
                            QuestionState::Correct => Some(true),
                            QuestionState::Wrong => Some(false),
                            QuestionState::Waiting => None,
                        };
                        self.staff.draw(ui, &self.current_note, highlighted);

                        ui.add_space(4.0);

                        let (text, color): (String, Color32) = match self.state {
                            QuestionState::Waiting => {
                                ("Play the note".into(), Color32::from_gray(100))
                            }
                            QuestionState::Correct => {
                                ("Correct! Press any key to continue".into(), Color32::from_rgb(40, 160, 40))
                            }
                            QuestionState::Wrong => {
                                (format!("Wrong — correct answer is {}", self.current_note), Color32::from_rgb(200, 40, 40))
                            }
                        };
                        ui.label(egui::RichText::new(text).size(16.0).color(color));
                    });
                },
            );

            ui.add_space(6.0);

            if let Some(midi) = self.keyboard.draw(ui) {
                out = Some(NoteEvent { midi_note: midi, velocity: 100 });
            }
        });

        out
    }
}
