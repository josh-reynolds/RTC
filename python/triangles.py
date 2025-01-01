# to run tests: python -m unittest -v triangles

import unittest
import materials
import shapes
import tuples
import rays
import utils
import intersections

class Triangle(shapes.Shape):
    def __init__(self, p1, p2, p3):
        shapes.Shape.__init__(self)
        self.p1 = p1
        self.p2 = p2
        self.p3 = p3
        self.e1 = p2 - p1
        self.e2 = p3 - p1
        self.normal = (self.e2.cross(self.e1)).normalize()

    def local_intersect(self, r):
        dir_cross_e2 = r.direction.cross(self.e2)
        determinant = self.e1.dot(dir_cross_e2)
        if abs(determinant) < utils.EPSILON:
            return []

        f = 1.0 / determinant

        p1_to_origin = r.origin - self.p1
        u = f * p1_to_origin.dot(dir_cross_e2)
        if u < 0 or u > 1:
            return []

        return [intersections.intersection(1, self)]

    def local_normal_at(self, pt):
        return self.normal

    #def bounds(self):
        #return (tuples.point(-1, -1, -1),
                #tuples.point(1, 1, 1))

def triangle(p1, p2, p3):
    return Triangle(p1, p2, p3)

class TriangleTestCase(unittest.TestCase):
    def test_a_triangle_is_a_shape(self):
        t = triangle(tuples.point( 0, 1, 0),
                     tuples.point(-1, 0, 0),
                     tuples.point( 1, 0, 0))

        self.assertTrue(isinstance(t, shapes.Shape))

    def test_constructing_a_triangle(self):
        p1 = tuples.point( 0, 1, 0)
        p2 = tuples.point(-1, 0, 0)
        p3 = tuples.point( 1, 0, 0)

        t = triangle(p1, p2, p3)

        self.assertEqual(t.p1, p1)
        self.assertEqual(t.p2, p2)
        self.assertEqual(t.p3, p3)
        self.assertEqual(t.e1, tuples.vector(-1, -1, 0))
        self.assertEqual(t.e2, tuples.vector(1, -1, 0))
        self.assertEqual(t.normal, tuples.vector(0, 0, -1))

    def test_finding_the_normal_on_a_triangle(self):
        t = triangle(tuples.point( 0, 1, 0),
                     tuples.point(-1, 0, 0),
                     tuples.point( 1, 0, 0))

        n1 = t.local_normal_at(tuples.point(   0,  0.5, 0))
        n2 = t.local_normal_at(tuples.point(-0.5, 0.75, 0))
        n3 = t.local_normal_at(tuples.point( 0.5, 0.25, 0))

        self.assertEqual(n1, t.normal)
        self.assertEqual(n2, t.normal)
        self.assertEqual(n3, t.normal)

    def test_intersecting_ray_parallel_to_triangle(self):
        t = triangle(tuples.point( 0, 1, 0),
                     tuples.point(-1, 0, 0),
                     tuples.point( 1, 0, 0))
        r = rays.ray(tuples.point(0, -1, -2),
                     tuples.vector(0, 1, 0))

        xs = t.local_intersect(r)

        self.assertEqual(len(xs), 0)

    def test_a_ray_misses_p1_p3_edge(self):
        t = triangle(tuples.point( 0, 1, 0),
                     tuples.point(-1, 0, 0),
                     tuples.point( 1, 0, 0))
        r = rays.ray(tuples.point(1, 1, -2),
                     tuples.vector(0, 0, 1))

        xs = t.local_intersect(r)

        self.assertEqual(len(xs), 0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
