import math
from datetime import datetime
from tuples import point, vector
from colors import WHITE, BLACK, RED, GREEN, BLUE, MAGENTA, YELLOW, DARK_GREY, LIGHT_GREY, color
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
floor.material.color = DARK_GREY

wall = plane()
wall.set_transform(translation(0, 0, 15) * rotation_x(math.pi/4))
wall.material.color = LIGHT_GREY

c = cube()
c.material.color = MAGENTA
c.set_transform(translation(0, 1, 0) * 
                rotation_z(math.pi/3) * 
                rotation_x(math.pi/3) *
                scaling(0.5, 0.5, 0.5))

cy1 = cylinder()
cy1.minimum = -2
cy1.maximum = 2
cy1.closed = True
cy1.material.color= GREEN
cy1.set_transform(translation(0, 1, 0) * 
                 rotation_z(math.pi/3) * 
                 rotation_x(math.pi/3) *
                 scaling(0.3, 0.3, 0.3))

cy2 = cylinder()
cy2.minimum = -2
cy2.maximum = 2
cy2.closed = True
cy2.material.color= GREEN
cy2.set_transform(translation(0, 1, 0) * 
                 rotation_z(math.pi/3) * 
                 rotation_x(math.pi/3 + math.pi/2) *
                 scaling(0.3, 0.3, 0.3))

c1 = csg("difference", c, cy1)
c2 = csg("difference", c1, cy2)

w = world()
w.objects.append(floor)
w.objects.append(wall)
w.objects.append(c2)

w.light = point_light(point(-10, 10, -10), WHITE)

cam = camera(400, 200, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)
image_to_file(image, "./output/csg_2.ppm")

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))


