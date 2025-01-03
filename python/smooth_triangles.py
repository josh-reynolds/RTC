# to run tests: python -m unittest -v smooth_triangles

import unittest
import materials
import triangles
import shapes
import tuples
#import rays
#import utils
#import intersections

class SmoothTriangle(triangles.Triangle):
    def __init__(self, p1, p2, p3, n1, n2, n3):
        triangles.Triangle.__init__(self, p1, p2, p3)
        self.n1 = n1
        self.n2 = n2
        self.n3 = n3

    #def local_intersect(self, r):
        #dir_cross_e2 = r.direction.cross(self.e2)
        #determinant = self.e1.dot(dir_cross_e2)
        #if abs(determinant) < utils.EPSILON:
            #return []
#
        #f = 1.0 / determinant
#
        #p1_to_origin = r.origin - self.p1
        #u = f * p1_to_origin.dot(dir_cross_e2)
        #if u < 0 or u > 1:
            #return []
#
        #origin_cross_e1 = p1_to_origin.cross(self.e1)
        #v = f * r.direction.dot(origin_cross_e1)
        #if v < 0 or (u + v) > 1:
            #return []
#
        #t = f * self.e2.dot(origin_cross_e1)
        #return [intersections.intersection(t, self)]

    #def local_normal_at(self, pt):
        #return self.normal

    #def bounds(self):
        #xs = [self.p1.x, self.p2.x, self.p3.x]
        #ys = [self.p1.y, self.p2.y, self.p3.y]
        #zs = [self.p1.z, self.p2.z, self.p3.z]
#
        #return (tuples.point(min(xs), min(ys), min(zs)),
                #tuples.point(max(xs), max(ys), max(zs)))

def smooth_triangle(p1, p2, p3, n1, n2, n3):
    return SmoothTriangle(p1, p2, p3, n1, n2, n3)

class SmoothTriangleTestCase(unittest.TestCase):
    def setUp(self):
        p1 = tuples.point( 0, 1, 0)
        p2 = tuples.point(-1, 0, 0)
        p3 = tuples.point( 1, 0, 0)
        n1 = tuples.vector(0, 1, 0)
        n2 = tuples.vector(-1, 0, 0)
        n3 = tuples.vector(1, 0, 0)

        self.tri = smooth_triangle(p1, p2, p3, n1, n2, n3)

    def test_a_smooth_triangle_is_a_shape(self):
        self.assertTrue(isinstance(self.tri, shapes.Shape))

    def test_a_smooth_triangle_is_a_triangle(self):
        self.assertTrue(isinstance(self.tri, triangles.Triangle))

    def test_constructing_a_smooth_triangle(self):
        self.assertEqual(self.tri.p1, tuples.point( 0, 1, 0))
        self.assertEqual(self.tri.p2, tuples.point(-1, 0, 0))
        self.assertEqual(self.tri.p3, tuples.point( 1, 0, 0))
        self.assertEqual(self.tri.n1, tuples.vector( 0, 1, 0))
        self.assertEqual(self.tri.n2, tuples.vector(-1, 0, 0))
        self.assertEqual(self.tri.n3, tuples.vector( 1, 0, 0))

    #def test_finding_the_normal_on_a_triangle(self):
        #t = triangle(tuples.point( 0, 1, 0),
                     #tuples.point(-1, 0, 0),
                     #tuples.point( 1, 0, 0))
#
        #n1 = t.local_normal_at(tuples.point(   0,  0.5, 0))
        #n2 = t.local_normal_at(tuples.point(-0.5, 0.75, 0))
        #n3 = t.local_normal_at(tuples.point( 0.5, 0.25, 0))
#
        #self.assertEqual(n1, t.normal)
        #self.assertEqual(n2, t.normal)
        #self.assertEqual(n3, t.normal)
#
    #def test_intersecting_ray_parallel_to_triangle(self):
        #t = triangle(tuples.point( 0, 1, 0),
                     #tuples.point(-1, 0, 0),
                     #tuples.point( 1, 0, 0))
        #r = rays.ray(tuples.point(0, -1, -2),
                     #tuples.vector(0, 1, 0))
#
        #xs = t.local_intersect(r)
#
        #self.assertEqual(len(xs), 0)
#
    #def test_a_ray_misses_p1_p3_edge(self):
        #t = triangle(tuples.point( 0, 1, 0),
                     #tuples.point(-1, 0, 0),
                     #tuples.point( 1, 0, 0))
        #r = rays.ray(tuples.point(1, 1, -2),
                     #tuples.vector(0, 0, 1))
#
        #xs = t.local_intersect(r)
#
        #self.assertEqual(len(xs), 0)
#
    #def test_a_ray_misses_p1_p2_edge(self):
        #t = triangle(tuples.point( 0, 1, 0),
                     #tuples.point(-1, 0, 0),
                     #tuples.point( 1, 0, 0))
        #r = rays.ray(tuples.point(-1, 1, -2),
                     #tuples.vector(0, 0, 1))
#
        #xs = t.local_intersect(r)
#
        #self.assertEqual(len(xs), 0)
#
    #def test_a_ray_misses_p2_p3_edge(self):
        #t = triangle(tuples.point( 0, 1, 0),
                     #tuples.point(-1, 0, 0),
                     #tuples.point( 1, 0, 0))
        #r = rays.ray(tuples.point( 0, -1, -2),
                     #tuples.vector(0, 0, 1))
#
        #xs = t.local_intersect(r)
#
        #self.assertEqual(len(xs), 0)
#
    #def test_a_ray_strikes_a_triangle(self):
        #t = triangle(tuples.point( 0, 1, 0),
                     #tuples.point(-1, 0, 0),
                     #tuples.point( 1, 0, 0))
        #r = rays.ray(tuples.point( 0, 0.5, -2),
                     #tuples.vector(0, 0, 1))
#
        #xs = t.local_intersect(r)
#
        #self.assertEqual(len(xs), 1)
        #self.assertEqual(xs[0].t, 2)
#
    #def test_triangle_bounds(self):
        #t = triangle(tuples.point( 0, 1, 0),
                     #tuples.point(-1, 0, 0),
                     #tuples.point( 1, 0, 0))
#
        #b = t.bounds()
#
        #self.assertEqual(b[0].x, -1)
        #self.assertEqual(b[0].y, 0)
        #self.assertEqual(b[0].z, 0)
        #self.assertEqual(b[1].x, 1)
        #self.assertEqual(b[1].y, 1)
        #self.assertEqual(b[1].z, 0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
