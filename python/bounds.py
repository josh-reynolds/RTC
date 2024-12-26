# to run tests: python -m unittest -v bounds

import unittest
import materials
import shapes
import tuples
import spheres
import cubes

class Bounds(shapes.Shape):
    def __init__(self, minimum, maximum):
        shapes.Shape.__init__(self)
        self.minimum = minimum
        self.maximum = maximum

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt):
        pass

def bounds(shape):
    minimum, maximum = shape.bounds()
    return Bounds(minimum, maximum)

class BoundsTestCase(unittest.TestCase):
    def test_bounds_is_a_shape(self):
        b = bounds(spheres.sphere())

        self.assertTrue(isinstance(b, shapes.Shape))

    def test_bounds_calculated_from_a_sphere(self):
        b = bounds(spheres.sphere())
        
        self.assertEqual(b.minimum.x, -1)
        self.assertEqual(b.minimum.y, -1)
        self.assertEqual(b.minimum.z, -1)
        self.assertEqual(b.maximum.x, 1)
        self.assertEqual(b.maximum.y, 1)
        self.assertEqual(b.maximum.z, 1)

    def test_bounds_calculated_from_a_cube(self):
        b = bounds(cubes.cube())

        self.assertEqual(b.minimum.x, -1)
        self.assertEqual(b.minimum.y, -1)
        self.assertEqual(b.minimum.z, -1)
        self.assertEqual(b.maximum.x, 1)
        self.assertEqual(b.maximum.y, 1)
        self.assertEqual(b.maximum.z, 1)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
