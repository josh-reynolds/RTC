import math
from tuple import point
from canvas import canvas
from color import color
from transformations import rotation_y, scaling, translation

size = 400
radius = size * 3 / 8
center = size / 2
c = canvas(size, size)
col = color(1, 0, 0)

twelve = point(0, 0, 1)

for i in range(12):
    p = (translation(center, 0, center) * 
         scaling(radius, 0, radius) * 
         rotation_y(i * math.pi / 6) * 
         twelve)
    c.write_pixel(math.floor(p.x), math.floor(p.z), col)

f = open("clock.ppm", "w")
lines = c.to_ppm()
for line in lines:
    f.write(line + "\n")
f.close()

