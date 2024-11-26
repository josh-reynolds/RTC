# to run tests: python -m unittest -v spheres

import unittest
import math
from rays import ray
from tuple import point, vector

class Sphere:
    def __init__(self):
        pass

    def intersect(self, r):
        result = []
        sphere_to_ray = r.origin - point(0, 0, 0)

        a = r.direction.dot(r.direction)
        b = 2 * r.direction.dot(sphere_to_ray)
        c = sphere_to_ray.dot(sphere_to_ray) - 1

        discriminant = (b ** 2) - (4 * a * c)

        if discriminant >= 0:
            dsqrt = math.sqrt(discriminant)
            t1 = (-b - dsqrt) / (2 * a)
            t2 = (-b + dsqrt) / (2 * a)
            result.append(t1)
            result.append(t2)

        return result

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

    def test_a_ray_intersects_a_sphere_at_a_tangent(self):
        r = ray(point(0, 1, -5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0], 5.0)
        self.assertEqual(xs[1], 5.0)

    def test_a_ray_misses_a_sphere(self):
        r = ray(point(0, 2, -5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_a_ray_originates_inside_a_sphere(self):
        r = ray(point(0, 0, 0), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0], -1.0)
        self.assertEqual(xs[1], 1.0)

    def test_a_sphere_is_behind_a_ray(self):
        r = ray(point(0, 0, 5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0], -6.0)
        self.assertEqual(xs[1], -4.0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
