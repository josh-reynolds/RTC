# to run tests: python -m unittest -v bounds

import unittest
import materials
import shapes

class Bounds(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

    def local_intersect(self, r):
        pass

    def local_normal_at(self, pt):
        pass

def bounds():
    return Bounds()

class BoundsTestCase(unittest.TestCase):
    def test_bounds_is_a_shape(self):
        b = bounds()

        self.assertTrue(isinstance(b, shapes.Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
