# to run tests: python -m unittest -v intersections

import unittest
from spheres import sphere

class Intersection:
    def __init__(self, t, obj):
        self.t = t
        self.object = obj

def intersection(t, obj):
    return Intersection(t, obj)

class IntersectionsTestCase(unittest.TestCase):
    def test_an_intersection_encapsulates_t_and_object(self):
        s = sphere()

        i = intersection(3.5, s)

        self.assertEqual(i.t, 3.5)
        self.assertEqual(i.object, s)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
