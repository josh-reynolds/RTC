# to run tests: python -m unittest -v cubes

import unittest
import materials
import shapes

class Cube(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt):
        pass

def cube():
    return Cube()

class CubeTestCase(unittest.TestCase):
    def test_a_cube_is_a_shape(self):
        c = cube()

        self.assertTrue(isinstance(c, shapes.Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
