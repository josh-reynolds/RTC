# to run tests: python -m unittest -v patterns

import unittest
import math
from colors import color, WHITE, BLACK
from tuples import point
import spheres
from transformations import scaling, translation
from matrices import identity

class Pattern():                                        # 'abstract' base class
    def __init__(self):
        self.transform = identity()

    def set_transform(self, transform):
        self.transform = transform

    def pattern_at_shape(self, obj, world_pt):
        obj_pt = obj.transform.inverse() * world_pt
        pat_pt = self.transform.inverse() * obj_pt

        return self.pattern_at(pat_pt)

    def pattern_at(self, pt):                           # override in child classes
        return color(pt.x, pt.y, pt.z)                  # this implementation for test purposes only

def test_pattern():
    return Pattern()

class PatternTestCase(unittest.TestCase):
    def test_default_pattern_transformation(self):
        pattern = test_pattern()

        self.assertEqual(pattern.transform, identity())

    def test_assigning_a_pattern_transformation(self):
        pattern = test_pattern()
        pattern.set_transform(translation(1, 2, 3))

        self.assertEqual(pattern.transform, translation(1, 2, 3))

    def test_pattern_with_an_object_transformation(self):
        obj = spheres.sphere()
        obj.set_transform(scaling(2, 2, 2))
        pattern = test_pattern()

        c = pattern.pattern_at_shape(obj, point(2, 3, 4))

        self.assertEqual(c, color(1, 1.5, 2))

    def test_pattern_with_a_pattern_transformation(self):
        obj = spheres.sphere()
        pattern = test_pattern()
        pattern.set_transform(scaling(2, 2, 2))

        c = pattern.pattern_at_shape(obj, point(2, 3, 4))

        self.assertEqual(c, color(1, 1.5, 2))

    def test_stripes_with_both_pattern_and_object_transformations(self):
        obj = spheres.sphere()
        obj.set_transform(scaling(2, 2, 2))
        pattern = test_pattern()
        pattern.set_transform(translation(0.5, 1, 1.5))

        c = pattern.pattern_at_shape(obj, point(2.5, 3, 3.5))

        self.assertEqual(c, color(0.75, 0.5, 0.25))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
