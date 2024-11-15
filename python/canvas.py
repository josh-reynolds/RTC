# to run tests: python -m unittest -v color

import unittest
#import math
#from tuple import flequal
from color import color

class Canvas():
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.pixels = [[color(0,0,0) for x in range(width)] for x in range(height)]

    def write_pixel(self, x, y, color):
        self.pixels[y][x] = color

    def pixel_at(self, x, y):
        return self.pixels[y][x]

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

    def __str__(self):
        result = ""
        for x in range(self.height):
            for y in range(self.width):
                result += self.pixels[x][y].__str__()
            result += "\n"
        return result

#    def __getitem__(self, w):
#        return self.pixels[w]

def canvas(width, height):
    return Canvas(width, height)

class CanvasTestCase(unittest.TestCase):
    def test_creating_a_canvas(self):
        c = canvas(10, 20)
        self.assertEqual(c.width, 10)
        self.assertEqual(c.height, 20)

        default = color(0,0,0)
        for row in c.pixels:
            for pixel in row:
                self.assertEqual(pixel, default)

    def test_writing_pixels_to_canvas(self):
        c = canvas(10, 20)
        red = color(1, 0, 0)
        c.write_pixel(2, 3, red)
        self.assertEqual(c.pixel_at(2, 3), red)


# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
