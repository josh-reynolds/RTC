EPSILON = 0.00001

def flequal(a, b):
    if abs(a - b) < EPSILON:
        return True
    return False
