# to run tests: python -m unittest -v solids

import unittest
import math
from colors import color, WHITE, BLACK
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
import patterns

class Solid(patterns.Pattern):
    def __init__(self, color):
        self.a = color
        self.b = None
        patterns.Pattern.__init__(self)

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return (self.a == other.a and
                    self.b == other.b)
        else:
            return False

    def pattern_at(self, pt):
        return self.a

def solid_pattern(color):
    return Solid(color)

class GradientTestCase(unittest.TestCase):
    def test_creating_a_solid_pattern(self):
        pattern1 = solid_pattern(WHITE)
        pattern2 = solid_pattern(BLACK)

        self.assertEqual(pattern1.a, WHITE)
        self.assertEqual(pattern2.a, BLACK)

    def test_a_solid_pattern_always_returns_the_same_color(self):
        pattern1 = solid_pattern(WHITE)
        pattern2 = solid_pattern(BLACK)

        self.assertEqual(pattern1.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern1.pattern_at(point(0.25, 0, 0.25)), WHITE)
        self.assertEqual(pattern1.pattern_at(point(0.5, 0.5, 0)), WHITE)
        self.assertEqual(pattern1.pattern_at(point(1, 1, 1)), WHITE)

        self.assertEqual(pattern2.pattern_at(point(0, 0, 0)), BLACK)
        self.assertEqual(pattern2.pattern_at(point(0.25, 0, 0.25)), BLACK)
        self.assertEqual(pattern2.pattern_at(point(0.5, 0.5, 0)), BLACK)
        self.assertEqual(pattern2.pattern_at(point(1, 1, 1)), BLACK)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
