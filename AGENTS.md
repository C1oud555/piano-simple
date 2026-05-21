# Piano Simple

## Commands

```sh
cargo run              # launch GUI app
cargo build --features midi   # build with MIDI input support (stub, needs midir)
```

## Architecture

- **`main.rs`** — entrypoint, creates eframe window (950×500)
- **`app.rs`** — `PianoApp` wires `InputManager` → `Box<dyn Exercise>` in an `eframe::App::update` loop
- **`exercises/`** — each exercise implements `Exercise` trait (`handle_input`, `render`). Add a new module + register in `PianoApp::new`
- **`input/`** — `InputManager` polls QWERTY keyboard; `midi.rs` is a placeholder behind `#[cfg(feature = "midi")]`
- **`ui/piano.rs`** — interactive 3-octave (C3–E5) piano keyboard widget with colour-coded `KeyState`
- **`theory/note.rs`** — `NoteName` / `Note` with MIDI conversion

## QWERTY layout

```
Bottom row (C3–B3): Z  S  X  D  C  V  G  B  H  N  J  M
Middle row (C4–B4): Q  2  W  3  E  R  5  T  6  Y  7  U
Top row    (C5–E5): I  9  O  0  P
```

White keys = letters, black keys = numbers.

## Adding a new exercise

1. Create `src/exercises/my_exercise.rs`
2. Implement `Exercise` for your struct
3. Swap it in `app.rs:13`: `Box::new(MyExercise::new())`
4. Later: wire exercise selection through the UI

## MIDI support

Not yet implemented. Enable with `--features midi` and implement `MidiInput::poll_event` in `src/input/midi.rs`. The `NoteEvent { midi_note, velocity }` type is the shared interface — new input sources only need to produce `NoteEvent`.
