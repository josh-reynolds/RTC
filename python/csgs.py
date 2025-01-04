# to run tests: python -m unittest -v csgs

import unittest
import materials
import shapes
import spheres
import cubes

class CSG(shapes.Shape):
    def __init__(self, op, shape1, shape2):
        shapes.Shape.__init__(self)
        self.operation = op
        
        shape1.parent = self
        self.left = shape1

        shape2.parent = self
        self.right = shape2

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt, i):
        pass

    def bounds(self):
        pass

def csg(op, shape1, shape2):
    return CSG(op, shape1, shape2)

def intersection_allowed(op, lhit, inl, inr):
    if op == "union":
        return (lhit and not inr) or (not lhit and not inl)
    return False

class CSGTestCase(unittest.TestCase):
    def test_a_csg_is_a_shape(self):
        c = csg("union", spheres.sphere(), spheres.sphere())

        self.assertTrue(isinstance(c, shapes.Shape))

    def test_csg_is_created_with_operation_and_two_shapes(self):
        s1 = spheres.sphere()
        s2 = cubes.cube()
        c = csg("union", s1, s2)

        self.assertEqual(c.operation, "union")
        self.assertEqual(c.left, s1)
        self.assertEqual(c.right, s2)
        self.assertEqual(s1.parent, c)
        self.assertEqual(s2.parent, c)

    def test_evaluating_rule_for_union_operation(self):
        result = intersection_allowed("union", True, True, True)
        self.assertEqual(result, False)

        result = intersection_allowed("union", True, True, False)
        self.assertEqual(result, True)

        result = intersection_allowed("union", True, False, True)
        self.assertEqual(result, False)

        result = intersection_allowed("union", True, False, False)
        self.assertEqual(result, True)

        result = intersection_allowed("union", False, True, True)
        self.assertEqual(result, False)

        result = intersection_allowed("union", False, True, False)
        self.assertEqual(result, False)

        result = intersection_allowed("union", False, False, True)
        self.assertEqual(result, True)

        result = intersection_allowed("union", False, False, False)
        self.assertEqual(result, True)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
