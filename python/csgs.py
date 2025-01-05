# to run tests: python -m unittest -v csgs

import unittest
import materials
import shapes
import spheres
import cubes
import intersections

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

    def filter_intersections(self, xs):
        inl = False
        inr = False
        result = []

        for i in xs:
            lhit = True     # dummy value until we implement Shape.includes()
            #lhit = csg.left includes i.object

            if intersection_allowed(self.operation, lhit, inl, inr):
                result.append(i)

            if lhit:
                inl = not inl
            else:
                inr = not inr

        return result

def csg(op, shape1, shape2):
    return CSG(op, shape1, shape2)

def intersection_allowed(op, lhit, inl, inr):
    if op == "union":
        return (lhit and not inr) or (not lhit and not inl)
    elif op == "intersect":
        return (lhit and inr) or (not lhit and inl)
    elif op == "difference":
        return (lhit and not inr) or (not lhit and inl)
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

    def test_evaluating_rule_for_intersect_operation(self):
        result = intersection_allowed("intersect", True, True, True)
        self.assertEqual(result, True)

        result = intersection_allowed("intersect", True, True, False)
        self.assertEqual(result, False)

        result = intersection_allowed("intersect", True, False, True)
        self.assertEqual(result, True)

        result = intersection_allowed("intersect", True, False, False)
        self.assertEqual(result, False)

        result = intersection_allowed("intersect", False, True, True)
        self.assertEqual(result, True)

        result = intersection_allowed("intersect", False, True, False)
        self.assertEqual(result, True)

        result = intersection_allowed("intersect", False, False, True)
        self.assertEqual(result, False)

        result = intersection_allowed("intersect", False, False, False)
        self.assertEqual(result, False)

    def test_evaluating_rule_for_difference_operation(self):
        result = intersection_allowed("difference", True, True, True)
        self.assertEqual(result, False)

        result = intersection_allowed("difference", True, True, False)
        self.assertEqual(result, True)

        result = intersection_allowed("difference", True, False, True)
        self.assertEqual(result, False)

        result = intersection_allowed("difference", True, False, False)
        self.assertEqual(result, True)

        result = intersection_allowed("difference", False, True, True)
        self.assertEqual(result, True)

        result = intersection_allowed("difference", False, True, False)
        self.assertEqual(result, True)

        result = intersection_allowed("difference", False, False, True)
        self.assertEqual(result, False)

        result = intersection_allowed("difference", False, False, False)
        self.assertEqual(result, False)

    def test_filtering_a_set_of_intersections(self):
        s1 = spheres.sphere()
        s2 = cubes.cube()

        xs = intersections.intersections(intersections.intersection(1, s1),
                                         intersections.intersection(2, s2),
                                         intersections.intersection(3, s1),
                                         intersections.intersection(4, s2))

        c = csg("union", s1, s2)
        result = c.filter_intersections(xs)
        self.assertEqual(len(result), 2)
        self.assertEqual(result[0], xs[0])
        self.assertEqual(result[1], xs[3])

        c = csg("intersection", s1, s2)
        result = c.filter_intersections(xs)
        self.assertEqual(len(result), 2)
        self.assertEqual(result[0], xs[1])
        self.assertEqual(result[1], xs[2])

        c = csg("difference", s1, s2)
        result = c.filter_intersections(xs)
        self.assertEqual(len(result), 2)
        self.assertEqual(result[0], xs[0])
        self.assertEqual(result[1], xs[1])

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
