# to run tests: python -m unittest -v gradients

import unittest
import math
from colors import color, Color, WHITE, BLACK, LIGHT_GREY, GREY, DARK_GREY
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
import patterns
from solids import solid_pattern

class Gradient(patterns.Pattern):
    def __init__(self, pattern1, pattern2):
        self.a = pattern1
        self.b = pattern2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        pat_pt = self.transform.inverse() * pt
        distance = self.b.pattern_at(pat_pt) - self.a.pattern_at(pat_pt)
        fraction = pat_pt.x - math.floor(pat_pt.x)
        return self.a.pattern_at(pat_pt) + distance * fraction

def gradient_pattern(first, second):
    if isinstance(first, Color):
        first = solid_pattern(first)
    if isinstance(second, Color):
        second = solid_pattern(second)
    return Gradient(first, second)

class GradientTestCase(unittest.TestCase):
    def test_creating_a_gradient_pattern(self):
        pattern = gradient_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, solid_pattern(WHITE))
        self.assertEqual(pattern.b, solid_pattern(BLACK))

    def test_a_gradient_linearly_interpolates_between_colors(self):
        pattern = gradient_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0.25, 0, 0)), LIGHT_GREY)
        self.assertEqual(pattern.pattern_at(point(0.5, 0, 0)), GREY)
        self.assertEqual(pattern.pattern_at(point(0.75, 0, 0)), DARK_GREY)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
