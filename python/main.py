from tuple import point, vector
from canvas import canvas
from color import color

class Projectile:
    def __init__(self, position, velocity):
        self.position = position
        self.velocity = velocity

class Environment:
    def __init__(self, gravity, wind):
        self.gravity = gravity
        self.wind = wind

def tick(env, proj):
    position = proj.position + proj.velocity
    velocity = proj.velocity + env.gravity + env.wind
    return Projectile(position, velocity)

start = point(0, 1, 0)
velocity = vector(1, 1.8, 0).normalize() * 11.25
p = Projectile(start, velocity)

gravity = vector(0, -0.1, 0)
wind = vector(-0.01, 0, 0)
e = Environment(gravity, wind)

c = canvas(900, 550)

count = 0
col = color(1, 0, 0)
print("{0}\t{1}\t{2}".format("Tick", "x", "y"))
while p.position.y > 0:
    print("{0}\t{1:.2f}\t{2:.2f}".format(count, p.position.x, p.position.y))
    px = round(p.position.x)
    py = 550 - round(p.position.y)
    c.write_pixel(px, py, col)
    count += 1
    p = tick(e, p)

f = open("test_cannon.ppm", "w")
lines = c.to_ppm()
for line in lines:
    f.write(line + "\n")
f.close()

