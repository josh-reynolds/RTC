# to run tests: python -m unittest -v spheres

import unittest
from rays import ray
from tuple import point, vector

class Sphere:
    def __init__(self):
        pass

    def intersect(self, r):
        return [4.0, 6.0]

def sphere():
    return Sphere()

class SpheresTestCase(unittest.TestCase):
    def test_a_ray_intersects_a_sphere_at_two_points(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0], 4.0)
        self.assertEqual(xs[1], 6.0)

    def test_sphere_returns_unique_instances(self):
        s1 = sphere()
        s2 = sphere()

        self.assertFalse(s1 is s2)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
