# Piano Simple

A piano learning desktop application built with Rust and egui.

## Features

- **Sight Reading** — practice identifying notes on the keyboard
- **QWERTY Input** — use your computer keyboard as a piano (3 octaves)
- **Interactive Piano** — click the on-screen keyboard with your mouse

## Quick Start

```sh
cargo run
```

## QWERTY Layout

```
C3–B3: Z  S  X  D  C  V  G  B  H  N  J  M
C4–B4: Q  2  W  3  E  R  5  T  6  Y  7  U
C5–E5: I  9  O  0  P
```

White keys = letters, black keys = numbers.

## MIDI Support

Enable with `--features midi` (not yet implemented — a stub is ready for development).

## License

GNU General Public License v3.0 or later — see [LICENSE](LICENSE).
