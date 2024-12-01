# to run tests: python -m unittest -v color

import unittest
import math
from utils import flequal

class Color():
    def __init__(self, r, g, b):
        self.red = r
        self.green = g
        self.blue = b

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return (flequal(self.red, other.red) and
                    flequal(self.green, other.green) and
                    flequal(self.blue, other.blue))
        else:
            return False

    def __ne__(self, other):
        return not self.__eq__(other)

    def __add__(self, other):
        return Color(self.red + other.red,
                     self.green + other.green,
                     self.blue + other.blue)

    def __sub__(self, other):
        return Color(self.red - other.red,
                     self.green - other.green,
                     self.blue - other.blue)

    def __mul__(self, rhs):
        if isinstance(rhs, float) or isinstance(rhs, int):
            return Color(self.red * rhs, self.green * rhs, self.blue * rhs)
        elif isinstance(rhs, Color):
            return Color(self.red * rhs.red,
                         self.green * rhs.green,
                         self.blue * rhs.blue)

    def __rmul__(self, lhs):
        return self.__mul__(lhs)

    def __str__(self):
        return "(" + (str(self.red) + ", " +
                     str(self.green) + ", " +
                     str(self.blue) + ")")

def color(r, g, b):
    return Color(r, g, b)

class ColorTestCase(unittest.TestCase):
    def test_colors_are_r_g_b_tuples(self):
        c = color(-0.5, 0.4, 1.7)
        self.assertEqual(c.red, -0.5)
        self.assertEqual(c.green, 0.4)
        self.assertEqual(c.blue, 1.7)

    def test_adding_colors(self):
        c1 = color(0.9, 0.6, 0.75)
        c2 = color(0.7, 0.1, 0.25)
        self.assertEqual(c1 + c2, color(1.6, 0.7, 1.0))

    def test_subtracting_colors(self):
        c1 = color(0.9, 0.6, 0.75)
        c2 = color(0.7, 0.1, 0.25)
        self.assertEqual(c1 - c2, color(0.2, 0.5, 0.5))

    def test_multiplying_color_by_scalar(self):
        c = color(0.2, 0.3, 0.4)
        self.assertEqual(c * 2, color(0.4, 0.6, 0.8))
        self.assertEqual(2 * c, color(0.4, 0.6, 0.8))

    def test_multiplying_two_colors(self):
        c1 = color(1, 0.2, 0.4)
        c2 = color(0.9, 1, 0.1)
        self.assertEqual(c1 * c2, color(0.9, 0.2, 0.04))
        self.assertEqual(c2 * c1, color(0.9, 0.2, 0.04))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
