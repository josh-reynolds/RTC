# to run tests: python -m unittest -v spheres

import unittest
import math
from rays import ray
from tuple import point, vector
import intersections
from matrix import identity
from transformations import translation, scaling, rotation_z

class Sphere:
    def __init__(self):
        self.transform = identity()

    def intersect(self, r):
        result = []
        r2 = r.transform(self.transform.inverse())
        sphere_to_ray = r2.origin - point(0, 0, 0)

        a = r2.direction.dot(r2.direction)
        b = 2 * r2.direction.dot(sphere_to_ray)
        c = sphere_to_ray.dot(sphere_to_ray) - 1

        discriminant = (b ** 2) - (4 * a * c)

        if discriminant >= 0:
            dsqrt = math.sqrt(discriminant)
            t1 = (-b - dsqrt) / (2 * a)
            t2 = (-b + dsqrt) / (2 * a)
            result.append(intersections.intersection(t1, self))
            result.append(intersections.intersection(t2, self))

        return result
    
    def set_transform(self, t):
        self.transform = t

    def normal_at(self, pt):
        object_point = self.transform.inverse() * pt
        object_normal = object_point - point(0, 0, 0)
        world_normal = self.transform.inverse().transpose() * object_normal
        world_normal.w = 0
        return world_normal.normalize()

def sphere():
    return Sphere()

class SpheresTestCase(unittest.TestCase):
    def test_a_ray_intersects_a_sphere_at_two_points(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4.0)
        self.assertEqual(xs[1].t, 6.0)

    def test_sphere_returns_unique_instances(self):
        s1 = sphere()
        s2 = sphere()

        self.assertFalse(s1 is s2)

    def test_a_ray_intersects_a_sphere_at_a_tangent(self):
        r = ray(point(0, 1, -5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 5.0)
        self.assertEqual(xs[1].t, 5.0)

    def test_a_ray_misses_a_sphere(self):
        r = ray(point(0, 2, -5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_a_ray_originates_inside_a_sphere(self):
        r = ray(point(0, 0, 0), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, -1.0)
        self.assertEqual(xs[1].t, 1.0)

    def test_a_sphere_is_behind_a_ray(self):
        r = ray(point(0, 0, 5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, -6.0)
        self.assertEqual(xs[1].t, -4.0)

    def test_intersect_sets_object_on_intersection(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].object, s)
        self.assertEqual(xs[1].object, s)

    def test_a_spheres_default_transformation(self):
        s = sphere()

        self.assertEqual(s.transform, identity())

    def test_setting_a_spheres_transform(self):
        s = sphere()
        t = translation(2, 3, 4)

        s. set_transform(t)

        self.assertEqual(s.transform, t)

    def test_intersecting_scaled_sphere_with_ray(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = sphere()
        s.set_transform(scaling(2, 2, 2))
        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 3)
        self.assertEqual(xs[1].t, 7)

    def test_intersecting_translated_sphere_with_ray(self):
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        s = sphere()
        s.set_transform(translation(5, 0, 0))
        xs = s.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_normal_on_a_sphere_at_point_on_x_axis(self):
        s = sphere()
        n = s.normal_at(point(1, 0, 0))

        self.assertEqual(n, vector(1, 0, 0))

    def test_normal_on_a_sphere_at_point_on_y_axis(self):
        s = sphere()
        n = s.normal_at(point(0, 1, 0))

        self.assertEqual(n, vector(0, 1, 0))

    def test_normal_on_a_sphere_at_point_on_z_axis(self):
        s = sphere()
        n = s.normal_at(point(0, 0, 1))

        self.assertEqual(n, vector(0, 0, 1))

    def test_normal_on_a_sphere_at_nonaxial_point(self):
        s = sphere()
        n = s.normal_at(point(math.sqrt(3)/3, math.sqrt(3)/3, math.sqrt(3)/3))

        self.assertEqual(n, vector(math.sqrt(3)/3, math.sqrt(3)/3, math.sqrt(3)/3))

    def test_normal_is_normalized_vector(self):
        s = sphere()
        n = s.normal_at(point(math.sqrt(3)/3, math.sqrt(3)/3, math.sqrt(3)/3))

        self.assertEqual(n, n.normalize())

    def test_computing_normal_on_translated_sphere(self):
        s = sphere()
        s.set_transform(translation(0, 1, 0))
        n = s.normal_at(point(0, 1.70711, -0.70711))

        self.assertEqual(n, vector(0, 0.70711, -0.70711))

    def test_computing_normal_on_transformed_sphere(self):
        s = sphere()
        m = scaling(1, 0.5, 1) * rotation_z(math.pi/5)
        s.set_transform(m)
        n = s.normal_at(point(0, math.sqrt(2)/2, -math.sqrt(2)/2))

        self.assertEqual(n, vector(0, 0.97014, -0.24254))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
