# to run tests: python -m unittest -v gradients

import unittest
import math
from colors import WHITE, BLACK
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
import patterns

class Gradient(patterns.Pattern):
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        if math.floor(pt.x) % 2 == 0:
            return self.a
        else:
            return self.b

def gradient_pattern(color1, color2):
    return Gradient(color1, color2)

class GradientTestCase(unittest.TestCase):
    def test_creating_a_gradient_pattern(self):
        pattern = gradient_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, WHITE)
        self.assertEqual(pattern.b, BLACK)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
