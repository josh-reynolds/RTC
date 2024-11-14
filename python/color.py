# to run tests: python -m unittest -v color

import unittest
import math
from tuple import flequal

class Color():
    def __init__(self, r, g, b):
        self.red = r
        self.green = g
        self.blue = b

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return flequal(self.red, other.red) and \
                   flequal(self.green, other.green) and \
                   flequal(self.blue, other.blue)
        else:
            return False

    def __ne__(self, other):
        return not self.__eq__(other)

    def __add__(self, other):
        return Color(self.red + other.red,
                     self.green + other.green,
                     self.blue + other.blue)

#    def __sub__(self, other):
#        return Tuple(self.x - other.x,
#                     self.y - other.y,
#                     self.z - other.z,
#                     self.w - other.w)

#    def __neg__(self):
#        return Tuple(-self.x, -self.y, -self.z, -self.w)

#    def __mul__(self, rhs):
#        if isinstance(rhs, float) or isinstance(rhs, int):
#            return Tuple(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)

#    def __rmul__(self, lhs):
#        return self.__mul__(lhs)

#    def __truediv__(self, rhs):
#        if isinstance(rhs, float) or isinstance(rhs, int):
#            return Tuple(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)

    def __str__(self):
        return "(" + str(self.red) + ", " + \
                     str(self.green) + ", " + \
                     str(self.blue) + ")"

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
