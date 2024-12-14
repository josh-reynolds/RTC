# to run tests: python -m unittest -v checkers

import unittest
import math
from colors import WHITE, BLACK, color, Color
import spheres
import patterns
from tuples import point
from solids import solid_pattern

class Checker(patterns.Pattern):
    def __init__(self, pattern1, pattern2):
        self.a = pattern1
        self.b = pattern2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        pat_pt = self.transform.inverse() * pt
        if (math.floor(pat_pt.x) + math.floor(pat_pt.y) + math.floor(pat_pt.z)) % 2 == 0:
            return self.a.pattern_at(pat_pt)
        else:
            return self.b.pattern_at(pat_pt)

def checker_pattern(first, second):
    if isinstance(first, Color):
        first = solid_pattern(first)
    if isinstance(second, Color):
        second = solid_pattern(second)
    return Checker(first, second)

class CheckerTestCase(unittest.TestCase):
    def test_creating_a_checker_pattern(self):
        pattern = checker_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, solid_pattern(WHITE))
        self.assertEqual(pattern.b, solid_pattern(BLACK))

    def test_checkers_should_repeat_in_x(self):
        pattern = checker_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0.99, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(1.01, 0, 0)), BLACK)

    def test_checkers_should_repeat_in_y(self):
        pattern = checker_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 0.99, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 1.01, 0)), BLACK)

    def test_checkers_should_repeat_in_z(self):
        pattern = checker_pattern(WHITE, BLACK)

        self.assertEqual(pattern.pattern_at(point(0, 0, 0)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 0, 0.99)), WHITE)
        self.assertEqual(pattern.pattern_at(point(0, 0, 1.01)), BLACK)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
