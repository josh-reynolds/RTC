# to run tests: python -m unittest -v stripes

import unittest
import math
from colors import color, WHITE, BLACK
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
from patterns import Pattern

class Stripe(Pattern):
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2
        Pattern.__init__(self)

    def pattern_at(self, pt):
        if math.floor(pt.x) % 2 == 0:
            return self.a
        else:
            return self.b

def stripe_pattern(color1, color2):
    return Stripe(color1, color2)

class PatternsTestCase(unittest.TestCase):
    def test_color_constants(self):
        self.assertEqual(BLACK, color(0, 0, 0))
        self.assertEqual(WHITE, color(1, 1, 1))

    def test_creating_a_stripe_pattern(self):
        pattern = stripe_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, WHITE)
        self.assertEqual(pattern.b, BLACK)

    def test_a_stripe_pattern_is_constant_in_y(self):
        pattern = stripe_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 1, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 2, 0)), WHITE)

    def test_a_stripe_pattern_is_constant_in_z(self):
        pattern = stripe_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 0, 1)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 0, 2)), WHITE)

    def test_a_stripe_pattern_alternates_in_x(self):
        pattern = stripe_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(   0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point( 0.9, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(   1, 0, 0)), BLACK)
        self.assertEqual(pattern.pattern_at(point(-0.1, 0, 0)), BLACK)
        self.assertEqual(pattern.pattern_at(point(  -1, 0, 0)), BLACK)
        self.assertEqual(pattern.pattern_at(point(-1.1, 0, 0)), WHITE)

    def test_stripes_with_an_object_transformation(self):
        obj = spheres.sphere()
        obj.set_transform(scaling(2, 2, 2))
        pattern = stripe_pattern(WHITE, BLACK)

        c = pattern.pattern_at_shape(obj, point(1.5, 0, 0))

        self.assertEqual(c, WHITE)

    def test_stripes_with_a_pattern_transformation(self):
        obj = spheres.sphere()
        pattern = stripe_pattern(WHITE, BLACK)
        pattern.set_transform(scaling(2, 2, 2))

        c = pattern.pattern_at_shape(obj, point(1.5, 0, 0))

        self.assertEqual(c, WHITE)

    def test_stripes_with_both_pattern_and_object_transformations(self):
        obj = spheres.sphere()
        obj.set_transform(scaling(2, 2, 2))
        pattern = stripe_pattern(WHITE, BLACK)
        pattern.set_transform(translation(0.5, 0, 0))

        c = pattern.pattern_at_shape(obj, point(2.5, 0, 0))

        self.assertEqual(c, WHITE)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
