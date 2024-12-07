import math
from datetime import datetime
from tuple import point, vector
from color import color
from spheres import sphere
from world import world
from lights import point_light
from camera import camera
from transformations import view_transform, translation, rotation_y, rotation_x, scaling
from planes import plane

start_time = datetime.now()

floor = plane()
#floor.transform = scaling(10, 0.01, 10)
#floor.material.color = color(1, 0.9, 0.9)
#floor.material.specular = 0

middle = sphere()
middle.transform = translation(-0.5, 1, 0.5)
middle.material.color = color(0.1, 1, 0.5)
middle.material.diffuse = 0.7
middle.material.specular = 0.3

right = sphere()
right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5)
right.material.color = color(0.5, 1, 0.1)
right.material.diffuse = 0.7
right.material.specular = 0.3

left = sphere()
left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33)
left.material.color = color(1, 0.8, 0.1)
left.material.diffuse = 0.7
left.material.specular = 0.3

w = world()

w.objects.append(floor)
w.objects.append(middle)
w.objects.append(right)
w.objects.append(left)

w.light = point_light(point(-10, 10, -10), color(1, 1, 1))

cam = camera(200, 100, math.pi/3)
cam.transform = view_transform(point(0, 1.5, -5),
                               point(0, 1, 0),
                               vector(0, 1, 0))

image = cam.render(w)

f = open("plane_render.ppm", "w")
lines = image.to_ppm()
for line in lines:
    f.write(line + "\n")
f.close()

end_time = datetime.now()
print("Image size: {} x {}".format(cam.hsize, cam.vsize))
print('Render time: {}'.format(end_time - start_time))
