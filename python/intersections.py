# to run tests: python -m unittest -v intersections

import unittest
import spheres

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
        
# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
