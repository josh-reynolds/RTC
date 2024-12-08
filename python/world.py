# to run tests: python -m unittest -v world

import unittest
from tuple import point, vector
from color import color
from lights import point_light
from spheres import sphere
from materials import material, lighting
from transformations import scaling, translation
from rays import ray
from intersections import intersection, prepare_computations, hit

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
            return color(0, 0, 0)

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

def world():
    return World()

def default_world():
    w = World()

    light = point_light(point(-10, 10, -10), color(1, 1, 1))
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
        light = point_light(point(-10, 10, -10), color(1, 1, 1))

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
        w.light = point_light(point(0, 0.25, 0), color(1, 1, 1))
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

        self.assertEqual(c, color(0, 0, 0))

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
        w.light = point_light(point(0, 0, -10), color(1, 1, 1))

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
