# to run tests: python -m unittest -v cylinders

import unittest
import materials
import shapes
from rays import ray
from tuples import point, vector
from utils import flequal
from intersections import intersection

class Cylinder(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        a = r.direction.x ** 2 + r.direction.z ** 2
        if flequal(a, 0):
            return []

        b = (2 * r.origin.x * r.direction.x +
             2 * r.origin.z * r.direction.z)

        c = r.origin.x ** 2 + r.origin.z ** 2 - 1
        
        disc = b ** 2 - 4 * a * c

        if disc < 0:
            return []

        return [intersection(1, self)]

    def local_normal_at(self, pt):
        pass

def cylinder():
    return Cylinder()

class CylinderTestCase(unittest.TestCase):
    def test_a_cylinder_is_a_shape(self):
        c = cylinder()

        self.assertTrue(isinstance(c, shapes.Shape))

    def test_a_ray_misses_a_cylinder(self):
        c = cylinder()

        r = ray(point(1, 0, 0), vector(0, 1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 0, 0), vector(0, 1, 0).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

        r = ray(point(0, 0, -5), vector(1, 1, 1).normalize())
        xs = c.local_intersect(r)
        self.assertEqual(len(xs), 0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
