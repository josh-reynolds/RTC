# to run tests: python -m unittest -v csgs

import unittest
import materials
import shapes
import spheres

class CSG(shapes.Shape):
    def __init__(self, op, shape1, shape2):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt, i):
        pass

    def bounds(self):
        pass

def csg(op, shape1, shape2):
    return CSG(op, shape1, shape2)

class CSGTestCase(unittest.TestCase):
    def test_a_csg_is_a_shape(self):
        c = csg("union", spheres.sphere(), spheres.sphere())

        self.assertTrue(isinstance(c, shapes.Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
