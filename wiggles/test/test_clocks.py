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
"""Tests for the clocks module."""

from nose.tools import assert_equals, assert_raises
from wiggles.test.isclose import assert_close
from wiggles.clocks import Rate, Clock, ClockMultiplier
from wiggles.singleton import Singleton

class MockWallTime(object):
    """Simple clock used to ease testing.  Runs at 1.0 seconds per frame."""
    def __init__(self):
        self.frame_num = 0
        self.time = 0.0

    def tick(self, num = 1):
        self.frame_num += num
        self.time += num


class TestRate(object):

    def test_round_trip(self):

        freq_hz = 60.0

        assert_equals(freq_hz, Rate(freq_hz).hz)
        assert_equals(freq_hz, Rate(freq_hz, 'hz').hz)

        freq_bpm = 120.0

        assert_equals(freq_bpm, Rate(freq_bpm, 'bpm').bpm)

class TestClock(object):

    def setUp(self):

        self.wt = MockWallTime()

    def test_clock_simple(self):

        wt = self.wt

        cl = Clock(Rate(0.5), timebase = wt)
        assert_equals(cl.phase(), 0.0)
        assert_equals(cl.ticks(), 0)
        wt.tick()
        assert_equals(cl.phase(), 0.5)
        assert_equals(cl.ticks(), 0)
        wt.tick()
        assert_equals(cl.phase(), 0.0)
        assert_equals(cl.ticks(), 1)

        # check to make sure clocks still work correctly if their timebase
        # doesn't start at zero
        cl = Clock(Rate(0.3), timebase = wt)
        assert_equals(cl.phase(), 0.0)
        wt.tick()
        assert_equals(cl.phase(), 0.3)
        assert_equals(cl.ticks(), 0)

        # check for repeated calls
        assert_equals(cl.phase(), 0.3)
        assert_equals(cl.phase(), 0.3)
        assert_equals(cl.phase(), 0.3)
        assert_equals(cl.ticks(), 0)

        wt.tick()
        assert_equals(cl.phase(), 0.6)

        # clock should update correctly if it did not poll
        wt.tick(2)
        assert_close(cl.phase(), 0.2)
        assert_equals(cl.ticks(), 1)

    def test_clock_mult(self):

        wt = self.wt

        cl = Clock(Rate(0.1), timebase = wt)
        cl_m = ClockMultiplier(cl, mult=2.0)

        assert_equals(cl.phase(), 0.0)
        assert_equals(cl_m.phase(), 0.0)
        wt.tick()
        assert_equals(cl.phase(), 0.1)
        assert_equals(cl_m.phase(), 0.2)

        # changing the rate at this point should have no effect yet
        cl.rate = Rate(0.15)
        assert_equals(cl.phase(), 0.1)
        assert_equals(cl_m.phase(), 0.2)

        wt.tick()
        assert_equals(cl.phase(), 0.25)        
        assert_equals(cl_m.phase(), 0.5)





