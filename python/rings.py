# to run tests: python -m unittest -v rings

import unittest
import math
from colors import WHITE, BLACK, color
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
import patterns

class Ring(patterns.Pattern):
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        pass

def ring_pattern(color1, color2):
    return Ring(color1, color2)

class RingTestCase(unittest.TestCase):
    def test_creating_a_ring_pattern(self):
        pattern = ring_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, WHITE)
        self.assertEqual(pattern.b, BLACK)

    #def test_a_gradient_linearly_interpolates_between_colors(self):
        #pattern = gradient_pattern(WHITE, BLACK)
#
        #self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        #self.assertEqual(pattern.pattern_at(point(0.25, 0, 0)), color(0.75, 0.75, 0.75))
        #self.assertEqual(pattern.pattern_at(point(0.5, 0, 0)), color(0.5, 0.5, 0.5))
        #self.assertEqual(pattern.pattern_at(point(0.75, 0, 0)), color(0.25, 0.25, 0.25))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
