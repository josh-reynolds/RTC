# to run tests: python -m unittest -v cylinders

import unittest
import materials
import shapes

class Cylinder(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt):
        pass

def cylinder():
    return Cylinder()

class CylinderTestCase(unittest.TestCase):
    def test_a_cylinder_is_a_shape(self):
        c = cylinder()

        self.assertTrue(isinstance(c, shapes.Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
