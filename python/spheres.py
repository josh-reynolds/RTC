# to run tests: python -m unittest -v spheres

import unittest
import math
import rays
import tuples
import intersections
import matrices
import transformations
import materials
import shapes

class Sphere(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        result = []
        sphere_to_ray = r.origin - tuples.point(0, 0, 0)

        a = r.direction.dot(r.direction)
        b = 2 * r.direction.dot(sphere_to_ray)
        c = sphere_to_ray.dot(sphere_to_ray) - 1

        discriminant = (b ** 2) - (4 * a * c)

        if discriminant >= 0:
            dsqrt = math.sqrt(discriminant)
            t1 = (-b - dsqrt) / (2 * a)
            t2 = (-b + dsqrt) / (2 * a)
            result.append(intersections.intersection(t1, self))
            result.append(intersections.intersection(t2, self))

        return result

    def local_normal_at(self, pt):
        return pt - tuples.point(0, 0, 0)

def sphere():
    return Sphere()

def glass_sphere():
    s = sphere()
    s.material.transparency = 1.0
    s.material.refractive_index = 1.5
    return s

class SphereTestCase(unittest.TestCase):
    def test_a_ray_intersects_a_sphere_at_two_points(self):
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
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
        r = rays.ray(tuples.point(0, 1, -5), tuples.vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 5.0)
        self.assertEqual(xs[1].t, 5.0)

    def test_a_ray_misses_a_sphere(self):
        r = rays.ray(tuples.point(0, 2, -5), tuples.vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_a_ray_originates_inside_a_sphere(self):
        r = rays.ray(tuples.point(0, 0, 0), tuples.vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, -1.0)
        self.assertEqual(xs[1].t, 1.0)

    def test_a_sphere_is_behind_a_ray(self):
        r = rays.ray(tuples.point(0, 0, 5), tuples.vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, -6.0)
        self.assertEqual(xs[1].t, -4.0)

    def test_intersect_sets_object_on_intersection(self):
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        s = sphere()

        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].object, s)
        self.assertEqual(xs[1].object, s)

    def test_a_spheres_default_transformation(self):
        s = sphere()

        self.assertEqual(s.transform, matrices.identity())

    def test_setting_a_spheres_transform(self):
        s = sphere()
        t = transformations.translation(2, 3, 4)

        s. set_transform(t)

        self.assertEqual(s.transform, t)

    def test_intersecting_scaled_sphere_with_ray(self):
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        s = sphere()
        s.set_transform(transformations.scaling(2, 2, 2))
        xs = s.intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 3)
        self.assertEqual(xs[1].t, 7)

    def test_intersecting_translated_sphere_with_ray(self):
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        s = sphere()
        s.set_transform(transformations.translation(5, 0, 0))
        xs = s.intersect(r)

        self.assertEqual(len(xs), 0)

    def test_normal_on_a_sphere_at_point_on_x_axis(self):
        s = sphere()
        n = s.normal_at(tuples.point(1, 0, 0))

        self.assertEqual(n, tuples.vector(1, 0, 0))

    def test_normal_on_a_sphere_at_point_on_y_axis(self):
        s = sphere()
        n = s.normal_at(tuples.point(0, 1, 0))

        self.assertEqual(n, tuples.vector(0, 1, 0))

    def test_normal_on_a_sphere_at_point_on_z_axis(self):
        s = sphere()
        n = s.normal_at(tuples.point(0, 0, 1))

        self.assertEqual(n, tuples.vector(0, 0, 1))

    def test_normal_on_a_sphere_at_nonaxial_point(self):
        s = sphere()
        n = s.normal_at(tuples.point(math.sqrt(3)/3, math.sqrt(3)/3, math.sqrt(3)/3))

        self.assertEqual(n, tuples.vector(math.sqrt(3)/3, math.sqrt(3)/3, math.sqrt(3)/3))

    def test_normal_is_normalized_vector(self):
        s = sphere()
        n = s.normal_at(tuples.point(math.sqrt(3)/3, math.sqrt(3)/3, math.sqrt(3)/3))

        self.assertEqual(n, n.normalize())

    def test_computing_normal_on_translated_sphere(self):
        s = sphere()
        s.set_transform(transformations.translation(0, 1, 0))
        n = s.normal_at(tuples.point(0, 1.70711, -0.70711))

        self.assertEqual(n, tuples.vector(0, 0.70711, -0.70711))

    def test_computing_normal_on_transformed_sphere(self):
        s = sphere()
        m = transformations.scaling(1, 0.5, 1) * transformations.rotation_z(math.pi/5)
        s.set_transform(m)
        n = s.normal_at(tuples.point(0, math.sqrt(2)/2, -math.sqrt(2)/2))

        self.assertEqual(n, tuples.vector(0, 0.97014, -0.24254))

    def test_a_sphere_has_a_default_material(self):
        s = sphere()
        m = materials.material()

        self.assertEqual(s.material, m)

    def test_a_sphere_may_be_assigned_a_material(self):
        s = sphere()
        m = materials.material()
        m.ambient = 1

        s.material = m

        self.assertEqual(s.material, m)

    def test_a_sphere_is_a_shape(self):
        s = sphere()

        self.assertTrue(isinstance(s, shapes.Shape))

    def test_glass_sphere_helper(self):
        s = glass_sphere()

        self.assertEqual(s.transform, matrices.identity())
        self.assertEqual(s.material.transparency, 1.0)
        self.assertEqual(s.material.refractive_index, 1.5)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
