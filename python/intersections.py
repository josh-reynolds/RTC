# to run tests: python -m unittest -v intersections

import unittest
import math
import spheres
import rays
import tuples
import transformations
import utils
import planes
import triangles

class Intersection:
    def __init__(self, t, obj):
        self.t = t
        self.object = obj

    def __repr__(self):
        return "Intersection(%r, %r)" % (self.t, self.object)

def intersection(t, obj):
    return Intersection(t, obj)

def intersection_with_uv(t, obj, u, v):
    i = Intersection(t, obj)
    i.u = u
    i.v = v
    return i

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

def schlick(comps):
    cos = comps.eyev.dot(comps.normalv)

    if comps.n1 > comps.n2:
        n = comps.n1 / comps.n2
        sin2_t = n ** 2 * (1.0 - cos ** 2)
        if sin2_t > 1.0:
            return 1.0

        cos_t = math.sqrt(1.0 - sin2_t)
        cos = cos_t

    r0 = ((comps.n1 - comps.n2)/(comps.n1 + comps.n2)) ** 2
    return r0 + (1 - r0) * (1 - cos) ** 5

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

        self.over_point = self.point + self.normalv * utils.EPSILON
        self.under_point = self.point - self.normalv * utils.EPSILON
        self.reflectv = ray.direction.reflect(self.normalv)
        self.n1 = 1.0
        self.n2 = 1.0

def prepare_computations(intersect, ray, xs=None):
    comp = Computation(intersect, ray)

    if xs:
        containers = []
        for i in xs:
            if i == intersect:
                if containers:
                    comp.n1 = containers[-1].material.refractive_index

            if i.object in containers:
                containers.remove(i.object)
            else:
                containers.append(i.object)

            if i == intersect:
                if containers:
                    comp.n2 = containers[-1].material.refractive_index
                break

    return comp

