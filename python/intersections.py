# to run tests: python -m unittest -v intersections

import unittest
import spheres
from rays import ray
from tuples import point, vector
from transformations import translation
from utils import EPSILON

class Intersection:
    def __init__(self, t, obj):
        self.t = t
        self.object = obj

def intersection(t, obj):
    return Intersection(t, obj)

def intersections(*args):
    result = []
    for arg in args:
        result.append(arg)
    result.sort(key=lambda x: x.t)
    return result

def hit(xs):
    for i in xs:
        if i.t >=0:
            return i

class Computation:
    def __init__(self, intersect, ray):
        self.t = intersect.t
        self.object = intersect.object
        self.point = ray.position(self.t)
        self.eyev = -ray.direction
        self.normalv = self.object.normal_at(self.point)

        if self.normalv.dot(self.eyev) < 0:
            self.inside = True
            self.normalv = -self.normalv
        else:
            self.inside = False

        self.over_point = self.point + self.normalv * EPSILON

def prepare_computations(intersect, ray):
    return Computation(intersect, ray)

class IntersectionsTestCase(unittest.TestCase):
    def test_an_intersection_encapsulates_t_and_object(self):
        s = spheres.sphere()

        i = intersection(3.5, s)

        self.assertEqual(i.t, 3.5)
        self.assertEqual(i.object, s)

    def test_aggregating_intersections(self):
        s = spheres.sphere()
        i1 = intersection(1, s)
        i2 = intersection(2, s)

        xs = intersections(i1, i2)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 1)
        self.assertEqual(xs[1].t, 2)

    def test_the_hit_when_all_intersections_positive(self):
        s = spheres.sphere()
        i1 = intersection(1, s)
        i2 = intersection(2, s)
        xs = intersections(i2, i1)

        i = hit(xs)

        self.assertEqual(i, i1)
        
    def test_the_hit_when_some_intersections_negative(self):
        s = spheres.sphere()
        i1 = intersection(-1, s)
        i2 = intersection(1, s)
        xs = intersections(i2, i1)

        i = hit(xs)

        self.assertEqual(i, i2)
        
    def test_the_hit_when_all_intersections_negative(self):
        s = spheres.sphere()
        i1 = intersection(-2, s)
        i2 = intersection(-1, s)
        xs = intersections(i2, i1)

        i = hit(xs)

        self.assertEqual(i, None)
        
    def test_hit_is_always_lowest_nonnegative_intersection(self):
        s = spheres.sphere()
        i1 = intersection(5, s)
        i2 = intersection(7, s)
        i3 = intersection(-3, s)
        i4 = intersection(2, s)
        xs = intersections(i1, i2, i3, i4)

        i = hit(xs)

        self.assertEqual(i, i4)

    def test_precomputing_the_state_of_an_intersection(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = spheres.sphere()
        i = intersection(4, s)

        comps = prepare_computations(i, r)

        self.assertEqual(comps.t, i.t)
        self.assertEqual(comps.object, i.object)
        self.assertEqual(comps.point, point(0, 0, -1))
        self.assertEqual(comps.eyev, vector(0, 0, -1))
        self.assertEqual(comps.normalv, vector(0, 0, -1))

    def test_hit_when_intersection_occurs_on_outside(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = spheres.sphere()
        i = intersection(4, s)

        comps = prepare_computations(i, r)

        self.assertFalse(comps.inside)

    def test_hit_when_intersection_occurs_on_inside(self):
        r = ray(point(0, 0, 0), vector(0, 0, 1))
        s = spheres.sphere()
        i = intersection(1, s)

        comps = prepare_computations(i, r)

        self.assertEqual(comps.point, point(0, 0, 1))
        self.assertEqual(comps.eyev, vector(0, 0, -1))
        self.assertTrue(comps.inside)
        self.assertEqual(comps.normalv, vector(0, 0, -1))

    def test_hit_should_offset_point(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = spheres.sphere()
        s.transform = translation(0, 0, 1)
        i = intersection(5, s)

        comps = prepare_computations(i, r)

        self.assertTrue(comps.over_point.z < -EPSILON/2)
        self.assertTrue(comps.point.z > comps.over_point.z)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
