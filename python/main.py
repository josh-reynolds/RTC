import math
from tuple import point
from canvas import canvas
from color import color
from spheres import sphere
from rays import ray
from intersections import hit

ray_origin = point(0, 0, -5)
wall_z = 10
wall_size = 7.0

canvas_pixels = 100
pixel_size = wall_size / canvas_pixels
half = wall_size / 2

c = canvas(canvas_pixels, canvas_pixels)
col = color(1, 0, 0)
shape = sphere()

for y in range(canvas_pixels):
    world_y = half - pixel_size * y
    
    for x in range(canvas_pixels):
        world_x = half - pixel_size * x
        position = point(world_x, world_y, wall_z)
        r = ray(ray_origin, (position - ray_origin).normalize())
        xs = shape.intersect(r)
        if hit(xs):
            c.write_pixel(x, y, col)

f = open("sphere.ppm", "w")
lines = c.to_ppm()
for line in lines:
    f.write(line + "\n")
f.close()

