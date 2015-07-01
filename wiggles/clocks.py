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

class WallTime(object):
    """Simple placeholder class which provides wall time and frame number.

    Implemented as a Singleton.
    """
    # make this a singleton:
    __metaclass__ = Singleton

    def __init__(self, frame_num=0):
        self.frame_num = frame_num

    @property
    def frame_num(self):
        return self._frame_num

    # when the frame number is set, cache the current time
    @frame_num.setter
    def frame_num(self, value):
        self._time = time.time()
        self._frame_num = value

    @property
    def time(self):
        """The reference time for this frame."""
        return self._time


# decorator to ensure a clock is up to date
def check_if_current(method):
    from functools import wraps
    @wraps(method)
    def checked(*args, **kwargs):
        self = args[0]
        if not self.current():
            self.update()
        return method(*args, **kwargs)
    return checked

# TODO: refactor the Clock interface out of Clock and ClockMultiplier

class Clock(object):
    """Primitive class for clocks.

    Clocks periodically update their state when polled, and cache their values
    for the current and prior time.
    """

    def __init__(self, rate, phase=0.0, timebase=WallTime()):
        """Create a new clock with rate object and an initial phase.

        Args:
            rate (Rate): the rate that this clock will tick.
            phase (unit float): the initial phase of the clock.
            timebase: object that this clock uses to check the wall time and get
                the frame number.  Defaults to using the WallTime.
        """
        self.rate = rate
        self._phase = phase
        self.timebase = timebase
        self._frame_num = timebase.frame_num
        self._last_time = timebase.time
        self.accumulated_ticks = 0
        self.accumulated_phase = 0.0
        self.total_ticks = 0

    def current(self):
        return self._frame_num == self.timebase.frame_num

    def update(self):
        """Update and recompute the phase of this clock."""
        self._frame_num = self.timebase.frame_num
        current_time = self.timebase.time
        self.accumulated_phase = (current_time - self._last_time)*self.rate.hz
        new_phase = self._phase + self.accumulated_phase

        # this clock has ticked floor(new_phase) times since the last time it was
        # updated
        self.accumulated_ticks = int(new_phase)
        self.total_ticks += self.accumulated_ticks

        # wrap phase to the correct range
        self._phase = new_phase % 1.0

        self._last_time = current_time

    @property
    @check_if_current
    def frame_num(self):
        return self._frame_num

    @check_if_current
    def phase(self):
        return self._phase

    @check_if_current
    def ticks(self):
        return self.accumulated_ticks

class ClockMultiplier(object):
    """Clock which ticks faster or slower than another clock."""

    def __init__(self, source, mult=1.0):
        """Make a new multiplier on an existing clock."""
        self.source = source
        self.mult = mult
        self._phase = source.phase()
        self._frame_num = source.frame_num
        self.accumulated_ticks = 0
        self.accumulated_phase = 0.0
        self.total_ticks = 0

    def current(self):
        return self._frame_num == self.source.frame_num

    def update(self):
        """Update the phase of this clock based on the master."""
        self._frame_num = self.source.frame_num
        self.accumulated_phase = self.source.accumulated_phase * self.mult
        new_phase = self._phase + self.accumulated_phase

        # this clock has ticked floor(new_phase) times since the last time it was
        # updated
        self.accumulated_ticks = int(new_phase)
        self.total_ticks += self.accumulated_ticks

        # wrap phase to the correct range
        self._phase = new_phase % 1.0

    def resync(self):
        """Resync the phase of this multiplier to that of its master."""
        self._phase = self.source.phase() * self.mult

    @property
    @check_if_current
    def frame_num(self):
        return self._frame_num

    @check_if_current
    def phase(self):
        return self._phase

    @check_if_current
    def ticks(self):
        return self.accumulated_ticks