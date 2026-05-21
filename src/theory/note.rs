use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteName {
    C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B,
}

impl NoteName {
    pub fn from_semitone(semitone: u8) -> Self {
        match semitone % 12 {
            0 => Self::C,
            1 => Self::Cs,
            2 => Self::D,
            3 => Self::Ds,
            4 => Self::E,
            5 => Self::F,
            6 => Self::Fs,
            7 => Self::G,
            8 => Self::Gs,
            9 => Self::A,
            10 => Self::As,
            11 => Self::B,
            _ => unreachable!(),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::C => "C", Self::Cs => "C#", Self::D => "D",
            Self::Ds => "D#", Self::E => "E", Self::F => "F",
            Self::Fs => "F#", Self::G => "G", Self::Gs => "G#",
            Self::A => "A", Self::As => "A#", Self::B => "B",
        }
    }

    #[allow(dead_code)]
    pub fn is_sharp(&self) -> bool {
        matches!(self, Self::Cs | Self::Ds | Self::Fs | Self::Gs | Self::As)
    }

    pub fn semitone(&self) -> u8 {
        match self {
            Self::C => 0, Self::Cs => 1, Self::D => 2, Self::Ds => 3,
            Self::E => 4, Self::F => 5, Self::Fs => 6, Self::G => 7,
            Self::Gs => 8, Self::A => 9, Self::As => 10, Self::B => 11,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub name: NoteName,
    pub octave: i8,
}

impl Note {
    pub fn from_midi(midi: u8) -> Self {
        let name = NoteName::from_semitone(midi);
        let octave = (midi as i8 / 12) - 1;
        Self { name, octave }
    }

    pub fn to_midi(&self) -> u8 {
        ((self.octave + 1) * 12 + self.name.semitone() as i8) as u8
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name.to_str(), self.octave)
    }
}
