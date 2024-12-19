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

start_time = datetime.now()

floor = plane()
p1 = stripe_pattern(GREEN, WHITE)
p1.set_transform(rotation_y(math.pi/2))
p2 = stripe_pattern(YELLOW, BLUE)
floor.material.pattern = blend_pattern(p1, p2)
floor.material.pattern.set_transform(rotation_y(math.pi/4) * translation(0, 0.1, 0))
floor.material.reflective = 0.9

middle = sphere()
middle.transform = translation(-0.5, 1, 0.5) * rotation_z(math.pi/4)
middle.material.diffuse = 0.2
middle.material.ambient = 0.2
middle.material.specular = 1
middle.material.shininess = 300
middle.material.reflective = 0.9
middle.material.transparency = 0.9
middle.material.refractive_index = 1.5
middle.material.color = color(0, 0, 0.1)

right = cube()
right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5)
right.material.diffuse = 0.7
right.material.specular = 0.3
right.material.pattern = gradient_pattern(MAGENTA, GREEN)
right.material.pattern.set_transform(translation(1, 0, 0) * scaling(2, 1, 1))

left = cube()
left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33)
left.material.diffuse = 0.7
left.material.specular = 0.3
left.material.reflective = 0.5
left.material.pattern = ring_pattern(checker_pattern(WHITE, BLACK), 
                                     YELLOW)
left.material.pattern.set_transform(scaling(0.1, 0.1, 0.1) * rotation_x(math.pi/2))

w = world()
w.objects.append(floor)
w.objects.append(middle)
w.objects.append(right)
w.objects.append(left)
w.light = point_light(point(-10, 10, -10), WHITE)

cam = camera(300, 150, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)
image_to_file(image, "./output/cubes.ppm")

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))
