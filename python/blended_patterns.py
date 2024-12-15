# to run tests: python -m unittest -v Blend

import unittest
import math
from colors import WHITE, BLACK, GREY, color, Color
import spheres
import patterns
from tuples import point
from solids import solid_pattern

class Blend(patterns.Pattern):
    def __init__(self, pattern1, pattern2):
        self.a = pattern1
        self.b = pattern2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        pat_pt = self.transform.inverse() * pt
        color1 = self.a.pattern_at(pat_pt)
        color2 = self.b.pattern_at(pat_pt)
        return color((color1.red + color2.red)/2,
                     (color1.green + color2.green)/2,
                     (color1.blue + color2.blue)/2)

def blend_pattern(first, second):
    if isinstance(first, Color):
        first = solid_pattern(first)
    if isinstance(second, Color):
        second = solid_pattern(second)
    return Blend(first, second)

class BlendTestCase(unittest.TestCase):
    def test_creating_a_blend_pattern(self):
        pattern = blend_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, solid_pattern(WHITE))
        self.assertEqual(pattern.b, solid_pattern(BLACK))

    def test_blends_should_blend_color(self):
        pattern = blend_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), GREY)
        self.assertEqual(pattern.pattern_at(point(0.99, 0, 0)), GREY)
        self.assertEqual(pattern.pattern_at(point(1.01, 0, 0)), GREY)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
