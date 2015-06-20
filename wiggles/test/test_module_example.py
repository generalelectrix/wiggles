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
"""Module of nosetest-style tests for module_example.

Run these from the wiggles module (.../wiggles/wiggles) by running
    nosetests
at the command line.
"""

from nose.tools import assert_equals, assert_raises

import wiggles.module_example as mod_ex

# tests can be bare functions
# see the nose documentation for how it discovers tests to run
def test_example_function():
    # since tests are only ever run by the test runner,
    # they don't really need docstrings.  I like to use long, explicit function
    # names that make it clear what is being tested, and that is usually enough.
    print_me = "I'm a test string, lah dee dah."
    assert_equals(print_me, mod_ex.example_function(print_me, True))

# a test that makes sure a function call raises an exception
def test_example_exception():

    # note that assert_raises takes an exception, a callable, then the arguments
    # to be passed to that callable as additional function arguments.
    assert_raises(mod_ex.NotIPError, mod_ex.example_exception, "not IP at all")
    assert_raises(mod_ex.NotIPError, mod_ex.example_exception, "still not IP")
    assert_equals("imaginary photons", mod_ex.example_exception("imaginary photons"))

class TestExampleClass(object):
    # you can also organize tests into classes.  see the nose docs for more on
    # this. Note that these tests are NOT in a class because they are testing a
    # class.  A test class is basically just a way to organize tests, and allow
    # nice organization inside test modules.  You can also create class-level
    # text fixtures; see the nose docs for more info.

    def setUp(self):
        # this is a magic method that nose will call before each test in the
        # class is run.  each test in the class will be an independent instance
        # of TestExampleClass, so if one test mutates self, the other tests will
        # not be affected.
        self.name = "imaginary photons"

    def test_is_ip(self):
        # self here is the instance of TestExampleClass, of course.  you can
        # store things like class-level test fixtures in the instance of the test
        # class

        # I'm only passing name as a keyword arg here to make it clear that
        # self.name refers to the instance of the test class, whose attribute
        # "name" is now being passed into the "name" field of the constructor.
        my_ex_cls_instance = mod_ex.ExampleClass(name=self.name)
        assert my_ex_cls_instance.is_ip()

        another_instance = mod_ex.ExampleClass("not IP")
        assert not another_instance.is_ip()


