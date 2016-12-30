//! Declarations of various re-used dataflow types and type aliases.

#[derive(Clone, Copy, Debug)]
/// Floating-point representation of a rate, permitting the use of various
/// unit semantics.
pub enum Rate {
    Hz(f64),
    Bpm(f64),
    Period(f64)
}

impl Rate {
    /// Convert a rate value into a floating-point value with implicit units of Hz.
    pub fn in_hz(&self) -> f64 {
        match *self {
            Rate::Hz(v) => v,
            Rate::Bpm(bpm) => bpm / 60.0,
            Rate::Period(seconds) => 1.0 / seconds
        }
    }
}