# to run tests: python -m unittest -v materials

import unittest
from color import color

class Material:
    def __init__(self):
        self.color = color(1, 1, 1)
        self.ambient = 0.1
        self.diffuse = 0.9
        self.specular = 0.9
        self.shininess = 200.0

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
