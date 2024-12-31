# to run tests: python -m unittest -v triangles

import unittest
import materials
import shapes
import tuples

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
        pass

    def local_normal_at(self, pt):
        pass

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
