use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (_stream, handle) = OutputStream::try_default().expect("failed to open audio output");
        let sink = Sink::try_new(&handle).expect("failed to create audio sink");
        Self { _stream, sink }
    }

    pub fn play_note(&self, midi: u8, _velocity: u8) {
        let freq = 440.0 * 2.0f32.powf((midi as f32 - 69.0) / 12.0);
        self.sink.append(PianoTone::new(freq));
    }
}

struct PianoTone {
    freq: f32,
    sample_rate: u32,
    pos: usize,
    total: usize,
}

impl PianoTone {
    fn new(freq: f32) -> Self {
        let sample_rate = 44100u32;
        let total = (sample_rate as f32 * 1.5) as usize;
        Self { freq, sample_rate, pos: 0, total }
    }

    fn envelope(&self, t: f32) -> f32 {
        if t < 0.008 {
            t / 0.008
        } else {
            (-(t - 0.008) * 2.8).exp()
        }
    }
}

impl Iterator for PianoTone {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.pos >= self.total {
            return None;
        }
        let t = self.pos as f32 / self.sample_rate as f32;
        let phase = std::f32::consts::TAU * t;
        let mut s = (self.freq * phase).sin();
        s += 0.55 * (2.0 * self.freq * phase).sin();
        s += 0.35 * (3.0 * self.freq * phase).sin();
        s += 0.18 * (4.0 * self.freq * phase).sin();
        s += 0.09 * (5.0 * self.freq * phase).sin();
        s += 0.04 * (6.0 * self.freq * phase).sin();
        s *= 0.35 * self.envelope(t);
        self.pos += 1;
        Some(s)
    }
}

impl Source for PianoTone {
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(self.total as f64 / self.sample_rate as f64))
    }
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.total.saturating_sub(self.pos))
    }
}
