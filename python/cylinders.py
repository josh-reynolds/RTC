# to run tests: python -m unittest -v cylinders

import math
import unittest
import materials
import shapes
from rays import ray
from tuples import point, vector
from utils import flequal
from intersections import intersection

class Cylinder(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)
        self.minimum = -math.inf
        self.maximum = math.inf
        self.closed = False

    def local_intersect(self, r):
        xs = []

        a = r.direction.x ** 2 + r.direction.z ** 2
        if not flequal(a, 0):
            b = (2 * r.origin.x * r.direction.x +
                 2 * r.origin.z * r.direction.z)
    
            c = r.origin.x ** 2 + r.origin.z ** 2 - 1
            
            disc = b ** 2 - 4 * a * c
            if disc < 0:
                return []
    
            t0 = (-b - math.sqrt(disc)) / (2 * a)
            t1 = (-b + math.sqrt(disc)) / (2 * a)
            if t0 > t1:
                t0, t1 = t1, t0
    
    
            y0 = r.origin.y + t0 * r.direction.y
            if self.minimum < y0 and y0 < self.maximum:
                xs.append(intersection(t0, self))
    
            y1 = r.origin.y + t1 * r.direction.y
            if self.minimum < y1 and y1 < self.maximum:
                xs.append(intersection(t1, self))

        self.intersect_caps(r, xs)

        return xs

    def intersect_caps(self, r, xs):
        if not self.closed or flequal(r.direction.y, 0):
            return

        t = (self.minimum - r.origin.y) / r.direction.y
        if check_cap(r, t):
            xs.append(intersection(t, self))

        t = (self.maximum - r.origin.y) / r.direction.y
        if check_cap(r, t):
            xs.append(intersection(t, self))

    def local_normal_at(self, pt):
        return vector(pt.x, 0, pt.z)

def cylinder():
    return Cylinder()

def check_cap(r, t):
    x = r.origin.x + t * r.direction.x
    z = r.origin.z + t * r.direction.z
    return (x ** 2 + z ** 2) <= 1

class CylinderTestCase(unittest.TestCase):
    def test_a_cylinder_is_a_shape(self):
        c = cylinder()

        self.assertTrue(isinstance(c, shapes.Shape))

    def test_a_ray_misses_a_cylinder(self):
        c = cylinder()

        r = ray(point(1, 0, 0), vector(0, 1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 0, 0), vector(0, 1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 0, -5), vector(1, 1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

    def test_a_ray_strikes_a_cylinder(self):
        c = cylinder()

        r = ray(point(1, 0, -5), vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 5)
        self.assertEqual(xs[1].t, 5)

        r = ray(point(0, 0, -5), vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 6)

        r = ray(point(0.5, 0, -5), vector(0.1, 1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertTrue(flequal(xs[0].t, 6.80798))
        self.assertTrue(flequal(xs[1].t, 7.08872))

    def test_normal_vector_on_a_cylinder(self):
        c = cylinder()

        n = c.local_normal_at(point(1, 0, 0))
        self.assertEqual(n, vector(1, 0, 0))

        n = c.local_normal_at(point(0, 5, -1))
        self.assertEqual(n, vector(0, 0, -1))

        n = c.local_normal_at(point(0, -2, 1))
        self.assertEqual(n, vector(0, 0, 1))

        n = c.local_normal_at(point(-1, 1, 0))
        self.assertEqual(n, vector(-1, 0, 0))

    def test_default_min_max_for_a_cylinder(self):
        c = cylinder()

        self.assertEqual(c.minimum, -math.inf)
        self.assertEqual(c.maximum, math.inf)

    def test_intersecting_a_constrained_cylinder(self):
        c = cylinder()
        c.minimum = 1
        c.maximum = 2

        r = ray(point(0, 1.5, 0), vector(0.1, 1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 3, -5), vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 0, -5), vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 2, -5), vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 1, -5), vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 1.5, -2), vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)

    def test_default_closed_value_for_a_cylinder(self):
        c = cylinder()

        self.assertFalse(c.closed)

    def test_intersecting_caps_of_closed_cylinder(self):
        c = cylinder()
        c.minimum = 1
        c.maximum = 2
        c.closed = True

        r = ray(point(0, 3, 0), vector(0, -1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)

        r = ray(point(0, 3, -2), vector(0, -1, 2).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)

        r = ray(point(0, 4, -2), vector(0, -1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)

        r = ray(point(0, 0, -2), vector(0, 1, 2).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)

        r = ray(point(0, -1, -2), vector(0, 1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
