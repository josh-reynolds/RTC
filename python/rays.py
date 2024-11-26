# to run tests: python -m unittest -v rays

import unittest
from tuple import point, vector

class Ray:
    def __init__(self, origin, direction):
        self.origin = origin
        self.direction = direction

def ray(origin, direction):
    return Ray(origin, direction)

class RaysTestCase(unittest.TestCase):
    def test_creating_and_querying_a_ray(self):
        origin = point(1, 2, 3)
        direction = vector(4, 5, 6)

        r = ray(origin, direction)

        self.assertEqual(r.origin, origin)
        self.assertEqual(r.direction, direction)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
