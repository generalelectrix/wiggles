//! An event loop for lighting controllers.
//! Clients poll the event loop which acts as an endless iterator of actions.
//! Provides debug-level logging for tracing event triggering.

extern crate log;

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::cmp;
use std::error::Error;
use std::iter::Iterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Event loop settings.
pub struct Settings {
    /// Fixed duration between updates.
    pub update_interval: Duration,
    /// Min duration between render events; could be more than this if the show is lagging due to
    /// limited computational resources.
    pub render_interval: Duration,
    /// Command the application to autosave at this interval.
    /// If None, do not use autosave.
    pub autosave_interval: Option<Duration>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            // update at 100 fps
            update_interval: Duration::from_millis(10),
            // DMX is limited to 50 fps max
            render_interval: Duration::from_millis(20),
            // By default do not autosave.
            autosave_interval: None,
        }
    }
}

/// Events that can occur, to be interpreted and acted upon by the show.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Event {
    /// Command to render a frame.
    Render,
    /// Command to update the show state using this delta-t.
    Update(Duration),
    /// Command to autosave the show.
    Autosave,
    /// Instruct the show that it has this Duration to do work until the next event.
    Idle(Duration),
}

/// 
enum State {

}

pub struct EventLoop {
    pub settings: Settings,
    last_update: Instant,
    last_render: Instant,
    last_autosave: Instant,
    first_frame: bool,
}

/// Return the number of nanoseconds represented by this Duration.
fn nanoseconds(duration: Duration) -> u64 {
    duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64
}

/// Return the number of nanoseconds until we need to perform an action.
/// If this action is overdue, return a negative value.
fn ns_until(now: Instant, last: Instant, interval: Duration) -> i64 {
    let should_run_at = last + interval;
    if should_run_at < now {
        -1 * nanoseconds(now.duration_since(should_run_at)) as i64
    }
    else {
        nanoseconds(should_run_at.duration_since(now)) as i64
    }
}

impl EventLoop {
    pub fn new() -> Self {
        let now = Instant::now();
        // Render our first frame immediately, with the first update coming one update interval
        // later.
        let settings = Settings::default();
        EventLoop {
            settings: settings,
            // The show starts initialized and does not need an update before render.
            last_update: now,
            // The first event is always a Render.
            last_render: now,
            last_autosave: now,
            first_frame: true,
        }
    }

    /// Get the next action that the application should undertake.
    pub fn next(&mut self) -> Event {
        if self.first_frame {
            self.first_frame = false;
            return Event::Render;
        }
        let now = Instant::now();
        let ns_until_render = ns_until(now, self.last_render, self.settings.render_interval);
        let ns_until_update = ns_until(now, self.last_update, self.settings.update_interval);
        let ns_until_autosave = 
            if let Some(interval) = self.settings.autosave_interval {
                ns_until(now, self.last_autosave, interval)
            }
            else {
                std::i64::MAX
            };
        let ns_until_next = cmp::min(cmp::min(ns_until_update, ns_until_render), ns_until_autosave);
        if ns_until_next <= 0 {
            if ns_until_next == ns_until_update {
                Event::Update(self.settings.update_interval)
            }
            else if ns_until_next == ns_until_render {
                Event::Render
            }
            else {
                Event::Autosave
            }
        }
        else {
            Event::Idle(Duration::new(0, ns_until_next as u32))
        }
    }
}