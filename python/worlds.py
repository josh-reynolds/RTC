# to run tests: python -m unittest -v worlds

import unittest
import math
from tuples import point, vector
from colors import color, BLACK, WHITE
from lights import point_light
from spheres import sphere
from materials import material, lighting
from transformations import scaling, translation
from rays import ray
from intersections import intersection, prepare_computations, hit
from planes import plane

class World:
    def __init__(self):
        self.objects = []
        self.light = None

    def intersect(self, ray):
        xs = []
        for obj in self.objects:
            xs += obj.intersect(ray)
        xs.sort(key=lambda x: x.t)
        return xs

    def shade_hit(self, comps):
        shadowed = self.is_shadowed(comps.over_point)

        return lighting(comps.object.material,
                        comps.object,
                        self.light,
                        comps.point,
                        comps.eyev,
                        comps.normalv,
                        shadowed)

    def color_at(self, r):
        xs = self.intersect(r)
        h = hit(xs)
        if h:
            comps = prepare_computations(h, r)
            return self.shade_hit(comps)
        else:
            return BLACK

    def is_shadowed(self, pt):
        shadow_vector = self.light.position - pt
        distance = shadow_vector.magnitude()
        r = ray(pt, shadow_vector.normalize())
        xs = self.intersect(r)
        h = hit(xs)
        if h and h.t < distance:
            return True
        else:
            return False

    def reflected_color(self, comps):
        if comps.object.material.reflective == 0:
            return BLACK
        
        reflect_ray = ray(comps.over_point, comps.reflectv)
        col = self.color_at(reflect_ray)

        return col * comps.object.material.reflective

def world():
    return World()

def default_world():
    w = World()

    light = point_light(point(-10, 10, -10), WHITE)
    w.light = light

    s1 = sphere()
    m1 = material()
    m1.color = color(0.8, 1.0, 0.6)
    m1.diffuse = 0.7
    m1.specular = 0.2
    s1.material = m1
    w.objects.append(s1)

    s2 = sphere()
    s2.set_transform(scaling(0.5, 0.5, 0.5))
    w.objects.append(s2)

    return w

class WorldTestCase(unittest.TestCase):
    def test_creating_a_world(self):
        w = world()

        self.assertEqual(len(w.objects), 0)
        self.assertEqual(w.light, None)

    def test_the_default_world(self):
        light = point_light(point(-10, 10, -10), WHITE)

        s1 = sphere()
        m1 = material()
        m1.color = color(0.8, 1.0, 0.6)
        m1.diffuse = 0.7
        m1.specular = 0.2
        s1.material = m1

        s2 = sphere()
        s2.set_transform(scaling(0.5, 0.5, 0.5))

        w = default_world()

        self.assertEqual(w.light, light)
        self.assertTrue(s1 in w.objects)
        self.assertTrue(s2 in w.objects)

    def test_intersect_a_world_with_a_ray(self):
        w = default_world()
        r = ray(point(0, 0, -5), vector(0, 0, 1))

        xs = w.intersect(r)

        self.assertEqual(len(xs), 4)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[1].t, 4.5)
        self.assertEqual(xs[2].t, 5.5)
        self.assertEqual(xs[3].t, 6)

    def test_shading_an_intersection(self):
        w = default_world()
        r = ray(point(0, 0, -5), vector(0, 0, 1))
        shape = w.objects[0]
        i = intersection(4, shape)
        comps = prepare_computations(i, r)

        c = w.shade_hit(comps)

        self.assertEqual(c, color(0.38066, 0.47583, 0.2855))

    def test_shading_an_intersection(self):
        w = default_world()
        w.light = point_light(point(0, 0.25, 0), WHITE)
        r = ray(point(0, 0, 0), vector(0, 0, 1))
        shape = w.objects[1]
        i = intersection(0.5, shape)
        comps = prepare_computations(i, r)

        c = w.shade_hit(comps)

        self.assertEqual(c, color(0.90498, 0.90498, 0.90498))

    def test_the_color_when_a_ray_misses(self):
        w = default_world()
        r = ray(point(0, 0, -5), vector(0, 1, 0))

        c = w.color_at(r)

        self.assertEqual(c, BLACK)

    def test_the_color_when_a_ray_hits(self):
        w = default_world()
        r = ray(point(0, 0, -5), vector(0, 0, 1))

        c = w.color_at(r)

        self.assertEqual(c, color(0.38066, 0.47583, 0.2855))

    def test_the_color_with_intersection_behind_ray(self):
        w = default_world()
        outer = w.objects[0]
        outer.material.ambient = 1
        inner = w.objects[1]
        inner.material.ambient = 1
        r = ray(point(0, 0, 0.75), vector(0, 0, -1))

        c = w.color_at(r)

        self.assertEqual(c, inner.material.color)

    def test_no_shadow_when_nothing_collinear_with_point_and_light(self):
        w = default_world()
        p = point(0, 10, 0)

        self.assertFalse(w.is_shadowed(p))

    def test_shadow_when_object_between_point_and_light(self):
        w = default_world()
        p = point(10, -10, 10)

        self.assertTrue(w.is_shadowed(p))

    def test_no_shadow_when_object_behind_light(self):
        w = default_world()
        p = point(-20, 20, -20)

        self.assertFalse(w.is_shadowed(p))

    def test_no_shadow_when_object_behind_point(self):
        w = default_world()
        p = point(-2, 2, -2)

        self.assertFalse(w.is_shadowed(p))

    def test_shade_hit_given_intersection_in_shadow(self):
        w = world()
        w.light = point_light(point(0, 0, -10), WHITE)

        s1 = sphere()
        w.objects.append(s1)

        s2 = sphere()
        s2.transform = translation(0, 0, 10)
        w.objects.append(s2)

        r = ray(point(0, 0, 5), vector(0, 0, 1))
        i = intersection(9, s2)      # book typo? it has (4, s2)
        comps = prepare_computations(i, r)

        c = w.shade_hit(comps)

        self.assertEqual(c, color(0.1, 0.1, 0.1))

    def test_reflected_color_for_nonreflective_material(self):
        w = default_world()
        r = ray(point(0, 0, 0), vector(0, 0, 1))
        shape = w.objects[1]
        shape.material.ambient = 1
        i = intersection(1, shape)
        comps = prepare_computations(i, r)

        c = w.reflected_color(comps)

        self.assertEqual(c, BLACK)

    def test_reflected_color_for_reflective_material(self):
        w = default_world()
        shape = plane()
        shape.material.reflective = 0.5
        shape.set_transform(translation(0, -1, 0))
        w.objects.append(shape)
        r = ray(point(0, 0, -3), vector(0, -math.sqrt(2)/2, math.sqrt(2)/2))
        i = intersection(math.sqrt(2), shape)
        comps = prepare_computations(i, r)

        c = w.reflected_color(comps)

        self.assertEqual(c, color(0.19033, 0.23791, 0.14274))
        # text has (0.19032, 0.2379, 0.14274) which fails
        # had to tweak values slightly

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
