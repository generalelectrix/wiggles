# Copyright (C) 2015  Chris Macklin
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
"""The basis for timekeeping.

The main limitation of this module as presently written is that all updates are
performed as lazy pull operations.  Better would be some kind of push
infrastructure, which would eliminate all the "are you current?" checks needed
when asking things in this module for their current value.  That would require
bidirectional references and need more work around maintaining those references.
"""

from weakref import WeakKeyDictionary

import time
from wiggles.singleton import Singleton

class Rate(object):
    """Helper object for handling rates.

    Provides for automatic conversion between different rate-keeping systems.
    Intrinsically stored as hertz.
    """
    def __init__(self, rate, unit='Hz'):
        """Create a Rate object given a value and a unit of measure.

        unit can be either 'hz' or 'bpm', case-insensitive
        """
        # TODO: probably better to use some kind of class or singleton for Hz
        # and Bpm
        unit = unit.lower()
        if unit == 'hz':
            self.rate = rate
        elif unit == 'bpm':
            self.rate = rate / 60
        else:
            raise Exception("Could not interpret the unit '{}'".format(unit))

    @property
    def hz(self):
        return self.rate

    @property
    def bpm(self):
        return self.rate * 60


class Broadcaster(object):
    """Superclass for things that want to inform subscribers of updates.

    Holds its listeners in a WeakKeyDictionary to allow GC."""
    def _init_broadcast(self):
        """Implementing classes should call super().__init__"""
        self._listeners = WeakKeyDictionary()

    def add_listener(self, listener, method='_update'):
        """Add a listener and associated method name to call.

        method should be a string, NOT the method itself.

        Default method is the '_update' method, presumed to have one argument
        consisting of the new frame number.
        """
        self._listeners[listener] = method

    def remove_listener(self, listener):
        """Remove a listener."""
        try:
            del self._listeners[listener]
        except KeyError:
            pass

    def _notify_listeners(self, *args, **kwargs):
        """Notify listeners of an update."""
        for listener, method in self._listeners.iteritems():
            getattr(listener, method)(*args, **kwargs)


class Transciever(Broadcaster):
    """Base class for classes which receive updates and broadcast updates."""
    def _init_transciever(self, frame_num):
        """Initialize the broadcasting mechanism."""
        self._init_broadcast()
        self.frame_num = frame_num

    def _update(self, frame_num):
        """Update the frame number, call the update method, and notify listeners."""
        self.frame_num = frame_num
        self.update()
        self._notify_listeners(frame_num)

    def update(self):
        """Inheriting classes should override this method."""
        pass


class WallTime(Transciever):
    """Simple placeholder class which provides wall time and frame number.

    Implemented as a Singleton.
    """
    # make this a singleton:
    __metaclass__ = Singleton

    def __init__(self, frame_num=0):
        self._init_transciever(frame_num)

    def update(self):
        """Get the wall time for this frame."""
        self.time = time.time()


class ClockBase(object):
    """Clocks tick and provide their phase."""
    def __init__(self):
        self.phase = 0.0
        self.accumulated_phase = 0.0
        self.total_ticks = 0
        self.accumulated_ticks = 0

    @property
    def ticks(self):
        return self.accumulated_ticks

    def reset(self):
        """Reinitialize this clock to zero.

        Subclasses can override this if it makes sense for them.
        """
        self.phase = 0.0
        self.accumulated_phase = 0.0
        self.total_ticks = 0
        self.accumulated_ticks = 0


    def clock_update(self):
        new_phase = self.phase + self.accumulated_phase

        # this clock has ticked floor(new_phase) times since the last time it was
        # updated
        self.accumulated_ticks = int(new_phase)
        self.total_ticks += self.accumulated_ticks

        # wrap phase to the correct range
        self.phase = new_phase % 1.0


class Clock(ClockBase, Transciever):
    """Simple clock that ticks at a fixed rate."""

    def __init__(self, rate, phase=0.0, timebase=WallTime()):
        """Create a new clock with rate object and an initial phase.

        Args:
            rate (Rate): the rate that this clock will tick.
            phase (unit float): the initial phase of the clock.
            timebase: object that this clock uses to check the wall time and get
                the frame number.  Defaults to using the WallTime.
        """
        super(Clock, self).__init__()
        self._init_transciever(timebase.frame_num)

        self.rate = rate
        self.timebase = timebase
        timebase.add_listener(self)

        self._last_time = timebase.time

    def update(self):
        """Recompute the phase of this clock, and how many times it ticked."""
        current_time = self.timebase.time
        self.accumulated_phase = (current_time - self._last_time)*self.rate.hz
        self.clock_update()
        self._last_time = current_time


class ClockMultiplier(ClockBase, Transciever):
    """Clock which ticks faster or slower than another clock."""

    def __init__(self, source, mult=1.0):
        """Make a new multiplier on an existing clock."""
        super(ClockMultiplier, self).__init__()
        self._init_transciever(source.frame_num)

        self.source = source
        source.add_listener(self)

        self.mult = mult

    def reset(self):
        """Resync the phase of this multiplier to that of its master."""
        self.phase = self.source.phase
        self.accumulated_phase = 0.0
        self.accumulated_ticks = 0

    def update(self):
        """Update the phase of this clock based on the master."""
        self.accumulated_phase = self.source.accumulated_phase * self.mult
        self.clock_update()