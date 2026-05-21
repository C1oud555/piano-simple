# Piano Simple — Development Plan

Five phases, each building on the previous. Every phase produces a working, testable app.

---

## Phase 1: Audio Engine

**Goal**: Play sound when a key is pressed (QWERTY, mouse, or future MIDI).

**Implementation**:
- Add `rodio` to `Cargo.toml`
- Create `src/audio/` module:
  - `AudioEngine` struct with a `rodio::OutputStream` + `rodio::Sink`
  - Method `play_note(midi_note: u8, duration: Duration)`:
    - Generate a sine wave at frequency `440.0 * 2.0^((midi - 69) / 12.0)`
    - Apply a simple ADSR-like envelope (fade in/out) to avoid clicks
    - Write via `rodio::source::SineWave` or custom `Source`
  - Method `stop_all()` for when note is released
- Thread `AudioEngine` through `PianoApp` → `Exercise::handle_input`
- Play sound on every `NoteEvent` (both QWERTY and mouse click)

**Key files**: `Cargo.toml`, `src/audio/mod.rs`, `src/audio/engine.rs`

---

## Phase 2: Flexible Piano Keyboard

**Goal**: Support any keyboard range (49/61/76/88 keys), auto-scale to fill window width, Neohesia-style layout.

**Implementation**:
- Rewrite `PianoKeyboard` to mirror Neohesia's `piano-layout` approach:
  - `white_key_count` determines `key_width = available_width / white_key_count`
  - Black key width = `key_width * 0.625`, height = `white_height * 0.635`
  - Two-zone octave subdivision (CDE block + FGAB block) for correct black key X positions
  - Keyboard anchored at bottom-left, spans full width
  - `KeyboardRange` struct: `{ start_note: u8, end_note: u8 }`
- QWERTY input mapping automatically updates to only map available keys
- QWERTY labels drawn on each key (moved to fit smaller keys if needed)

**Key files**: `src/ui/piano.rs`, `src/input/qwerty.rs`

---

## Phase 3: Settings & Exercise Selection

**Goal**: Settings panel + top-level navigation between exercise modes.

**Implementation**:
- Add a settings sidebar or top bar with:
  - **Keyboard range**: dropdown (49/61/76/88 keys)
  - **Exercise mode**: tabs or dropdown ("Free Practice" / "Song Practice")
- `PianoApp` holds the active exercise + the `KeyboardRange` setting
- `KeyboardRange` is passed to both the keyboard rendering and the input mapping
- Settings page is rendered as an egui `SidePanel` or `Window`

**Key files**: `src/app.rs`, `src/exercises/settings.rs`

---

## Phase 4: Song Practice

**Goal**: Play through preset songs shown on a staff, one note at a time.

**Implementation**:
- Define song data structure:
  ```rust
  struct SongNote { midi: u8, duration: f32, start_time: f32 }
  struct Song { title: &str, clef: Clef, notes: Vec<SongNote> }
  ```
- Create `src/songs/` with a few simple songs:
  - "Twinkle Twinkle Little Star" (C major, treble clef)
  - "Mary Had a Little Lamb" (C major)
  - "Ode to Joy" (C major)
  - "Happy Birthday" (F major)
- Create `SongExercise`:
  - Displays the song title
  - Shows the staff with ALL notes visible (simplified: one measure at a time)
  - Highlights the current note to be played
  - Waits for correct input → advances to next note
  - Shows progress (e.g., "5 / 32 notes played")
- Staff rendering enhancement:
  - Support multiple notes on the same staff
  - Show note durations as basic rhythm indicators (hollow/filled note heads, stems)
- Sound plays as each note is correctly played

**Key files**: `src/songs/mod.rs`, `src/songs/data.rs`, `src/exercises/song.rs`, `src/ui/staff.rs`

---

## Phase 5: Polish

**Goal**: Fine-tune the experience.

- Song selection screen (browse + pick a song)
- Results screen after completing a song (accuracy, time)
- Volume control for audio
- MIDI input integration test
- Keyboard shortcuts for common actions
- Visual improvements: key press animation, smoother feedback

---

## Dependency Roadmap

| Phase | New crate | Purpose |
|-------|-----------|---------|
| 1 | `rodio` | Audio output / tone generation |
| — | (none else required) | Everything else uses std + egui |

## File Tree After All Phases

```
src/
├── main.rs
├── app.rs                  # PianoApp: wires InputManager + AudioEngine + current Exercise
├── theory/
│   ├── mod.rs
│   └── note.rs             # NoteName, Note, MIDI
├── input/
│   ├── mod.rs              # InputManager, NoteEvent
│   ├── qwerty.rs           # QWERTY mapping (from KeyboardRange)
│   └── midi.rs             # Stub
├── audio/
│   ├── mod.rs
│   └── engine.rs           # AudioEngine (rodio sine wave playback)
├── ui/
│   ├── mod.rs
│   ├── piano.rs            # Flexible piano keyboard (Neothesia-style)
│   └── staff.rs            # Staff with multi-note support
├── exercises/
│   ├── mod.rs              # Exercise trait
│   ├── sight_read.rs       # Free practice (current)
│   ├── song.rs             # Song practice
│   └── settings.rs         # Settings panel
└── songs/
    ├── mod.rs
    └── data.rs             # Preset song definitions
```

## Verification

After each phase, run `cargo build` and `cargo run` to verify correctness.
