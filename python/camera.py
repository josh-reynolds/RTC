# to run tests: python -m unittest -v camera

import unittest
import math
from matrix import identity
from utils import flequal
from tuple import point, vector
from rays import ray

class Camera:
    def __init__(self, hsize, vsize, field_of_view):
        self.hsize = hsize
        self.vsize = vsize
        self.field_of_view = field_of_view
        self.transform = identity()

        half_view = math.tan(self.field_of_view/2)
        aspect = self.hsize/self.vsize

        if aspect >= 1:
            self.half_width = half_view
            self.half_height = half_view / aspect
        else:
            self.half_width = half_view * aspect
            self.half_height = half_view

        self.pixel_size = (self.half_width * 2) / self.hsize

    def ray_for_pixel(self, x, y):
        return ray(point(0, 0, 0), vector(0, 0, -1))

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

    def test_pixel_size_for_horizontal_canvas(self):
        c = camera(200, 125, math.pi/2)

        self.assertTrue(flequal(c.pixel_size, 0.01))

    def test_pixel_size_for_vertical_canvas(self):
        c = camera(125, 200, math.pi/2)

        self.assertTrue(flequal(c.pixel_size, 0.01))

    def test_constructing_ray_through_center_of_canvas(self):
        c = camera(201, 101, math.pi/2)

        r = c.ray_for_pixel(100, 50)

        self.assertEqual(r.origin, point(0, 0, 0))
        self.assertEqual(r.direction, vector(0, 0, -1))
        

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()