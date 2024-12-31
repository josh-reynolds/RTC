# to run tests: python -m unittest -v triangles

import unittest
import materials
import shapes

class Triangle(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt):
        pass

    #def bounds(self):
        #return (tuples.point(-1, -1, -1),
                #tuples.point(1, 1, 1))

def triangle():
    return Triangle()

class TriangleTestCase(unittest.TestCase):
    def test_a_triangle_is_a_shape(self):
        t = triangle()

        self.assertTrue(isinstance(t, shapes.Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
