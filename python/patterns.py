# to run tests: python -m unittest -v patterns

import unittest
from color import color

class Stripe():
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
