# to run tests: python -m unittest -v planes

import unittest
from rays import ray
from tuples import point, vector
import intersections
import shapes
from utils import EPSILON

class Plane(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        result = []

        if abs(r.direction.y) >= EPSILON:
            t = -r.origin.y / r.direction.y
            result.append(intersections.intersection(t, self))

        return result

    def local_normal_at(self, pt):
        return vector(0, 1, 0)

def plane():
    return Plane()

class PlaneTestCase(unittest.TestCase):
    def test_normal_of_a_plane_is_constant_everywere(self):
        p = plane()
        n1 = p.local_normal_at(point(0, 0, 0))
        n2 = p.local_normal_at(point(10, 0, -10))
        n3 = p.local_normal_at(point(-5, 0, 150))

        self.assertEqual(n1, vector(0, 1, 0))
        self.assertEqual(n2, vector(0, 1, 0))
        self.assertEqual(n3, vector(0, 1, 0))

    def test_intersect_with_ray_parallel_to_plane(self):
        p = plane()
        r = ray(point(0, 10, 0), vector(0, 0, 1))

        xs = p.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_intersect_with_coplanar_ray(self):
        p = plane()
        r = ray(point(0, 0, 0), vector(0, 0, 1))

        xs = p.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_ray_intersecting_from_above(self):
        p = plane()
        r = ray(point(0, 1, 0), vector(0, -1, 0))

        xs = p.intersect(r)

        self.assertEqual(len(xs), 1)
        self.assertEqual(xs[0].t, 1)
        self.assertEqual(xs[0].object, p)

    def test_ray_intersecting_from_below(self):
        p = plane()
        r = ray(point(0, -1, 0), vector(0, 1, 0))

        xs = p.intersect(r)

        self.assertEqual(len(xs), 1)
        self.assertEqual(xs[0].t, 1)
        self.assertEqual(xs[0].object, p)

    def test_plane_returns_unique_instances(self):
        p1 = plane()
        p2 = plane()

        self.assertFalse(p1 is p2)

    def test_a_sphere_is_a_shape(self):
        p = plane()

        self.assertTrue(isinstance(p, shapes.Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
