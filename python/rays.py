# to run tests: python -m unittest -v rays

import unittest
from tuple import point, vector

class Ray:
    def __init__(self, origin, direction):
        self.origin = origin
        self.direction = direction

    def position(self, t):
        return self.origin + self.direction * t

def ray(origin, direction):
    return Ray(origin, direction)

class RaysTestCase(unittest.TestCase):
    def test_creating_and_querying_a_ray(self):
        origin = point(1, 2, 3)
        direction = vector(4, 5, 6)

        r = ray(origin, direction)

        self.assertEqual(r.origin, origin)
        self.assertEqual(r.direction, direction)

    def test_computing_a_point_from_a_distance(self):
        r = ray(point(2, 3, 4), vector(1, 0, 0))

        self.assertEqual(r.position(0), point(2, 3, 4))
        self.assertEqual(r.position(1), point(3, 3, 4))
        self.assertEqual(r.position(-1), point(1, 3, 4))
        self.assertEqual(r.position(2.5), point(4.5, 3, 4))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
