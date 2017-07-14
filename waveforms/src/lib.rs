//! Basic waveform generation functions, wrapped into a Wiggles-compatible interface.
//! All of these waveform generators are pure functions.
extern crate wiggles_value;

use std::f64::consts::{PI, FRAC_PI_2 as HALF_PI};
use wiggles_value::*;

const TWOPI: f64 = 2.0 * PI;

// Helper functions to avoid having to call math functions as methods.
fn sin(x: f64) -> f64 { x.sin() }

/// Generate a unit-amplitude sine wave on the interval [-1.0, 1.0].
pub fn sine(
        Unipolar(angle): Unipolar,
        Unipolar(duty_cycle): Unipolar,
        pulse: bool,
        type_hint: Option<Datatype>)
        -> Data
{
    match type_hint {
        Some(Datatype::Unipolar) | None => Data::Unipolar(sine_unipolar(angle, duty_cycle)),
        Some(Datatype::Bipolar) => Data::Bipolar(sine_bipolar(angle, duty_cycle, pulse))
    }
}

/// Bipolar output specialized impl of sine function.
fn sine_bipolar(
        angle: f64,
        duty_cycle: f64,
        pulse: bool)
        -> Bipolar
{
    if angle > duty_cycle || duty_cycle == 0.0 {
        return Bipolar(0.0);
    }

    let angle = angle / duty_cycle;
    if pulse {
        Bipolar((sin(TWOPI * angle - HALF_PI) + 1.0) / 2.0)
    }
    else {
        Bipolar(sin(TWOPI + angle))
    }
}

/// Unipolar output specialized impl of sine function.
fn sine_unipolar(angle: f64, duty_cycle: f64) -> Unipolar {
    if angle > duty_cycle || duty_cycle == 0.0 {
        return Unipolar(0.0);
    }

    let angle = angle / duty_cycle;
    Unipolar((sin(TWOPI * angle - HALF_PI) + 1.0) / 2.0)
}

// fn sawtooth(
//         Unipolar(angle): Unipolar,
//         Unipolar(smoothing): Unipolar,
//         Unipolar(duty_cycle): Unipolar,
//         pulse: bool)
//         -> Bipolar
// {
//     if angle > duty_cycle || duty_cycle == 0.0 {
//         return Bipolar(0.0);
//     }

//     let angle = angle / duty_cycle;
//     if pulse {
//         if angle < 0.5 {
//             Bipolar(2.0 * angle)
//         }
//         else {
//             Bipolar(2.0 * (1.0 - angle))
//         }
//     }
//     else {
//         if angle < 0.25 {
//             Bipolar(4.0 * angle)
//         }
//     }