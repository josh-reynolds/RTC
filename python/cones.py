# to run tests: python -m unittest -v cones

import math
import unittest
import materials
import shapes
from rays import ray
import tuples
from intersections import intersection
from utils import flequal, EPSILON

class Cone(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)
        self.minimum = -math.inf
        self.maximum = math.inf
        self.closed = False

    def local_intersect(self, r):
        xs = []

        a = (r.direction.x ** 2 - 
             r.direction.y ** 2 + 
             r.direction.z ** 2)

        b = (2 * r.origin.x * r.direction.x -
             2 * r.origin.y * r.direction.y +
             2 * r.origin.z * r.direction.z)
    
        c = (r.origin.x ** 2 - 
             r.origin.y ** 2 +
             r.origin.z ** 2)

        if flequal(a, 0):
            if not flequal(b, 0):
                t = -c / (2 * b)
                xs.append(intersection(t, self))
        else:
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
        pass
        if not self.closed or flequal(r.direction.y, 0):
            return

        t = (self.minimum - r.origin.y) / r.direction.y
        if check_cap(r, t, abs(self.minimum)):
            xs.append(intersection(t, self))

        t = (self.maximum - r.origin.y) / r.direction.y
        if check_cap(r, t, abs(self.maximum)):
            xs.append(intersection(t, self))

    def local_normal_at(self, pt):
        dist = pt.x ** 2 + pt.z ** 2

        if dist < 1 and pt.y >= self.maximum - EPSILON:
            return tuples.vector(0, 1, 0)

        if dist < 1 and pt.y <= self.minimum + EPSILON:
            return tuples.vector(0, -1, 0)

        y = math.sqrt(dist)
        if pt.y > 0:
            y = -y

        return tuples.vector(pt.x, y, pt.z)

    def bounds(self):
        max_radius = max(abs(self.minimum), abs(self.maximum))
        return (tuples.point(-max_radius, self.minimum, -max_radius),
                tuples.point(max_radius, self.maximum, max_radius))

def cone():
    return Cone()

def check_cap(r, t, radius):
    x = r.origin.x + t * r.direction.x
    z = r.origin.z + t * r.direction.z
    return (x ** 2 + z ** 2) <= radius

class ConeTestCase(unittest.TestCase):
    def test_a_cone_is_a_shape(self):
        c = cone()

        self.assertTrue(isinstance(c, shapes.Shape))

    def test_intersecting_a_ray_with_a_cone(self):
        c = cone()

        r = ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 5)
        self.assertEqual(xs[1].t, 5)

        r = ray(tuples.point(0, 0, -5), tuples.vector(1, 1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertTrue(flequal(xs[0].t, 8.66025))
        self.assertTrue(flequal(xs[1].t, 8.66025))

        r = ray(tuples.point(1, 1, -5), tuples.vector(-0.5, -1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)
        self.assertTrue(flequal(xs[0].t,  4.55006))
        self.assertTrue(flequal(xs[1].t, 49.44994))

    def test_intersecting_cone_with_ray_parallel_to_one_half(self):
        c = cone()

        r = ray(tuples.point(0, 0, -1), tuples.vector(0, 1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 1)
        self.assertTrue(flequal(xs[0].t, 0.35355))

    def test_default_min_max_for_a_cone(self):
        c = cone()

        self.assertEqual(c.minimum, -math.inf)
        self.assertEqual(c.maximum, math.inf)
        
    def test_default_closed_value_for_a_cone(self):
        c = cone()

        self.assertFalse(c.closed)
        
    def test_intersecting_caps_of_closed_cone(self):
        c = cone()
        c.minimum = -0.5
        c.maximum = 0.5
        c.closed = True

        r = ray(tuples.point(0, 0, -5), tuples.vector(0, 1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(tuples.point(0, 0, -0.25), tuples.vector(0, 1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 2)

        r = ray(tuples.point(0, 0, -0.25), tuples.vector(0, 1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 4)

    def test_normal_vector_on_a_cone(self):
        c = cone()

        n = c.local_normal_at(tuples.point(0, 0, 0))
        self.assertEqual(n, tuples.vector(0, 0, 0))

        n = c.local_normal_at(tuples.point(1, 1, 1))
        self.assertEqual(n, tuples.vector(1, -math.sqrt(2), 1))

        n = c.local_normal_at(tuples.point(-1, -1, 0))
        self.assertEqual(n, tuples.vector(-1, 1, 0))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
