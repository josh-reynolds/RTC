from tuple import point, vector

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

p = Projectile(point(0, 1, 0), vector(1, 1, 0).normalize())
e = Environment(vector(0, -0.1, 0), vector(-0.01, 0, 0))

count = 0
print("{0}\t{1}\t{2}".format("Tick", "x", "y"))
while p.position.y > 0:
    print("{0}\t{1:.2f}\t{2:.2f}".format(count, p.position.x, p.position.y))
    count += 1
    p = tick(e, p)



