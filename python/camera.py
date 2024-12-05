# to run tests: python -m unittest -v camera

import unittest
import math
from matrix import identity

class Camera:
    def __init__(self, hsize, vsize, field_of_view):
        self.hsize = hsize
        self.vsize = vsize
        self.field_of_view = field_of_view
        self.transform = identity()

def camera(hsize, vsize, field_of_view):
    return Camera(hsize, vsize, field_of_view)

class CameraTestCase(unittest.TestCase):
    def test_constructing_a_camera(self):
        hsize = 160
        vsize = 120
        field_of_view = math.pi / 2

        c = camera(hsize, vsize, field_of_view)

        self.assertEqual(c.hsize, 160)
        self.assertEqual(c.vsize, 120)
        self.assertEqual(c.field_of_view, math.pi/2)
        self.assertEqual(c.transform, identity())

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
