# to run tests: python -m unittest -v patterns

import unittest
from color import color
from tuple import point

class Stripe():
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2

    def stripe_at(self, pt):
        return WHITE

def stripe_pattern(color1, color2):
    return Stripe(color1, color2)

BLACK = color(0, 0, 0)
WHITE = color(1, 1, 1)

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

        self.assertEqual(pattern.stripe_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.stripe_at(point(0, 1, 0)), WHITE)
        self.assertEqual(pattern.stripe_at(point(0, 2, 0)), WHITE)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
