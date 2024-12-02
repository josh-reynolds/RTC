# to run tests: python -m unittest -v world

import unittest
#from tuple import point
#from color import color

class World:
    def __init__(self):
        self.objects = []
        self.light = None

def world():
    return World()

class WorldTestCase(unittest.TestCase):
    def test_creating_a_world(self):
        w = world()

        self.assertEqual(len(w.objects), 0)
        self.assertEqual(w.light, None)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
