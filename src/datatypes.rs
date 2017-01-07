//! Declarations of various re-used dataflow types and type aliases.
use std::fmt;
use std::error;
use knob::KnobMessage;
use clock_network::ClockMessage;

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


#[derive(Debug)]
/// Top-level wrapper for all subdomain errors.
pub enum ErrorMessage {
    Clock(ClockMessage),
    Knob(KnobMessage),
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorMessage::Clock(ref msg) => msg.fmt(f),
            ErrorMessage::Knob(ref msg) => msg.fmt(f),
        }
    }
}

impl error::Error for ErrorMessage {
    fn description(&self) -> &str { 
        match *self {
            ErrorMessage::Clock(ref msg) => msg.description(),
            ErrorMessage::Knob(ref msg) => msg.description(),
        }
     }

     fn cause(&self) -> Option<&error::Error> {
        match *self {
            ErrorMessage::Clock(ref msg) => Some(msg),
            ErrorMessage::Knob(ref msg) => Some(msg),
        }
     }
}

impl From<KnobMessage> for ErrorMessage {
    fn from(err: KnobMessage) -> Self {
        ErrorMessage::Knob(err)
    }
}

impl From<ClockMessage> for ErrorMessage {
    fn from(err: ClockMessage) -> Self {
        ErrorMessage::Clock(err)
    }
}