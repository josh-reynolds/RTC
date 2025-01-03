# to run tests: python -m unittest -v shapes

import unittest
import math
import matrices
import materials
import tuples


class Shape:                                          # 'abstract' base class
    def __init__(self):
        self.transform = matrices.identity()
        self.material = materials.material()
        self.parent = None

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return (self.transform == other.transform and
                    self.material == other.material)
        else:
            return False

    def intersect(self, r):
        local_ray = r.transform(self.transform.inverse())
        return self.local_intersect(local_ray)

    def local_intersect(self, r):                     # override in child classes
        self.saved_ray = r                            # this implementation for test purposes only
    
    def set_transform(self, t):
        self.transform = t

    def normal_at(self, pt, i):                       # i is only used by select subclasses for uv data
        local_point = self.world_to_object(pt)
        local_normal = self.local_normal_at(local_point, i)
        return self.normal_to_world(local_normal)

    def local_normal_at(self, pt, i):                    # override in child classes
        return tuples.vector(pt.x, pt.y, pt.z)        # this implementation for test purposes only

    def world_to_object(self, pt):
        if self.parent:
            pt = self.parent.world_to_object(pt)

        return self.transform.inverse() * pt

    def normal_to_world(self, normal):
        normal = self.transform.inverse().transpose() * normal
        normal.w = 0
        normal = normal.normalize()

        if self.parent:
            normal = self.parent.normal_to_world(normal)

        return normal


def test_shape():
    return Shape()

import rays
import transformations
import spheres
import groups
import intersections

class ShapeTestCase(unittest.TestCase):
    def test_a_shapes_default_transformation(self):
        s = test_shape()

        self.assertEqual(s.transform, matrices.identity())

    def test_assigning_a_transformation(self):
        s = test_shape()

        s.set_transform(transformations.translation(2, 3, 4))

        self.assertEqual(s.transform, transformations.translation(2, 3, 4))

    def test_a_shapes_default_materials(self):
        s = test_shape()

        self.assertEqual(s.material, materials.material())

    def test_assigning_a_materials(self):
        s = test_shape()
        m = materials.material()
        m.ambient = 1

        s.material = m

        self.assertEqual(s.material, m)

    def test_intersecting_a_scaled_shape_with_a_ray(self):
        s = test_shape()
        s.set_transform(transformations.scaling(2, 2, 2))
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))

        xs = s.intersect(r)

        self.assertEqual(s.saved_ray.origin, tuples.point(0, 0, -2.5))
        self.assertEqual(s.saved_ray.direction, tuples.vector(0, 0, 0.5))

    def test_intersecting_a_translated_shape_with_a_ray(self):
        s = test_shape()
        s.set_transform(transformations.translation(5, 0, 0))
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))

        xs = s.intersect(r)

        self.assertEqual(s.saved_ray.origin, tuples.point(-5, 0, -5))
        self.assertEqual(s.saved_ray.direction, tuples.vector(0, 0, 1))

    def test_computing_normal_on_translated_shape(self):
        s = test_shape()
        s.set_transform(transformations.translation(0, 1, 0))
        i = intersections.intersection(1, s)

        n = s.normal_at(tuples.point(0, 1.70711, -0.70711), i)

        self.assertEqual(n, tuples.vector(0, 0.70711, -0.70711))

    def test_computing_normal_on_transformed_shape(self):
        s = test_shape()
        m = transformations.scaling(1, 0.5, 1) * transformations.rotation_z(math.pi/5)
        s.set_transform(m)
        i = intersections.intersection(1, s)

        n = s.normal_at(tuples.point(0, math.sqrt(2)/2, -math.sqrt(2)/2), i)

        self.assertEqual(n, tuples.vector(0, 0.97014, -0.24254))

    def test_a_shape_has_a_parent_attribute(self):
        s = test_shape()

        self.assertEqual(s.parent, None)

    def test_converting_point_from_world_to_object_space(self):
        g1 = groups.group()
        g1.set_transform(transformations.rotation_y(math.pi/2))

        g2 = groups.group()
        g2.set_transform(transformations.scaling(2, 2, 2))
        g1.add_child(g2)

        s = spheres.sphere()
        s.set_transform(transformations.translation(5, 0, 0))
        g2.add_child(s)

        p = s.world_to_object(tuples.point(-2, 0, -10))

        self.assertEqual(p, tuples.point(0, 0, -1))

    def test_converting_a_normal_from_object_to_world_space(self):
        g1 = groups.group()
        g1.set_transform(transformations.rotation_y(math.pi/2))

        g2 = groups.group()
        g2.set_transform(transformations.scaling(1, 2, 3))
        g1.add_child(g2)

        s = spheres.sphere()
        s.set_transform(transformations.translation(5, 0, 0))
        g2.add_child(s)

        n = s.normal_to_world(tuples.vector(math.sqrt(3)/3, math.sqrt(3)/3, math.sqrt(3)/3))

        self.assertEqual(n, tuples.vector(0.28571, 0.42857, -0.85714))

    def test_finding_normal_on_a_child_object(self):
        g1 = groups.group()
        g1.set_transform(transformations.rotation_y(math.pi/2))

        g2 = groups.group()
        g2.set_transform(transformations.scaling(1, 2, 3))
        g1.add_child(g2)

        s = spheres.sphere()
        s.set_transform(transformations.translation(5, 0, 0))
        g2.add_child(s)

        i = intersections.intersection(1, s)

        n = s.normal_at(tuples.point(1.7321, 1.1547, -5.5774), i)

        self.assertEqual(n, tuples.vector(0.28570, 0.42854, -0.85716))


# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
