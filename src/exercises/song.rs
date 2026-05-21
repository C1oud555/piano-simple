use egui::Vec2;
use crate::exercises::Exercise;
use crate::input::NoteEvent;
use crate::songs::data::{all_songs, SongNote};
use crate::ui::piano::{KeyState, KeyboardRange, PianoKeyboard};
use crate::ui::staff::Staff;

pub struct SongExercise {
    notes: Vec<SongNote>,
    title: &'static str,
    current_note_idx: usize,
    completed: bool,
    total: u32,
    correct: u32,
    state: QuestionState,
    keyboard: PianoKeyboard,
    staff: Staff,
}

enum QuestionState {
    Waiting,
    Correct,
}

impl SongExercise {
    pub fn new(song_idx: usize) -> Self {
        let songs = all_songs();
        let safe_idx = song_idx.min(songs.len().saturating_sub(1));
        let song = songs.into_iter().nth(safe_idx).unwrap();
        Self {
            notes: song.notes,
            title: song.title,
            current_note_idx: 0,
            completed: false,
            total: 0,
            correct: 0,
            state: QuestionState::Waiting,
            keyboard: PianoKeyboard::new(KeyboardRange::new(48, 76)),
            staff: Staff::new(song.clef),
        }
    }

    fn current_note_midi(&self) -> Option<u8> {
        self.notes.get(self.current_note_idx).map(|n| n.midi)
    }

    fn check_answer(&mut self, midi: u8) {
        self.total += 1;
        if let Some(expected) = self.current_note_midi() {
            if midi == expected {
                self.correct += 1;
                self.state = QuestionState::Correct;
                self.keyboard.set_key_state(midi, KeyState::Correct);
                self.current_note_idx += 1;
                if self.current_note_idx >= self.notes.len() {
                    self.completed = true;
                }
            } else {
                self.keyboard.set_key_state(midi, KeyState::Wrong);
                self.keyboard.set_key_state(expected, KeyState::Highlight);
            }
        }
    }
}

impl Exercise for SongExercise {
    fn name(&self) -> &str {
        "Song Practice"
    }

    fn handle_input(&mut self, event: &NoteEvent) {
        if self.completed {
            return;
        }
        match self.state {
            QuestionState::Waiting => {
                self.check_answer(event.midi_note);
            }
            QuestionState::Correct => {
                self.state = QuestionState::Waiting;
                self.keyboard.reset_states();
            }
        }
    }

    fn render(&mut self, ui: &mut egui::Ui) -> Option<NoteEvent> {
        let avail_h = ui.available_height();
        let piano_h = 170.0;
        let mut out = None;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading(self.title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let pct = if self.total > 0 {
                        self.correct as f32 / self.total as f32 * 100.0
                    } else {
                        0.0
                    };
                    ui.label(format!(
                        "Score: {}/{} ({:.0}%)",
                        self.correct, self.total, pct
                    ));
                    ui.label(format!(
                        "Note {}/{}",
                        self.current_note_idx + 1,
                        self.notes.len()
                    ));
                });
            });
            ui.separator();

            let content_h = avail_h - piano_h - 60.0;
            ui.allocate_ui(Vec2::new(ui.available_width(), content_h), |ui| {
                ui.vertical_centered(|ui| {
                    if self.completed {
                        ui.label(
                            egui::RichText::new("Song complete! 🎉").size(20.0).color(
                                egui::Color32::from_rgb(40, 160, 40),
                            ),
                        );
                        ui.add_space(8.0);
                        let pct = if self.total > 0 {
                            self.correct as f32 / self.total as f32 * 100.0
                        } else {
                            0.0
                        };
                        ui.label(format!(
                            "Final score: {}/{} ({:.0}%)",
                            self.correct, self.total, pct
                        ));
                    } else {
                        let idx = if let QuestionState::Correct = self.state {
                            self.current_note_idx.saturating_sub(1)
                        } else {
                            self.current_note_idx
                        };
                        self.staff.draw_multi(ui, &self.notes, idx);

                        ui.add_space(4.0);

                        let text: String = match self.state {
                            QuestionState::Waiting => "Play the highlighted note".into(),
                            QuestionState::Correct => {
                                "Correct! Press any key to continue".into()
                            }
                        };
                        let color = match self.state {
                            QuestionState::Waiting => egui::Color32::from_gray(100),
                            QuestionState::Correct => egui::Color32::from_rgb(40, 160, 40),
                        };
                        ui.label(egui::RichText::new(text).size(16.0).color(color));
                    }
                });
            });

            ui.add_space(6.0);

            if let Some(midi) = self.keyboard.draw(ui) {
                out = Some(NoteEvent {
                    midi_note: midi,
                    velocity: 100,
                });
            }
        });

        out
    }
}
