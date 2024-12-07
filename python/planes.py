# to run tests: python -m unittest -v planes

import unittest
#import math
from rays import ray
from tuple import point, vector
#import intersections
#from matrix import identity
#from transformations import translation, scaling, rotation_z
#from materials import material
from shapes import Shape
from utils import EPSILON

class Plane(Shape):
    def __init__(self):
        Shape.__init__(self)

    def local_intersect(self, r):
        result = []

        if abs(r.direction.y) < EPSILON:
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

    def test_plane_returns_unique_instances(self):
        p1 = plane()
        p2 = plane()

        self.assertFalse(p1 is p2)

    def test_a_sphere_is_a_shape(self):
        p = plane()

        self.assertTrue(isinstance(p, Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
