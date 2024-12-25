# to run tests: python -m unittest -v rays

import unittest
import tuples
import transformations

class Ray:
    def __init__(self, origin, direction):
        self.origin = origin
        self.direction = direction

    def position(self, t):
        return self.origin + self.direction * t

    def transform(self, matrix):
        origin = matrix * self.origin
        direction = matrix * self.direction
        return ray(origin, direction)

def ray(origin, direction):
    return Ray(origin, direction)

class RayTestCase(unittest.TestCase):
    def test_creating_and_querying_a_ray(self):
        origin = tuples.point(1, 2, 3)
        direction = tuples.vector(4, 5, 6)

        r = ray(origin, direction)

        self.assertEqual(r.origin, origin)
        self.assertEqual(r.direction, direction)

    def test_computing_a_point_from_a_distance(self):
        r = ray(tuples.point(2, 3, 4), tuples.vector(1, 0, 0))

        self.assertEqual(r.position(0), tuples.point(2, 3, 4))
        self.assertEqual(r.position(1), tuples.point(3, 3, 4))
        self.assertEqual(r.position(-1), tuples.point(1, 3, 4))
        self.assertEqual(r.position(2.5), tuples.point(4.5, 3, 4))

    def test_translating_a_ray(self):
        r = ray(tuples.point(1, 2, 3), tuples.vector(0, 1, 0))
        m = transformations.translation(3, 4, 5)

        r2 = r.transform(m)

        self.assertEqual(r2.origin, tuples.point(4, 6, 8))
        self.assertEqual(r2.direction, tuples.vector(0, 1, 0))
        
    def test_scaling_a_ray(self):
        r = ray(tuples.point(1, 2, 3), tuples.vector(0, 1, 0))
        m = transformations.scaling(2, 3, 4)

        r2 = r.transform(m)

        self.assertEqual(r2.origin, tuples.point(2, 6, 12))
        self.assertEqual(r2.direction, tuples.vector(0, 3, 0))
        
# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
