use crate::ui::piano::KeyboardRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExerciseMode {
    FreePractice,
    SongPractice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardSize {
    Keys49,
    Keys61,
    Keys76,
    Keys88,
}

impl KeyboardSize {
    pub fn range(&self) -> KeyboardRange {
        match self {
            Self::Keys49 => KeyboardRange::new(48, 96),
            Self::Keys61 => KeyboardRange::new(48, 108),
            Self::Keys76 => KeyboardRange::new(40, 103),
            Self::Keys88 => KeyboardRange::new(21, 108),
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Keys49 => "49 keys (C3–C7)",
            Self::Keys61 => "61 keys (C3–C8)",
            Self::Keys76 => "76 keys (E2–G7)",
            Self::Keys88 => "88 keys (A0–C8)",
        }
    }

    pub fn all() -> &'static [KeyboardSize] {
        &[Self::Keys49, Self::Keys61, Self::Keys76, Self::Keys88]
    }
}

pub struct Settings {
    pub mode: ExerciseMode,
    pub keyboard_size: KeyboardSize,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            mode: ExerciseMode::FreePractice,
            keyboard_size: KeyboardSize::Keys49,
        }
    }
}
