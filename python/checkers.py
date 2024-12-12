# to run tests: python -m unittest -v checkers

import unittest
from colors import WHITE, BLACK
import spheres
import patterns

class Checker(patterns.Pattern):
    def __init__(self, color1, color2):
        self.a = color1
        self.b = color2
        patterns.Pattern.__init__(self)

    def pattern_at(self, pt):
        pass

def checker_pattern(color1, color2):
    return Checker(color1, color2)

class CheckerTestCase(unittest.TestCase):
    def test_creating_a_checker_pattern(self):
        pattern = checker_pattern(WHITE, BLACK)

        self.assertEqual(pattern.a, WHITE)
        self.assertEqual(pattern.b, BLACK)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
