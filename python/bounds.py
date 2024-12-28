# to run tests: python -m unittest -v bounds

import math
import unittest
import materials
import shapes
import tuples
import spheres
import cubes
import planes
import cylinders
import cones
import groups
import transformations
import utils

class Bounds(shapes.Shape):
    def __init__(self, minimum, maximum):
        shapes.Shape.__init__(self)
        self.minimum = minimum
        self.maximum = maximum

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt):
        pass

    def __repr__(self):
        return "Bounds({}, {})".format(self.minimum, self.maximum)

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

    def test_bounds_calculated_from_a_plane(self):
        b = bounds(planes.plane())

        self.assertEqual(b.minimum.x, -math.inf)
        self.assertEqual(b.minimum.y, -utils.EPSILON)
        self.assertEqual(b.minimum.z, -math.inf)
        self.assertEqual(b.maximum.x, math.inf)
        self.assertEqual(b.maximum.y, utils.EPSILON)
        self.assertEqual(b.maximum.z, math.inf)

    def test_bounds_calculated_from_a_cylinder(self):
        b = bounds(cylinders.cylinder())

        self.assertEqual(b.minimum.x, -1)
        self.assertEqual(b.minimum.y, -math.inf)
        self.assertEqual(b.minimum.z, -1)
        self.assertEqual(b.maximum.x, 1)
        self.assertEqual(b.maximum.y, math.inf)
        self.assertEqual(b.maximum.z, 1)

    def test_bounds_calculated_from_a_constrained_cylinder(self):
        c = cylinders.cylinder()
        c.minimum = -1
        c.maximum = 1
        b = bounds(c)

        self.assertEqual(b.minimum.x, -1)
        self.assertEqual(b.minimum.y, -1)
        self.assertEqual(b.minimum.z, -1)
        self.assertEqual(b.maximum.x, 1)
        self.assertEqual(b.maximum.y, 1)
        self.assertEqual(b.maximum.z, 1)

    def test_bounds_calculated_from_a_cone(self):
        b = bounds(cones.cone())

        self.assertEqual(b.minimum.x, -math.inf)
        self.assertEqual(b.minimum.y, -math.inf)
        self.assertEqual(b.minimum.z, -math.inf)
        self.assertEqual(b.maximum.x, math.inf)
        self.assertEqual(b.maximum.y, math.inf)
        self.assertEqual(b.maximum.z, math.inf)

    def test_bounds_calculated_from_a_constrained_cone(self):
        c = cones.cone()
        c.minimum = -1
        c.maximum = 2
        b = bounds(c)

        self.assertEqual(b.minimum.x, -2)
        self.assertEqual(b.minimum.y, -1)
        self.assertEqual(b.minimum.z, -2)
        self.assertEqual(b.maximum.x, 2)
        self.assertEqual(b.maximum.y, 2)
        self.assertEqual(b.maximum.z, 2)

    def test_bounds_from_a_one_item_group(self):
        g = groups.group()
        g.add_child(spheres.sphere())
        b = bounds (g)

        self.assertEqual(b.minimum.x, -1)
        self.assertEqual(b.minimum.y, -1)
        self.assertEqual(b.minimum.z, -1)
        self.assertEqual(b.maximum.x, 1)
        self.assertEqual(b.maximum.y, 1)
        self.assertEqual(b.maximum.z, 1)

    def test_bounds_from_an_empty_group(self):
        g = groups.group()
        b = bounds (g)

        self.assertEqual(b.minimum.x, 0)
        self.assertEqual(b.minimum.y, 0)
        self.assertEqual(b.minimum.z, 0)
        self.assertEqual(b.maximum.x, 0)
        self.assertEqual(b.maximum.y, 0)
        self.assertEqual(b.maximum.z, 0)

    def test_bounds_with_group_containing_transformed_shape(self):
        s = spheres.sphere()
        s.set_transform(transformations.scaling(2, 2, 2))
        g = groups.group()
        g.add_child(s)
        b = bounds(g)

        self.assertEqual(b.minimum.x, -2)
        self.assertEqual(b.minimum.y, -2)
        self.assertEqual(b.minimum.z, -2)
        self.assertEqual(b.maximum.x, 2)
        self.assertEqual(b.maximum.y, 2)
        self.assertEqual(b.maximum.z, 2)

    def test_bounds_with_group_containing_multiple_shapes(self):
        g = groups.group()
        s1 = spheres.sphere()
        s1.set_transform(transformations.translation(-1, 0, 0))
        g.add_child(s1)
        s2 = spheres.sphere()
        s2.set_transform(transformations.translation(1, 0, 0))
        g.add_child(s2)
        b = bounds(g)

        self.assertEqual(b.minimum.x, -2)
        self.assertEqual(b.minimum.y, -1)
        self.assertEqual(b.minimum.z, -1)
        self.assertEqual(b.maximum.x, 2)
        self.assertEqual(b.maximum.y, 1)
        self.assertEqual(b.maximum.z, 1)

    def test_bounds_with_translated_shape(self):
        g = groups.group()
        s = spheres.sphere()
        s.set_transform(transformations.translation(3, 3, 3))
        g.add_child(s)
        b = bounds(g)

        self.assertEqual(b.minimum.x, 2)
        self.assertEqual(b.minimum.y, 2)
        self.assertEqual(b.minimum.z, 2)
        self.assertEqual(b.maximum.x, 4)
        self.assertEqual(b.maximum.y, 4)
        self.assertEqual(b.maximum.z, 4)
        
# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
