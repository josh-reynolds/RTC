# to run tests: python -m unittest -v rings

import unittest
import math
from colors import WHITE, BLACK, color, Color
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity
import patterns
from solids import solid_pattern
from checkers import checker_pattern

class Ring(patterns.Pattern):
    def __init__(self, pattern1, pattern2):
        self.a = pattern1
        self.b = pattern2
        patterns.Pattern.__init__(self)

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return (self.a == other.a and
                    self.b == other.b)
        else:
            return False

    def pattern_at(self, pt):
        if math.floor(math.sqrt(pt.x ** 2 + pt.z ** 2)) % 2 == 0:
            return self.a.pattern_at(pt)
        else:
            return self.b.pattern_at(pt)

def ring_pattern(first, second):
    if isinstance(first, Color):
        first = solid_pattern(first)
    if isinstance(second, Color):
        second = solid_pattern(second)
    return Ring(first, second)

class RingTestCase(unittest.TestCase):
    def test_creating_a_ring_pattern(self):
        pattern = ring_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, solid_pattern(WHITE))
        self.assertEqual(pattern.b, solid_pattern(BLACK))

    def test_a_ring_should_extend_in_both_x_and_z(self):
        pattern = ring_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(1, 0, 0)), BLACK)
        self.assertEqual(pattern.pattern_at(point(0, 0, 1)), BLACK)
        self.assertEqual(pattern.pattern_at(point(0.708, 0, 0.708)), BLACK)

    def test_a_ring_can_contain_other_patterns(self):
        pattern = ring_pattern(checker_pattern(WHITE, BLACK),
                               WHITE)
        
        self.assertEqual(pattern.a, checker_pattern(WHITE, BLACK))
        self.assertEqual(pattern.b.a, WHITE)
        
# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
