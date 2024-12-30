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

def hexagon_corner():
    corner = sphere()
    corner.set_transform(translation(0, 0, -1) * 
                         scaling(0.25, 0.25, 0.25))
    return corner

def hexagon_edge():
    edge = cylinder()
    edge.minimum = 0
    edge.maximum = 1
    edge.set_transform(translation(0, 0, -1) *
                       rotation_y(-math.pi/6) * 
                       rotation_z(-math.pi/2) * 
                       scaling(0.25, 1, 0.25))
    return edge

def hexagon_side():
    side = group()
    side.skip_bounds_check = True
    side.add_child(hexagon_corner())
    side.add_child(hexagon_edge())
    return side

def hexagon():
    hx = group()
    hx.skip_bounds_check = True
    for n in range(6):
        side = hexagon_side()
        side.set_transform(rotation_y(n * math.pi/3))
        hx.add_child(side)
    return hx

start_time = datetime.now()

floor = plane()
floor.color = color(0.7, 0.4, 0.7)

wall = plane()
wall.transform = rotation_x(math.pi/2) * translation(0, 15, 0)
wall.material.color = color(0.9, 0.8, 0.7)

hx = hexagon()
hx.skip_bounds_check = True
hx.set_transform(translation(0, 1, 0) * rotation_x(math.pi/3))

w = world()
w.objects.append(floor)
w.objects.append(wall)
w.objects.append(hx)
w.light = point_light(point(-10, 10, -10), WHITE)

cam = camera(300, 150, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)
image_to_file(image, "./output/hexagon_group.ppm")

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))


