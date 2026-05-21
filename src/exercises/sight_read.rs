use crate::exercises::Exercise;
use crate::input::NoteEvent;
use crate::theory::note::Note;
use crate::ui::piano::{KeyState, PianoKeyboard};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SightReadMode {
    NoteToKey,
}

#[derive(Debug, Clone, PartialEq)]
enum QuestionState {
    Waiting,
    Correct,
    Wrong,
}

pub struct SightReadExercise {
    #[allow(dead_code)]
    mode: SightReadMode,
    current_note: Note,
    state: QuestionState,
    total: u32,
    correct: u32,
    range: (u8, u8),
    keyboard: PianoKeyboard,
    wrong_note: Option<u8>,
}

impl SightReadExercise {
    pub fn new() -> Self {
        let mut ex = Self {
            mode: SightReadMode::NoteToKey,
            current_note: Note::from_midi(60),
            state: QuestionState::Waiting,
            total: 0,
            correct: 0,
            range: (48, 76),
            keyboard: PianoKeyboard::new(48, 3),
            wrong_note: None,
        };
        ex.next_question();
        ex
    }

    fn next_question(&mut self) {
        let mut rng = rand::thread_rng();
        let midi = rng.gen_range(self.range.0..=self.range.1);
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
            self.keyboard.set_key_state(
                self.current_note.to_midi(),
                KeyState::Highlight,
            );
        }
    }
}

impl Exercise for SightReadExercise {
    fn name(&self) -> &str {
        match self.mode {
            SightReadMode::NoteToKey => "识键",
        }
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

    fn render(&mut self, ui: &mut egui::Ui) {
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
                        "得分: {}/{} ({:.0}%)",
                        self.correct, self.total, pct
                    ));
                },
            );
        });
        ui.separator();

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);

            let (note_color, feedback) = match self.state {
                QuestionState::Waiting => (egui::Color32::BLACK, ""),
                QuestionState::Correct => (egui::Color32::GREEN, "✓ 正确！按任意键继续"),
                QuestionState::Wrong => (egui::Color32::RED, "✗ 错误，按任意键继续"),
            };

            ui.label(
                egui::RichText::new(self.current_note.to_string())
                    .size(64.0)
                    .color(note_color),
            );

            ui.add_space(8.0);
            if !feedback.is_empty() {
                ui.label(
                    egui::RichText::new(feedback)
                        .size(18.0)
                        .color(note_color),
                );
            }

            ui.add_space(20.0);

            if let Some(midi) = self.keyboard.draw(ui) {
                if self.state == QuestionState::Waiting {
                    self.check_answer(midi);
                } else {
                    self.next_question();
                }
            }
        });
    }
}
