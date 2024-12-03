# to run tests: python -m unittest -v world

import unittest
from tuple import point, vector
from color import color
from lights import point_light
from spheres import sphere
from materials import material
from transformations import scaling
from rays import ray

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
