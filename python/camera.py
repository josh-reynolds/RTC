# to run tests: python -m unittest -v camera

import unittest
import math
from matrix import identity
from utils import flequal
from tuple import point, vector
from rays import ray
from transformations import rotation_y, translation, view_transform
from world import default_world
from canvas import canvas
from color import color

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

    def ray_for_pixel(self, px, py):
        xoffset = (px + 0.5) * self.pixel_size
        yoffset = (py + 0.5) * self.pixel_size

        world_x = self.half_width - xoffset
        world_y = self.half_height - yoffset

        pixel = self.transform.inverse() * point(world_x, world_y, -1)
        origin = self.transform.inverse() * point(0, 0, 0)
        direction = (pixel - origin).normalize()

        return ray(origin, direction)
    
    def render(self, world):
        image = canvas(self.hsize, self.vsize)

        for y in range(self.vsize):
            for x in range (self.hsize):
                r = self.ray_for_pixel(x, y)
                col = world.color_at(r)
                image.write_pixel(x, y, col)

        return image

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
        
    def test_constructing_ray_through_corner_of_canvas(self):
        c = camera(201, 101, math.pi/2)

        r = c.ray_for_pixel(0, 0)

        self.assertEqual(r.origin, point(0, 0, 0))
        self.assertEqual(r.direction, vector(0.66519, 0.33259, -0.66851))

    def test_constructing_ray_when_camera_is_transformed(self):
        c = camera(201, 101, math.pi/2)
        c.transform = rotation_y(math.pi/4) * translation(0, -2, 5)

        r = c.ray_for_pixel(100, 50)

        self.assertEqual(r.origin, point(0, 2, -5))
        self.assertEqual(r.direction, vector(math.sqrt(2)/2, 0, -math.sqrt(2)/2))

    def test_rendering_a_world_with_a_camera(self):
        w = default_world()
        c = camera(11, 11, math.pi/2)

        frm = point(0, 0, -5)
        to = point(0, 0, 0)
        up = vector(0, 1, 0)
        c.transform = view_transform(frm, to, up)

        image = c.render(w)

        self.assertEqual(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
