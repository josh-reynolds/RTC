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

start_time = datetime.now()

floor = plane()
floor.color = color(0.7, 0.4, 0.7)

wall = plane()
wall.transform = rotation_x(math.pi/2) * translation(0, 15, 0)
wall.material.color = color(0.9, 0.8, 0.7)

eraser = sphere()
eraser.transform = translation(1.5, 1, 0) * scaling(0.5, 0.5, 0.5)
eraser.material.color = color(0.8, 0.24, 0.51)
eraser.material.diffuse = 1
eraser.material.specular = 0.1
eraser.material.shininess = 10

body = cylinder()
body.minimum = -2
body.maximum = 2
body.closed = True
body.transform = translation(0, 1, 0) * rotation_z(math.pi/2) * scaling(0.5, 0.5, 0.5)
body.material.color = color(0.73, 0.64, 0.08)
body.material.diffuse = 1
body.material.specular = 0.7
body.material.shininess = 100

ferrule = cylinder()
ferrule.minimum = 0
ferrule.maximum = 1
ferrule.closed = True
ferrule.transform = translation(1.5, 1, 0) * rotation_z(math.pi/2) * scaling(0.5, 0.5, 0.5)
ferrule.material.color = color(0.06, 0.2, 0.05)
ferrule.material.diffuse = 1
ferrule.material.specular = 1
ferrule.material.shininess = 300

lead = cone()
lead.minimum = -0.5
lead.maximum = 0.0
lead.transform = translation(-2, 1, 0) * rotation_z(math.pi/2) * scaling(0.5, 1, 0.5)
lead.material.color = color (0.35, 0.35, 0.35)
lead.material.diffuse = 0.7
lead.material.specular = 0.3

wood = cone()
wood.minimum = -1.0
wood.maximum = -0.5
wood.transform = translation(-2, 1, 0) * rotation_z(math.pi/2) * scaling(0.5, 1, 0.5)
wood.material.color = color(0.75, 0.54, 0.37)
wood.material.diffuse = 0.7
wood.material.specular = 0.3
wood.material.shininess = 10

pencil = group()
#pencil.skip_bounds_check = True
pencil.add_child(eraser)
pencil.add_child(body)
pencil.add_child(ferrule)
pencil.add_child(lead)
pencil.add_child(wood)
pencil.set_transform(rotation_z(math.pi/5))

w = world()
w.objects.append(floor)
w.objects.append(wall)
w.objects.append(pencil)
w.light = point_light(point(-10, 10, -10), WHITE)

cam = camera(300, 150, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)
image_to_file(image, "./output/bounded_pencil_3.ppm")

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))
