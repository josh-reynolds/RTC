# to run tests: python -m unittest -v stripes

import unittest
import math
from colors import color, Color, WHITE, BLACK
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
import patterns
from solids import solid_pattern

class Stripe(patterns.Pattern):
    def __init__(self, pattern1, pattern2):
        self.a = pattern1
        self.b = pattern2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        pat_pt = self.transform.inverse() * pt
        if math.floor(pat_pt.x) % 2 == 0:
            return self.a.pattern_at(pat_pt)
        else:
            return self.b.pattern_at(pat_pt)

def stripe_pattern(first, second):
    if isinstance(first, Color):
        first = solid_pattern(first)
    if isinstance(second, Color):
        second = solid_pattern(second)
    return Stripe(first, second)

class StripeTestCase(unittest.TestCase):
    def test_creating_a_stripe_pattern(self):
        pattern = stripe_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, solid_pattern(WHITE))
        self.assertEqual(pattern.b, solid_pattern(BLACK))

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
