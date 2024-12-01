# to run tests: python -m unittest -v materials

import unittest
from color import color
from utils import flequal

class Material:
    def __init__(self):
        self.color = color(1, 1, 1)
        self.ambient = 0.1
        self.diffuse = 0.9
        self.specular = 0.9
        self.shininess = 200.0

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

class MaterialTestCase(unittest.TestCase):
    def test_the_default_material(self):
        m = material()

        self.assertEqual(m.color, color(1, 1, 1))
        self.assertEqual(m.ambient, 0.1)
        self.assertEqual(m.diffuse, 0.9)
        self.assertEqual(m.specular, 0.9)
        self.assertEqual(m.shininess, 200.0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
