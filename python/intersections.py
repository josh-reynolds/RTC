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
    return result

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
