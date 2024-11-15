# to run tests: python -m unittest -v color

import unittest
#import math
#from tuple import flequal

class Canvas():
    def __init__(self, width, height):
        self.width = width
        self.height = height

#    def __eq__(self, other):
#        if isinstance(other, self.__class__):
#            return flequal(self.red, other.red) and \
#                   flequal(self.green, other.green) and \
#                   flequal(self.blue, other.blue)
#        else:
#            return False

#    def __ne__(self, other):
#        return not self.__eq__(other)

#    def __add__(self, other):
#        return Color(self.red + other.red,
#                     self.green + other.green,
#                     self.blue + other.blue)

#    def __sub__(self, other):
#        return Color(self.red - other.red,
#                     self.green - other.green,
#                     self.blue - other.blue)

#    def __neg__(self):
#        return Tuple(-self.x, -self.y, -self.z, -self.w)

#    def __mul__(self, rhs):
#        if isinstance(rhs, float) or isinstance(rhs, int):
#            return Color(self.red * rhs, self.green * rhs, self.blue * rhs)
#        elif isinstance(rhs, Color):
#            return Color(self.red * rhs.red,
#                         self.green * rhs.green,
#                         self.blue * rhs.blue)

#    def __rmul__(self, lhs):
#        return self.__mul__(lhs)

#    def __truediv__(self, rhs):
#        if isinstance(rhs, float) or isinstance(rhs, int):
#            return Tuple(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)

#    def __str__(self):
#        return "(" + str(self.red) + ", " + \
#                     str(self.green) + ", " + \
#                     str(self.blue) + ")"

def canvas(width, height):
    return Canvas(width, height)

class CanvasTestCase(unittest.TestCase):
    def test_creating_a_canvas(self):
        c = canvas(10, 20)
        self.assertEqual(c.width, 10)
        self.assertEqual(c.height, 20)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
