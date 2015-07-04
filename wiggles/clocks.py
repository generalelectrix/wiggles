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
    def _init_broadcast(self):
        self._listeners = WeakKeyDictionary()

    def add_listener(self, listener, method):
        self._listeners[listener] = method

    def remove_listener(self, listener):
        try:
            del self._listeners[listener]
        except KeyError:
            pass

    def _notify_listeners(self, *args, **kwargs):
        for listener, method in self._listeners.iteritems():
            getattr(listener, method)(*args, **kwargs)

class WallTime(Broadcaster):
    """Simple placeholder class which provides wall time and frame number.

    Implemented as a Singleton.
    """
    # make this a singleton:
    __metaclass__ = Singleton

    def __init__(self, frame_num=0):
        self.frame_num = frame_num
        self._init_broadcast()

    def update(self, frame_num):
        self.time = time.time()
        self._frame_num = frame_num
        self._notify_listeners(frame_num)



class Clock(Broadcaster):
    """Primitive class for clocks.

    Clocks update their state when their frame number is set.  They then alert
    all of their listeners of their new value.
    """

    def __init__(self, rate, phase=0.0, timebase=WallTime()):
        """Create a new clock with rate object and an initial phase.

        Args:
            rate (Rate): the rate that this clock will tick.
            phase (unit float): the initial phase of the clock.
            timebase: object that this clock uses to check the wall time and get
                the frame number.  Defaults to using the WallTime.
        """
        self._init_broadcast()
        self.rate = rate
        self.phase = phase
        self.timebase = timebase
        timebase.add_listener(self, method='update')

        self.frame_num = timebase.frame_num
        self._last_time = timebase.time
        self.accumulated_ticks = 0
        self.accumulated_phase = 0.0
        self.total_ticks = 0

        self._init_broadcast()

    @property
    def ticks(self):
        return self.accumulated_ticks


    def update(self, frame_num):
        """Update and recompute the phase of this clock."""
        self.frame_num = frame_num
        current_time = self.timebase.time
        self.accumulated_phase = (current_time - self._last_time)*self.rate.hz
        new_phase = self.phase + self.accumulated_phase

        # this clock has ticked floor(new_phase) times since the last time it was
        # updated
        self.accumulated_ticks = int(new_phase)
        self.total_ticks += self.accumulated_ticks

        # wrap phase to the correct range
        self.phase = new_phase % 1.0

        self._last_time = current_time
        self._notify_listeners(frame_num)


class ClockMultiplier(Broadcaster):
    """Clock which ticks faster or slower than another clock."""

    def __init__(self, source, mult=1.0):
        """Make a new multiplier on an existing clock."""
        self.source = source
        source.add_listener(self, method='update')
        self.mult = mult
        self.phase = source.phase
        self.frame_num = source.frame_num
        self.accumulated_ticks = 0
        self.accumulated_phase = 0.0
        self.total_ticks = 0

        self._init_broadcast()

    @property
    def ticks(self):
        return self.accumulated_ticks


    def update(self, frame_num):
        """Update the phase of this clock based on the master."""
        self.frame_num = frame_num
        self.accumulated_phase = self.source.accumulated_phase * self.mult
        new_phase = self.phase + self.accumulated_phase

        # this clock has ticked floor(new_phase) times since the last time it was
        # updated
        self.accumulated_ticks = int(new_phase)
        self.total_ticks += self.accumulated_ticks

        # wrap phase to the correct range
        self.phase = new_phase % 1.0

        self._notify_listeners(frame_num)

    def resync(self):
        """Resync the phase of this multiplier to that of its master."""
        self.phase = self.source.phase * self.mult