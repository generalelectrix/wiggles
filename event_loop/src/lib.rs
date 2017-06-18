//! An event loop for lighting controllers.
//! Clients poll the event loop which acts as an endless iterator of actions.
//! Provides debug-level logging for tracing event triggering.

#[macro_use] extern crate log;

use std::time::{Duration, Instant};
use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq)]
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
            // By default autosave once a minute.
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

#[derive(Debug)]
struct LastEvents {
    update: Instant,
    render: Instant,
    autosave: Instant,
}

impl LastEvents {
    pub fn new(now: Instant) -> Self {
        LastEvents {
            update: now,
            render: now,
            autosave: now,
        }
    }
}

#[derive(Debug)]
pub struct EventLoop {
    pub settings: Settings,
    last: LastEvents,
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
            last: LastEvents::new(now),
        }
    }

    /// Reset the state of the event loop to a state where every event happened right now.
    pub fn reset(&mut self) {
        self.last = LastEvents::new(Instant::now());
    }

    /// Generate the next event.
    pub fn next(&mut self) -> Event {
        let now = Instant::now();
        next_event(now, &self.settings, &mut self.last)
    }
}


#[inline(always)]
/// Get the next action that the application should undertake.
/// Broken out as a function to enable deterministic testing.
fn next_event(now: Instant, settings: &Settings, last: &mut LastEvents) -> Event {
    let ns_until_render = ns_until(now, last.render, settings.render_interval);
    let ns_until_update = ns_until(now, last.update, settings.update_interval);
    let ns_until_autosave = 
        if let Some(interval) = settings.autosave_interval {
            ns_until(now, last.autosave, interval)
        }
        else {
            std::i64::MAX
        };
    let ns_until_next = cmp::min(cmp::min(ns_until_update, ns_until_render), ns_until_autosave);
    if ns_until_next <= 0 {
        if ns_until_next == ns_until_update {
            // Always update in exactly deterministic timesteps; if we're running low on
            // computational resources and we are behind on updates, "batch" several updates by
            // running one update step with an integer multiple of update interval.
            // This is really not ideal, so log it as a warning if we're doing this.
            // We only do this if we're more than two updates behind.
            let updates_needed = 1 + (ns_until_update.abs() as u64 / nanoseconds(settings.update_interval)) as u32;
            if updates_needed > 2 {
                // if we're three steps behind, run 2x; four steps -> 3x, etc. 
                let updates_to_run = updates_needed - 1;
                warn!(
                    "Event loop is {} updates behind, batching {} updates to try to catch up.",
                    updates_needed,
                    updates_to_run);
                let fat_update_interval = settings.update_interval * updates_to_run;
                last.update += fat_update_interval;
                Event::Update(fat_update_interval)
            }
            else {
                // garden variety update
                last.update += settings.update_interval;
                Event::Update(settings.update_interval)
            }
        }
        else if ns_until_next == ns_until_render {
            // ideally we should always update if our state is stale;
            last.render = now;
            Event::Render
        }
        else {
            last.autosave = now;
            Event::Autosave
        }
    }
    else {
        Event::Idle(Duration::new(0, ns_until_next as u32))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Event::*;

    #[inline(always)]
    /// Shorthand for creating a Duration from milliseconds.
    fn millis(milliseconds: u64) -> Duration {
        Duration::from_millis(milliseconds)
    }

    #[test]
    fn test_event_loop_no_autosave() {
        let mut now = Instant::now();
        let mut last = LastEvents::new(now);
        let update_interval = millis(10);
        let settings = Settings {
            update_interval: update_interval,
            render_interval: millis(13),
            autosave_interval: None,
        };

        let mut assert_event = |time, event| {
            assert_eq!(event, next_event(time, &settings, &mut last));
        };

        let update_event = Update(update_interval);

        assert_event(now, Idle(millis(10)));
        now += millis(5);
        assert_event(now, Idle(millis(5)));
        now += millis(5);
        assert_event(now, update_event);
        // If we run again immediately, we should be idle until render at 13.
        assert_event(now, Idle(millis(3)));
        now += millis(3);
        assert_event(now, Render);
        assert_event(now, Idle(millis(7)));
        // If we wait too long, we should still run the next pending event.
        now += millis(10);
        assert_event(now, update_event);
        assert_event(now, Idle(millis(3)));
        // Get through the next render, then wait long enough that we should batch updates.
        now += millis(3);
        assert_event(now, Render);
        // Delay more than 2 update cycles, make sure we get a fat update.
        now += millis(35);
        assert_event(now, Update(update_interval * 3));
        // We should get a render next, which we're late on as well.
        assert_event(now, Render);
        // We're still behind on updates, so we should get another.
        assert_event(now, update_event);
        assert_event(now, Idle(millis(9)));
    }

    #[test]
    fn test_event_loop_with_autosave() {
        let mut now = Instant::now();
        let mut last = LastEvents::new(now);
        let update_interval = millis(10);
        let settings = Settings {
            update_interval: update_interval,
            render_interval: millis(13),
            autosave_interval: Some(millis(17)),
        };

        let mut assert_event = |time, event| {
            assert_eq!(event, next_event(time, &settings, &mut last));
        };

        let update_event = Update(update_interval);

        assert_event(now, Idle(millis(10)));
        now += millis(10);
        assert_event(now, update_event);
        now += millis(17);
        assert_event(now, Render);
        assert_event(now, Autosave);
        assert_event(now, update_event);
        assert_event(now, Idle(millis(3)));
    }
}