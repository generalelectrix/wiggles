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
    def __init__(self):
        """Make a new Broadcaster."""
        self._listeners = WeakKeyDictionary()

    def add_listener(self, listener):
        """Add a listener."""
        self._listeners[listener] = None

    def remove_listener(self, listener):
        """Remove a listener."""
        try:
            del self._listeners[listener]
        except KeyError:
            pass

    def _update_listeners(self, *args, **kwargs):
        """Notify listeners of an update."""
        for listener, method in self._listeners.iteritems():
            getattr(listener, method)(*args, **kwargs)


class FrameUpdater(Broadcaster):
    """Responsible for cascading frame update commands."""
    def __init__(self, client):
        """Initialize the broadcasting mechanism and store a backlink to the client."""
        super(FrameUpdater, self).__init__()
        self.client = client

    def frame_update(self, frame_num):
        """Call the update method and notify listeners."""
        self.client._frame_update(frame_num)
        for listener in self._listeners.iterkeys():
            listener.frame_update(frame_num)

class FrameUpdated(object):
    """Mixin to add frame updater interface."""
    def add_frame_client(self, client):
        self.frame_updater.add_listener(client.frame_updater)

    def remove_frame_client(self, client):
        self.frame_updater.remove_listener(client.frame_updater)

class Synchronizer(Broadcaster):
    """Responsible for passing along clock synchronization commands."""
    def __init__(self, client):
        """Initialize the broadcasting mechanism and store a backlink to the client."""
        super(Synchronizer, self).__init__()
        self.client = client

    def reset(self):
        """Call the _reset method and notify listeners."""
        self.client._reset()
        for listener in self._listeners.iterkeys():
            listener.reset()

    def force_tick(self):
        """Call the _force_tick method and notify listeners."""
        self.client._force_tick()
        for listener in self._listeners.iterkeys():
            listener.force_tick()

class Synchronized(object):
    """Mixin to add synchronizer interface."""
    def add_synchronize_client(self, client):
        self.synchronizer.add_listener(client.synchronizer)

    def remove_synchronized_client(self, client):
        self.synchronizer.remove_listener(client.synchronizer)

    def reset(self):
        """Force this clock and any slaves to reset."""
        self.synchronizer.reset()

    def force_tick(self):
        """Force this clock and any slaves to tick on the next frame."""
        self.synchronizer.force_tick()


class FrameMaster(FrameUpdated):
    """Grand master who decides when a new frame should happen.

    Implemented as a Singleton.
    """

    # make this a singleton:
    __metaclass__ = Singleton
    def __init__(self, frame_num=0):
        self.frame_num = 0
        self.frame_updater = FrameUpdater(self)

    def frame_update(self):
        self.frame_num += 1
        self.frame_updater.frame_update(self.frame_num)

    def _frame_update(self, frame_num):
        pass

class WallTime(FrameUpdated):
    """Simple placeholder class which provides the wall time.

    Implemented as a Singleton.
    """
    # make this a singleton:
    __metaclass__ = Singleton

    def __init__(self, frame_num=0):
        self.frame_updater = FrameUpdater(self)
        FrameMaster().add_frame_client(self)
        self.time = None

    def _frame_update(self, frame_num):
        """Get the wall time for this frame."""
        self.time = time.time()


class ClockBase(FrameUpdated, Synchronized):
    """Clocks tick, count phase, update on frame updates, and synchronize."""
    def __init__(self):
        self.phase = 0.0
        self.accumulated_phase = 0.0
        self.ticks = 0

        self.force_tick = False

        self.frame_updater = FrameUpdater(self)
        self.synchronizer = Synchronizer(self)

    def add_slave(self, slave):
        """Add a clock slaved to this clock."""
        self.add_synchronize_client(slave)
        self.add_frame_client(slave)

    def remove_slave(self, slave):
        """Remove a slave."""
        self.remove_synchronized_client(slave)
        self.remove_frame_client(slave)

    def _frame_update(self, frame_num):
        """Subclasses should override this method."""
        raise Exception("Subclasses of ClockBase should override _frame_update")

    def _reset(self):
        """Reinitialize this clock to zero.

        Subclasses can override this if it makes sense for them.
        """
        self.phase = 0.0
        self.accumulated_phase = 0.0
        self.ticks = 0

    def _force_tick(self):
        """Command this clock to reset and tick on the next update."""
        self.force_tick = True


    def update_clock(self):
        new_phase = self.phase + self.accumulated_phase

        # this clock has ticked floor(new_phase) times since the last time it was
        # updated
        self.ticks = int(new_phase)

        # wrap phase to the correct range
        self.phase = new_phase % 1.0


class Clock(ClockBase):
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

        self.rate = rate
        self.phase = phase
        self.timebase = timebase
        timebase.add_frame_client(self)

        self._last_time = timebase.time

        self.sync_master = None

    def slave_to(self, master):
        """Have this clock listen to reset and force_tick messages from another.

        master can be None to stop synchronizing to another clock."""
        if self.sync_master:
            self.sync_master.remove_synchronized_client(self)
        self.sync_master = master
        if master:
            master.add_synchronize_client(self)

    def _frame_update(self, frame_num):
        """Recompute the phase of this clock, and how many times it ticked."""
        current_time = self.timebase.time

        # this check should only be necessary in the rate event that this clock
        # was started at the same time as the WallTime (or its timebase)
        # can probably think of a more clever way to refactor this away.
        if self._last_time is None:
            self._last_time = current_time

        if self.force_tick:
            self._reset()
            self.ticks = 1
            self.force_tick = False
        else:
            self.accumulated_phase = (current_time - self._last_time)*self.rate.hz
            self.update_clock()
        self._last_time = current_time


class ClockMultiplier(ClockBase):
    """Clock which ticks faster or slower than another clock."""

    def __init__(self, master, mult=1.0):
        """Make a new multiplier on an existing clock."""
        super(ClockMultiplier, self).__init__()
        self.master = None
        self.slave_to(master)

        self.phase = master.phase
        self.mult = mult

    def slave_to(self, master):
        """Slave this multiplier to a master clock."""
        if self.master:
            self.master.remove_slave(self)
        self.master = master
        master.add_slave(self)

    def _reset(self):
        """Resync the phase of this multiplier to that of its master."""
        self.phase = self.master.phase
        self.accumulated_phase = 0.0
        self.ticks = 0

    def _frame_update(self, frame_num):
        """Update the phase of this clock based on the master."""
        if self.force_tick:
            self.phase = 0.0
            self.accumulated_phase = 0.0
            self.ticks = 1
            self.force_tick = False
        else:
            self.accumulated_phase = self.master.accumulated_phase * self.mult
            self.update_clock()