class IntersectionTestCase(unittest.TestCase):
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
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        s = spheres.sphere()
        i = intersection(4, s)

        comps = prepare_computations(i, r)

        self.assertEqual(comps.t, i.t)
        self.assertEqual(comps.object, i.object)
        self.assertEqual(comps.point, tuples.point(0, 0, -1))
        self.assertEqual(comps.eyev, tuples.vector(0, 0, -1))
        self.assertEqual(comps.normalv, tuples.vector(0, 0, -1))

    def test_hit_when_intersection_occurs_on_outside(self):
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        s = spheres.sphere()
        i = intersection(4, s)

        comps = prepare_computations(i, r)

        self.assertFalse(comps.inside)

    def test_hit_when_intersection_occurs_on_inside(self):
        r = rays.ray(tuples.point(0, 0, 0), tuples.vector(0, 0, 1))
        s = spheres.sphere()
        i = intersection(1, s)

        comps = prepare_computations(i, r)

        self.assertEqual(comps.point, tuples.point(0, 0, 1))
        self.assertEqual(comps.eyev, tuples.vector(0, 0, -1))
        self.assertTrue(comps.inside)
        self.assertEqual(comps.normalv, tuples.vector(0, 0, -1))

    def test_hit_should_offset_point(self):
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        s = spheres.sphere()
        s.transform = transformations.translation(0, 0, 1)
        i = intersection(5, s)

        comps = prepare_computations(i, r)

        self.assertTrue(comps.over_point.z < -utils.EPSILON/2)
        self.assertTrue(comps.point.z > comps.over_point.z)

    def test_precomputing_reflection_vector(self):
        shape = planes.plane()
        r = rays.ray(tuples.point(0, 1, -1), tuples.vector(0, -math.sqrt(2)/2, math.sqrt(2)/2))
        i = intersection(math.sqrt(2), shape)

        comps = prepare_computations(i, r)

        self.assertEqual(comps.reflectv, tuples.vector(0, math.sqrt(2)/2, math.sqrt(2)/2))

    def test_finding_n1_and_n2_at_various_intersections(self):
        a = spheres.glass_sphere()
        a.set_transform(transformations.scaling(2, 2, 2))
        a.material.refractive_index = 1.5

        b = spheres.glass_sphere()
        b.set_transform(transformations.translation(0, 0, -0.25))
        b.material.refractive_index = 2.0

        c = spheres.glass_sphere()
        c.set_transform(transformations.translation(0, 0, 0.25))
        c.material.refractive_index = 2.5

        r = rays.ray(tuples.point(0, 0, -4), tuples.vector(0, 0, 1))
        xs = intersections(intersection(2, a),
                           intersection(2.75, b),
                           intersection(3.25, c),
                           intersection(4.75, b),
                           intersection(5.25, c),
                           intersection(6, a))
        
        comps = prepare_computations(xs[0], r, xs)
        self.assertEqual(comps.n1, 1.0)
        self.assertEqual(comps.n2, 1.5)
        
        comps = prepare_computations(xs[1], r, xs)
        self.assertEqual(comps.n1, 1.5)
        self.assertEqual(comps.n2, 2.0)

        comps = prepare_computations(xs[2], r, xs)
        self.assertEqual(comps.n1, 2.0)
        self.assertEqual(comps.n2, 2.5)

        comps = prepare_computations(xs[3], r, xs)
        self.assertEqual(comps.n1, 2.5)
        self.assertEqual(comps.n2, 2.5)

        comps = prepare_computations(xs[4], r, xs)
        self.assertEqual(comps.n1, 2.5)
        self.assertEqual(comps.n2, 1.5)

        comps = prepare_computations(xs[5], r, xs)
        self.assertEqual(comps.n1, 1.5)
        self.assertEqual(comps.n2, 1.0)

    def test_under_point_is_offset_below_surface(self):
        shape = spheres.glass_sphere()
        shape.set_transform(transformations.translation(0, 0, 1))

        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        xs = intersections(intersection(5, shape))
        comps = prepare_computations(xs[0], r, xs)

        self.assertTrue(comps.under_point.z > utils.EPSILON/2)
        self.assertTrue(comps.point.z < comps.under_point.z)

    def test_schlick_approximation_under_total_internal_reflection(self):
        shape = spheres.glass_sphere()

        r = rays.ray(tuples.point(0, 0, math.sqrt(2)/2), tuples.vector(0, 1, 0))
        xs = intersections(intersection(-math.sqrt(2)/2, shape),
                           intersection( math.sqrt(2)/2, shape))
        comps = prepare_computations(xs[1], r, xs)

        reflectance = schlick(comps)

        self.assertEqual(reflectance, 1.0)

    def test_schlick_approximation_with_perpendicular_viewing_angle(self):
        shape = spheres.glass_sphere()

        r = rays.ray(tuples.point(0, 0, 0), tuples.vector(0, 1, 0))
        xs = intersections(intersection(-1, shape),
                           intersection( 1, shape))
        comps = prepare_computations(xs[1], r, xs)

        reflectance = schlick(comps)

        self.assertTrue(utils.flequal(reflectance, 0.04))

    def test_schlick_approximation_with_small_angle_and_n2_gt_n1(self):
        shape = spheres.glass_sphere()

        r = rays.ray(tuples.point(0, 0.99, -2), tuples.vector(0, 0, 1))
        xs = intersections(intersection(1.8589, shape))
        comps = prepare_computations(xs[0], r, xs)

        reflectance = schlick(comps)

        self.assertTrue(utils.flequal(reflectance, 0.48874))

    def test_an_intersection_can_encapsulate_u_and_v(self):
        s = triangles.triangle(tuples.point( 0, 1, 0),
                               tuples.point(-1, 0, 0),
                               tuples.point( 1, 0, 0))

        i = intersection_with_uv(3.5, s, 0.2, 0.4)

        self.assertEqual(i.u, 0.2)
        self.assertEqual(i.v, 0.4)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
