# to run tests: python -m unittest -v radial_gradients

import unittest
import math
from colors import color, WHITE, BLACK, LIGHT_GREY, GREY, DARK_GREY
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
import patterns

class RadialGradient(patterns.Pattern):
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        distance = self.b - self.a
        fraction = math.sqrt(pt.x ** 2 + pt.z ** 2)
        return self.a + distance * fraction

def radial_gradient_pattern(color1, color2):
    return RadialGradient(color1, color2)

class RadialGradientTestCase(unittest.TestCase):
    def test_creating_a_radial_gradient_pattern(self):
        pattern = radial_gradient_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, WHITE)
        self.assertEqual(pattern.b, BLACK)

    def test_a_radial_gradient_linearly_interpolates_in_both_x_and_z(self):
        pattern = radial_gradient_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0.25, 0, 0)), LIGHT_GREY)
        self.assertEqual(pattern.pattern_at(point(0.5, 0, 0)), GREY)
        self.assertEqual(pattern.pattern_at(point(0.75, 0, 0)), DARK_GREY)

        self.assertEqual(pattern.pattern_at(point(0.25, 0, 0.25)), color(0.64645, 0.64645, 0.64645))
        self.assertEqual(pattern.pattern_at(point(0.5, 0, 0.5)), color(0.29289, 0.29289, 0.29289))
        self.assertEqual(pattern.pattern_at(point(0.75, 0, 0.75)), color(-0.06066, -0.06066, -0.06066))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
