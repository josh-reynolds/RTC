import math
from datetime import datetime
from tuples import point, vector
from colors import WHITE, BLACK, RED, GREEN, BLUE, MAGENTA, YELLOW, color
from spheres import sphere
from worlds import world
from lights import point_light
from cameras import camera
from transformations import view_transform, translation, rotation_x, rotation_y, rotation_z, scaling
from planes import plane
from stripes import stripe_pattern
from gradients import gradient_pattern
from rings import ring_pattern
from checkers import checker_pattern
from radial_gradients import radial_gradient_pattern
from blended_patterns import blend_pattern
from utils import image_to_file
from cubes import cube
from cylinders import cylinder
from cones import cone
from groups import group
from triangles import triangle

start_time = datetime.now()

floor = plane()
floor.color = color(0.7, 0.4, 0.7)

wall = plane()
wall.transform = rotation_x(math.pi/2) * translation(0, 15, 0)
wall.material.color = color(0.9, 0.8, 0.7)

t = triangle(point( 0, 1, 0),
             point(-1, 0, 0),
             point( 1, 0, 0))
t.material.color = BLUE
t.material.transparency = 0.5
t.material.refractive_index = 1.3

w = world()
w.objects.append(floor)
w.objects.append(wall)
w.objects.append(t)
w.light = point_light(point(-10, 10, -10), WHITE)

cam = camera(300, 150, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)
image_to_file(image, "./output/triangle.ppm")

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))


