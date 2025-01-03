# to run tests: python -m unittest -v planes

import math
import unittest
import rays
import tuples
import intersections
import shapes
import utils

class Plane(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        result = []

        if abs(r.direction.y) >= utils.EPSILON:
            t = -r.origin.y / r.direction.y
            result.append(intersections.intersection(t, self))

        return result

    def local_normal_at(self, pt, i):
        return tuples.vector(0, 1, 0)
    
    def bounds(self):
        return (tuples.point(-math.inf, -utils.EPSILON, -math.inf),
                tuples.point(math.inf, utils.EPSILON, math.inf))

def plane():
    return Plane()

class PlaneTestCase(unittest.TestCase):
    def test_normal_of_a_plane_is_constant_everywere(self):
        p = plane()
        i = intersections.intersection(1, p)
        n1 = p.local_normal_at(tuples.point(0, 0, 0), i)
        n2 = p.local_normal_at(tuples.point(10, 0, -10), i)
        n3 = p.local_normal_at(tuples.point(-5, 0, 150), i)

        self.assertEqual(n1, tuples.vector(0, 1, 0))
        self.assertEqual(n2, tuples.vector(0, 1, 0))
        self.assertEqual(n3, tuples.vector(0, 1, 0))

    def test_intersect_with_ray_parallel_to_plane(self):
        p = plane()
        r = rays.ray(tuples.point(0, 10, 0), tuples.vector(0, 0, 1))

        xs = p.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_intersect_with_coplanar_ray(self):
        p = plane()
        r = rays.ray(tuples.point(0, 0, 0), tuples.vector(0, 0, 1))

        xs = p.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_ray_intersecting_from_above(self):
        p = plane()
        r = rays.ray(tuples.point(0, 1, 0), tuples.vector(0, -1, 0))

        xs = p.intersect(r)

        self.assertEqual(len(xs), 1)
        self.assertEqual(xs[0].t, 1)
        self.assertEqual(xs[0].object, p)

    def test_ray_intersecting_from_below(self):
        p = plane()
        r = rays.ray(tuples.point(0, -1, 0), tuples.vector(0, 1, 0))

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
