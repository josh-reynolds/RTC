# to run tests: python -m unittest -v checkers

import unittest
import math
from colors import WHITE, BLACK, color
import spheres
import patterns
from tuples import point

class Checker(patterns.Pattern):
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        if (math.floor(pt.x) + math.floor(pt.y) + math.floor(pt.z)) % 2 == 0:
            return self.a
        else:
            return self.b

def checker_pattern(color1, color2):
    return Checker(color1, color2)

class CheckerTestCase(unittest.TestCase):
    def test_creating_a_checker_pattern(self):
        pattern = checker_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, WHITE)
        self.assertEqual(pattern.b, BLACK)

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
