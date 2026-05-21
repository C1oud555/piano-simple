use crate::audio::AudioEngine;
use crate::exercises::settings::{ExerciseMode, KeyboardSize, Settings};
use crate::exercises::sight_read::SightReadExercise;
use crate::exercises::song::SongExercise;
use crate::exercises::Exercise;
use crate::input::InputManager;

pub struct PianoApp {
    input_manager: InputManager,
    exercise: Box<dyn Exercise>,
    audio: AudioEngine,
    settings: Settings,
    selected_song: usize,
}

impl PianoApp {
    pub fn new() -> Self {
        let settings = Settings::new();
        Self {
            input_manager: InputManager::new(),
            exercise: Box::new(SightReadExercise::new()),
            audio: AudioEngine::new(),
            settings,
            selected_song: 0,
        }
    }

    fn switch_exercise(&mut self) {
        match self.settings.mode {
            ExerciseMode::FreePractice => {
                self.exercise = Box::new(SightReadExercise::new());
            }
            ExerciseMode::SongPractice => {
                self.exercise = Box::new(SongExercise::new(self.selected_song));
            }
        }
    }
}

impl eframe::App for PianoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(event) = self.input_manager.poll_event(ctx) {
            self.audio.play_note(event.midi_note, event.velocity);
            self.exercise.handle_input(&event);
        }

        egui::SidePanel::left("settings_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Settings");
                ui.separator();
                ui.add_space(6.0);

                ui.label("Exercise Mode");
                let prev_mode = self.settings.mode;
                if ui
                    .selectable_label(
                        self.settings.mode == ExerciseMode::FreePractice,
                        "Free Practice",
                    )
                    .clicked()
                {
                    self.settings.mode = ExerciseMode::FreePractice;
                }
                if ui
                    .selectable_label(
                        self.settings.mode == ExerciseMode::SongPractice,
                        "Song Practice",
                    )
                    .clicked()
                {
                    self.settings.mode = ExerciseMode::SongPractice;
                }
                if self.settings.mode != prev_mode {
                    self.switch_exercise();
                }

                ui.add_space(12.0);

                ui.label("Keyboard Range");
                for &size in KeyboardSize::all() {
                    if ui
                        .selectable_label(self.settings.keyboard_size == size, size.label())
                        .clicked()
                    {
                        self.settings.keyboard_size = size;
                    }
                }

                if self.settings.mode == ExerciseMode::SongPractice {
                    ui.add_space(12.0);
                    ui.label("Select Song");
                    let songs = crate::songs::data::all_songs();
                    for (i, song) in songs.iter().enumerate() {
                        if ui
                            .selectable_label(self.selected_song == i, song.title)
                            .clicked()
                        {
                            self.selected_song = i;
                            self.switch_exercise();
                        }
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(event) = self.exercise.render(ui) {
                self.audio.play_note(event.midi_note, event.velocity);
                self.exercise.handle_input(&event);
            }
        });
    }
}

pub fn setup_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    visuals.panel_fill = egui::Color32::from_rgb(248, 244, 236);
    visuals.window_fill = egui::Color32::from_rgb(255, 252, 245);
    ctx.set_visuals(visuals);
}
