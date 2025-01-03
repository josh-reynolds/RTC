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

    def __eq__(self, other):                            # fields are defined in concrete classes
        if isinstance(other, self.__class__):           # should we move up?
            return (self.a == other.a and               # initially didn't want to limit this abstract
                    self.b == other.b)                  # class to exactly two colors
        else:
            return False

    def set_transform(self, transform):
        self.transform = transform

    def pattern_at_shape(self, obj, world_pt):
        obj_pt = obj.world_to_object(world_pt)
        return self.pattern_at(obj_pt)

    def pattern_at(self, pt):                           # override in child classes
        pat_pt = self.transform.inverse() * pt
        return color(pat_pt.x, pat_pt.y, pat_pt.z)      # this implementation for test purposes only

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
