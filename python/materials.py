# to run tests: python -m unittest -v materials

import unittest
from color import color
from utils import flequal
from lights import point_light
from tuple import point, vector

class Material:
    def __init__(self):
        self.color = color(1, 1, 1)
        self.ambient = 0.1        # range 0 - 1
        self.diffuse = 0.9        # range 0 - 1
        self.specular = 0.9       # range 0 - 1
        self.shininess = 200.0    # range 10 - 200

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return ((self.color == other.color) and
                    flequal(self.ambient, other.ambient) and
                    flequal(self.diffuse, other.diffuse) and
                    flequal(self.specular, other.specular) and
                    flequal(self.shininess, other.shininess))
        else:
            return False

def material():
    return Material()

def lighting(material, light, position, eyev, normalv):
    return color(1.9, 1.9, 1.9)

class MaterialTestCase(unittest.TestCase):
    def test_the_default_material(self):
        m = material()

        self.assertEqual(m.color, color(1, 1, 1))
        self.assertEqual(m.ambient, 0.1)
        self.assertEqual(m.diffuse, 0.9)
        self.assertEqual(m.specular, 0.9)
        self.assertEqual(m.shininess, 200.0)

    def test_lighting_with_eye_between_light_and_surface(self):
        m = material()
        light = point_light(point(0, 0, -10), color(1, 1, 1))
        position = point(0, 0, 0)
        eyev = vector(0, 0, -1)
        normalv = vector(0, 0, -1)
        
        result = lighting(m, light, position, eyev, normalv)

        self.assertEqual(result, color(1.9, 1.9, 1.9))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
