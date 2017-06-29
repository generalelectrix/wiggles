//! Basic waveform generation functions, wrapped into a Wiggles-compatible interface.
//! All of these waveform generators are pure functions.
extern crate wiggles_value;

use std::f64::consts::{PI, FRAC_PI_2 as HALF_PI};
use wiggles_value::*;

const TWOPI: f64 = 2.0 * PI;

// Helper functions to avoid having to call math functions as methods.
fn sin(x: f64) -> f64 { x.sin() }

// TODO: decide how we want to handle generation of bipolar vs. unipolar values.
// This is especially important in pulse mode.

/// Generate a unit-amplitude sine wave on the interval [-1.0, 1.0].
fn sine(
        Unipolar(angle): Unipolar,
        Unipolar(smoothing): Unipolar,
        Unipolar(duty_cycle): Unipolar,
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

fn triangle(
        Unipolar(angle): Unipolar,
        Unipolar(smoothing): Unipolar,
        Unipolar(duty_cycle): Unipolar,
        pulse: bool)
        -> Bipolar
{
    if angle > duty_cycle || duty_cycle == 0.0 {
        return Bipolar(0.0);
    }

    let angle = angle / duty_cycle;
    if pulse {
        if angle < 0.5 {
            Bipolar(2.0 * angle)
        }
        else {
            Bipolar(2.0 * (1.0 - angle))
        }
    }