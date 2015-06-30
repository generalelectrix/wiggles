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
"""Implementation of Singleton design pattern."""

class Singleton(type):
    """Implementation of Singleton design pattern.

    Taken from
    http://stackoverflow.com/questions/6760685/creating-a-singleton-in-python

    Short explanation: this class has a dictionary that contains zero or one
    instances of a class.  When the constructor of a class that declares
    Singleton its __metaclass__ is called for the first time, the class'
    constructor is called and the result put into the dictionary keyed to the
    class.  When the constructor is called again, the existing instance is
    returned instead of contructing a new one.

    This pattern is useful here as it enables us to effectively have globally
    synchronized clocks without having to declare a global.
    """
    _instances = {}
    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]
