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
"""This is the module-level docstring for module_example.

The docstring should have a concise, one-line description, possibly followed by
a nice block of more descriptive text.  Individual module members have their own
docstrings, so you don't need to list them here or anything.  Note the use of
triple-quotes to set a multi-line string.
It is conventional that multi-line docstrings have their closing triple-quotes
on their own line.
"""

def example_function(to_print, default_return_behavior=False):
    """An example of python function syntax/documentation.

    At least the one-line description.  If it is non-trivial, a block of text
    following like this is appreciated.  I've been using google's python style
    guide, so the function inputs/outputs would be documented like this:

    Args:
        to_print (str): print this to std out
        default_return_behavior=False (bool): if True, also return to_print

    Returns:
        str if default_return_behavior
    """
    print to_print
    if default_return_behavior:
        return to_print

def example_exception(str_in):
    """Raises an exception if its input isn't "imaginary photons".

    Since this is a tiny function with simple behavior, I'd probably not write
    out the whole google-style Args/etc, but for this example I will.

    Args:
        str_in (str): string to evaluate

    Returns:
        the string "imaginary photons" if the input was "imaginary photons".

    Raises:
        NotIPError if the input is not "imaginary photons"
    """
    if str_in != "imaginary photons":
        # note that an open paren continues a statement to another line
        raise NotIPError("This string: '{}' is some non-IP bullshit!"
                          .format(str_in))
    return str_in

# defining user-defined exceptions is usually just a matter of subclassing
# Exception with an empty class body, providing a new class of error that takes
# a string as a payload.
class NotIPError(Exception):
    """An error indicating a non imaginary photons condition has been detected."""
    pass

class ExampleClass(object):
    """A class to show off class-level documentation.

    Some more info about this class.  All classes should inherit from object if
    they don't inherit from anything else.

    You could document the constructor here.  I prefer to document the constructor
    in its own docstring attached to the function.  Either way is fine, we should
    just be consistent.
    """
    def __init__(self, name):
        """Create a new ExampleClass with a name field."""
        self.name = name

    def is_ip(self):
        """Returns True if this instance's name is "imaginary photons"."""
        return self.name == "imaginary photons"
