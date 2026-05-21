use crate::ui::staff::Clef;

#[derive(Debug, Clone)]
pub struct SongNote {
    pub midi: u8,
    pub duration: f32,
}

#[derive(Debug, Clone)]
pub struct Song {
    pub title: &'static str,
    pub clef: Clef,
    pub notes: Vec<SongNote>,
}

pub fn all_songs() -> Vec<Song> {
    vec![
        Song {
            title: "Twinkle Twinkle Little Star",
            clef: Clef::Treble,
            notes: twinkle_twinkle(),
        },
        Song {
            title: "Mary Had a Little Lamb",
            clef: Clef::Treble,
            notes: mary_lamb(),
        },
        Song {
            title: "Ode to Joy",
            clef: Clef::Treble,
            notes: ode_to_joy(),
        },
        Song {
            title: "Happy Birthday",
            clef: Clef::Treble,
            notes: happy_birthday(),
        },
    ]
}

fn twinkle_twinkle() -> Vec<SongNote> {
    let c4 = 60; let d4 = 62; let e4 = 64; let f4 = 65; let g4 = 67; let a4 = 69;
    vec![
        SongNote { midi: c4, duration: 1.0 }, SongNote { midi: c4, duration: 1.0 },
        SongNote { midi: g4, duration: 1.0 }, SongNote { midi: g4, duration: 1.0 },
        SongNote { midi: a4, duration: 1.0 }, SongNote { midi: a4, duration: 1.0 },
        SongNote { midi: g4, duration: 2.0 },
        SongNote { midi: f4, duration: 1.0 }, SongNote { midi: f4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: c4, duration: 2.0 },
    ]
}

fn mary_lamb() -> Vec<SongNote> {
    let e4 = 64; let d4 = 62; let c4 = 60; let g4 = 67;
    vec![
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: c4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: e4, duration: 2.0 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: d4, duration: 2.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: g4, duration: 1.0 },
        SongNote { midi: g4, duration: 2.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: c4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: c4, duration: 2.0 },
    ]
}

fn ode_to_joy() -> Vec<SongNote> {
    let c4 = 60; let d4 = 62; let e4 = 64; let f4 = 65; let g4 = 67;
    vec![
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: f4, duration: 1.0 }, SongNote { midi: g4, duration: 1.0 },
        SongNote { midi: g4, duration: 1.0 }, SongNote { midi: f4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: c4, duration: 1.0 }, SongNote { midi: c4, duration: 1.0 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.5 }, SongNote { midi: d4, duration: 0.5 },
        SongNote { midi: d4, duration: 2.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: f4, duration: 1.0 }, SongNote { midi: g4, duration: 1.0 },
        SongNote { midi: g4, duration: 1.0 }, SongNote { midi: f4, duration: 1.0 },
        SongNote { midi: e4, duration: 1.0 }, SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: c4, duration: 1.0 }, SongNote { midi: c4, duration: 1.0 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: c4, duration: 1.0 },
        SongNote { midi: c4, duration: 2.0 },
    ]
}

fn happy_birthday() -> Vec<SongNote> {
    let c4 = 60; let d4 = 62; let e4 = 64; let f4 = 65; let g4 = 67; let a4 = 69; let c5 = 72;
    vec![
        SongNote { midi: c4, duration: 0.75 }, SongNote { midi: c4, duration: 0.75 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: c4, duration: 1.0 },
        SongNote { midi: f4, duration: 1.0 }, SongNote { midi: e4, duration: 2.0 },
        SongNote { midi: c4, duration: 0.75 }, SongNote { midi: c4, duration: 0.75 },
        SongNote { midi: d4, duration: 1.0 }, SongNote { midi: c4, duration: 1.0 },
        SongNote { midi: g4, duration: 1.0 }, SongNote { midi: f4, duration: 2.0 },
        SongNote { midi: c4, duration: 0.75 }, SongNote { midi: c4, duration: 0.75 },
        SongNote { midi: c5, duration: 1.0 }, SongNote { midi: a4, duration: 1.0 },
        SongNote { midi: f4, duration: 1.0 }, SongNote { midi: e4, duration: 1.0 },
        SongNote { midi: d4, duration: 1.0 },
        SongNote { midi: a4, duration: 0.75 }, SongNote { midi: a4, duration: 0.75 },
        SongNote { midi: a4, duration: 1.0 }, SongNote { midi: f4, duration: 1.0 },
        SongNote { midi: g4, duration: 1.0 }, SongNote { midi: f4, duration: 2.0 },
    ]
}
