# to run tests: python -m unittest -v cubes

import math
import unittest
import materials
import shapes
from rays import ray
from tuples import point, vector
from intersections import intersections, intersection
from utils import EPSILON

class Cube(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        xtmin, xtmax = check_axis(r.origin.x, r.direction.x)
        ytmin, ytmax = check_axis(r.origin.y, r.direction.y)
        ztmin, ztmax = check_axis(r.origin.z, r.direction.z)

        tmin = max(xtmin, ytmin, ztmin)
        tmax = min(xtmax, ytmax, ztmax)

        if tmin > tmax:
            return []

        return intersections(intersection(tmin, self),
                             intersection(tmax, self))

    def local_normal_at(self, pt):
        maxc = max(abs(pt.x), abs(pt.y), abs(pt.z))

        if maxc == abs(pt.x):
            return vector(pt.x, 0, 0)
        elif maxc == abs(pt.y):
            return vector(0, pt.y, 0)

        return vector(0, 0, pt.z)

def cube():
    return Cube()

def check_axis(origin, direction):
    tmin_numerator = (-1 - origin)
    tmax_numerator = (1 - origin)

    if abs(direction) >= EPSILON:
        tmin = tmin_numerator / direction
        tmax = tmax_numerator / direction
    else:
        tmin = tmin_numerator * math.inf
        tmax = tmax_numerator * math.inf

    if tmin > tmax:
        tmin, tmax = tmax, tmin

    return (tmin, tmax)

class CubeTestCase(unittest.TestCase):
    def test_a_cube_is_a_shape(self):
        c = cube()

        self.assertTrue(isinstance(c, shapes.Shape))

    def test_a_ray_intersects_a_cube(self):
        c = cube()

        r = ray(point(5, 0.5, 0), vector(-1, 0, 0))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 6)

        r = ray(point(-5, 0.5, 0), vector(1, 0, 0))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 6)

        r = ray(point(0.5, 5, 0), vector(0, -1, 0))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 6)

        r = ray(point(0.5, -5, 0), vector(0, 1, 0))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 6)

        r = ray(point(0.5, 0, 5), vector(0, 0, -1))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 6)

        r = ray(point(0.5, 0, -5), vector(0, 0, 1))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 6)

        r = ray(point(0, 0.5, 0), vector(0, 0, 1))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, -1)
        self.assertEqual(xs[1].t, 1)

    def test_a_ray_misses_a_cube(self):
        c = cube()

        r = ray(point(-2, 0, 0), vector(0.2673, 0.5345, 0.8018))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, -2, 0), vector(0.8018, 0.2673, 0.5345))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 0, -2), vector(0.5345, 0.8018, 0.2673))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(2, 0, 2), vector(0, 0, -1))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 2, 2), vector(0, -1, 0))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(2, 2, 0), vector(-1, 0, 0))
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

    def test_normal_on_surface_of_a_cube(self):
        c = cube()

        p = point(1, 0.5, -0.8)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(1, 0, 0))

        p = point(-1, -0.2, 0.9)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(-1, 0, 0))

        p = point(-0.4, 1, -0.1)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(0, 1, 0))

        p = point(0.3, -1, -0.7)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(0, -1, 0))

        p = point(-0.6, 0.3, 1)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(0, 0, 1))

        p = point(0.4, 0.4, -1)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(0, 0, -1))

        p = point(1, 1, 1)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(1, 0, 0))

        p = point(-1, -1, -1)
        normal = c.local_normal_at(p)
        self.assertEqual(normal,vector(-1, 0, 0))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
