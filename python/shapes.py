# to run tests: python -m unittest -v shapes

import unittest
#import math
from rays import ray
from tuple import point, vector
#import intersections
from matrix import identity
from transformations import translation, scaling
from materials import material

class Shape:
    def __init__(self):
        self.transform = identity()
        self.material = material()

    #def __eq__(self, other):
        #if isinstance(other, self.__class__):
            #return (self.transform == other.transform and
                    #self.material == other.material)
        #else:
            #return False
#
    def intersect(self, r):
        local_ray = r.transform(self.transform.inverse())
        return self.local_intersect(local_ray)

    def local_intersect(self, r):
        self.saved_ray = r
    
    def set_transform(self, t):
        self.transform = t

    #def normal_at(self, pt):
        #object_point = self.transform.inverse() * pt
        #object_normal = object_point - point(0, 0, 0)
        #world_normal = self.transform.inverse().transpose() * object_normal
        #world_normal.w = 0
        #return world_normal.normalize()

def test_shape():
    return Shape()

class ShapeTestCase(unittest.TestCase):
    def test_a_shapes_default_transformation(self):
        s = test_shape()

        self.assertEqual(s.transform, identity())

    def test_assigning_a_transformation(self):
        s = test_shape()

        s.set_transform(translation(2, 3, 4))

        self.assertEqual(s.transform, translation(2, 3, 4))

    def test_a_shapes_default_material(self):
        s = test_shape()

        self.assertEqual(s.material, material())

    def test_assigning_a_material(self):
        s = test_shape()
        m = material()
        m.ambient = 1

        s.material = m

        self.assertEqual(s.material, m)

    def test_intersecting_a_scaled_shape_with_a_ray(self):
        s = test_shape()
        s.set_transform(scaling(2, 2, 2))
        r = ray(point(0, 0, -5), vector(0, 0, 1))

        xs = s.intersect(r)

        self.assertEqual(s.saved_ray.origin, point(0, 0, -2.5))
        self.assertEqual(s.saved_ray.direction, vector(0, 0, 0.5))

    def test_intersecting_a_translated_shape_with_a_ray(self):
        s = test_shape()
        s.set_transform(translation(5, 0, 0))
        r = ray(point(0, 0, -5), vector(0, 0, 1))

        xs = s.intersect(r)

        self.assertEqual(s.saved_ray.origin, point(-5, 0, -5))
        self.assertEqual(s.saved_ray.direction, vector(0, 0, 1))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
