import math
from tuple import point
from canvas import canvas
from color import color
from spheres import sphere
from rays import ray
from intersections import hit
from lights import point_light
from materials import lighting

ray_origin = point(0, 0, -5)
wall_z = 10
wall_size = 7.0

canvas_pixels = 100
pixel_size = wall_size / canvas_pixels
half = wall_size / 2
c = canvas(canvas_pixels, canvas_pixels)

shape = sphere()
shape.material.color = color(1, 0.2, 1)

light_position = point(-10, 10, -10)
light_color = color(1, 1, 1)
light = point_light(light_position, light_color)

for y in range(canvas_pixels):
    world_y = half - pixel_size * y
    
    for x in range(canvas_pixels):
        world_x = half - pixel_size * x
        position = point(world_x, world_y, wall_z)
        r = ray(ray_origin, (position - ray_origin).normalize())
        xs = shape.intersect(r)
        h = hit(xs)

        if h:
            pt = r.position(h.t)
            normal = h.object.normal_at(pt)
            eye = -r.direction
            col = lighting(h.object.material, light, pt, eye, normal)
            c.write_pixel(x, y, col)

f = open("phong_sphere.ppm", "w")
lines = c.to_ppm()
for line in lines:
    f.write(line + "\n")
f.close()

