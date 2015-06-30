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
from wiggles.clocks import Rate
from wiggles.singleton import Singleton

class MockWallTime(object):
    """Simple clock used to ease testing.  Runs at 1.0 seconds per frame."""
    def __init__(self):
        self.frame_num = 0
        self.time = 0.0

    def tick(self):
        self.frame_num += 1
        self.time += 1.0


class TestRate(object):

    def test_round_trip(self):

        freq_hz = 60.0

        assert_equals(freq_hz, Rate(freq_hz).hz)
        assert_equals(freq_hz, Rate(freq_hz, 'hz').hz)

        freq_bpm = 120.0

        assert_equals(freq_bpm, Rate(freq_bpm, 'bpm').bpm)