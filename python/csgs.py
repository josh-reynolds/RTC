# to run tests: python -m unittest -v csgs

import unittest
import materials
import shapes
import spheres
import cubes
import intersections
import rays
import tuples
import transformations
import groups

class CSG(shapes.Shape):
    def __init__(self, op, shape1, shape2):
        shapes.Shape.__init__(self)
        self.operation = op
        
        shape1.parent = self
        self.left = shape1

        shape2.parent = self
        self.right = shape2

    def __contains__(self, other):
        return (other in self.left or
                other in self.right)

    def local_intersect(self, r):
        leftxs = self.left.intersect(r)
        rightxs = self.right.intersect(r)
        xs = leftxs + rightxs
        xs.sort(key=lambda x: x.t)
        return self.filter_intersections(xs)

    def local_normal_at(self, pt, i):
        pass

    def bounds(self):
        minimum = maximum = None
        xs = []
        ys = []
        zs = []

        for shape in (self.left, self.right):
            newmin,newmax = shape.bounds()

            points = groups.cube_points(newmin, newmax)
            for p in points:
                newp = shape.transform * p
                xs.append(newp.x)
                ys.append(newp.y)
                zs.append(newp.z)

        minimum = tuples.point(min(xs), min(ys), min(zs))
        maximum = tuples.point(max(xs), max(ys), max(zs))

        return (minimum, maximum)

    def filter_intersections(self, xs):
        inl = False
        inr = False
        result = []

        for i in xs:
            lhit = i.object in self.left

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

        c = csg("intersect", s1, s2)
        result = c.filter_intersections(xs)
        self.assertEqual(len(result), 2)
        self.assertEqual(result[0], xs[1])
        self.assertEqual(result[1], xs[2])

        c = csg("difference", s1, s2)
        result = c.filter_intersections(xs)
        self.assertEqual(len(result), 2)
        self.assertEqual(result[0], xs[0])
        self.assertEqual(result[1], xs[1])

    def test_ray_misses_a_csg(self):
        c = csg("union", spheres.sphere(), cubes.cube())
        r = rays.ray(tuples.point(0, 2, -5), tuples.vector(0, 0, 1))
        xs = c.local_intersect(r)

        self.assertEqual(len(xs), 0)

    def test_ray_hits_a_csg(self):
        s1 = spheres.sphere()
        s2 = spheres.sphere()
        s2.set_transform(transformations.translation(0, 0, 0.5))
        c = csg("union", s1, s2)
        r = rays.ray(tuples.point(0, 0, -5), tuples.vector(0, 0, 1))
        xs = c.local_intersect(r)

        self.assertEqual(len(xs), 2)
        self.assertEqual(xs[0].t, 4)
        self.assertEqual(xs[0].object, s1)
        self.assertEqual(xs[1].t, 6.5)
        self.assertEqual(xs[1].object, s2)

    def test_calculating_bounds_on_a_csg(self):
        s1 = spheres.sphere()
        s1.set_transform(transformations.translation(0, -1, 0))
        s2 = spheres.sphere()
        s2.set_transform(transformations.translation(1, 0, 0))
        c = csg("union", s1, s2)

        b = c.bounds()

        self.assertEqual(b[0].x, -1)
        self.assertEqual(b[0].y, -2)
        self.assertEqual(b[0].z, -1)
        self.assertEqual(b[1].x, 2)
        self.assertEqual(b[1].y, 1)
        self.assertEqual(b[1].z, 1)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
