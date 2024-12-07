# to run tests: python -m unittest -v shapes

import unittest
import math
from rays import ray
from tuple import point, vector
from matrix import identity
from transformations import translation, scaling, rotation_z
from materials import material

class Shape:                                          # 'abstract' base class
    def __init__(self):
        self.transform = identity()
        self.material = material()

    def intersect(self, r):
        local_ray = r.transform(self.transform.inverse())
        return self.local_intersect(local_ray)

    def local_intersect(self, r):                     # override in child classes
        self.saved_ray = r                            # this implementation for test purposes only
    
    def set_transform(self, t):
        self.transform = t

    def normal_at(self, pt):
        local_point = self.transform.inverse() * pt
        local_normal = self.local_normal_at(local_point)
        world_normal = self.transform.inverse().transpose() * local_normal
        world_normal.w = 0
        return world_normal.normalize()

    def local_normal_at(self, pt):                    # override in child classes
        return vector(pt.x, pt.y, pt.z)               # this implementation for test purposes only

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

    def test_computing_normal_on_translated_shape(self):
        s = test_shape()
        s.set_transform(translation(0, 1, 0))

        n = s.normal_at(point(0, 1.70711, -0.70711))

        self.assertEqual(n, vector(0, 0.70711, -0.70711))

    def test_computing_normal_on_transformed_shape(self):
        s = test_shape()
        m = scaling(1, 0.5, 1) * rotation_z(math.pi/5)
        s.set_transform(m)

        n = s.normal_at(point(0, math.sqrt(2)/2, -math.sqrt(2)/2))

        self.assertEqual(n, vector(0, 0.97014, -0.24254))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
