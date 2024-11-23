# to run tests: python -m unittest -v transformations

import unittest
from tuple import point
from matrix import identity

def translation(dx, dy, dz):
    return identity()


class TransformationsTestCase(unittest.TestCase):
    def test_multiplying_by_a_translation_matrix(self):
        transform = translation(5, -3, 2)
        p = point(-3, 4, 5)

        self.assertEqual(transform * p, point(2, 1, 7))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
