import math
from datetime import datetime
from tuples import point, vector
from colors import WHITE, BLACK, RED, GREEN, BLUE, MAGENTA, YELLOW, color
from spheres import sphere, glass_sphere
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
from obj_files import parse_obj_file
from csgs import csg

start_time = datetime.now()

floor = plane()
floor.material.color = BLUE

wall = plane()
wall.set_transform(translation(0, 0, 15) * rotation_x(math.pi/4))
wall.material.pattern = gradient_pattern(RED, YELLOW)

s1 = glass_sphere()
s1.set_transform(translation(0.7, 1, 0))

s2 = glass_sphere()
s2.set_transform(translation(-0.7, 1, 0))

s3 = glass_sphere()
s3.set_transform(translation(0, 1, 0))

c1 = csg("union", s1, s2)
c2 = csg("union", c1, s3)

w = world()
w.objects.append(floor)
w.objects.append(wall)
#w.objects.append(s1)
#w.objects.append(s2)
#w.objects.append(s3)
w.objects.append(c2)

w.light = point_light(point(-10, 10, -10), WHITE)

cam = camera(300, 150, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)
image_to_file(image, "./output/csg.ppm")

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))


