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

pencil = cylinder()
pencil.minimum = -2
pencil.maximum = 2
pencil.closed = True
pencil.transform = translation(0, 1, 0) * rotation_z(math.pi/2) * scaling(0.5, 0.5, 0.5)
pencil.material.color = color(0.73, 0.64, 0.08)
pencil.material.diffuse = 1
pencil.material.specular = 0.7
pencil.material.shininess = 100

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

w = world()
w.objects.append(floor)
w.objects.append(wall)
w.objects.append(eraser)
w.objects.append(pencil)
w.objects.append(ferrule)
w.objects.append(lead)
w.objects.append(wood)
w.light = point_light(point(-10, 10, -10), WHITE)

cam = camera(600, 300, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)
image_to_file(image, "./output/pencil.ppm")

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))
