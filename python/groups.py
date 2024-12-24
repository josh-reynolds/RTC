# to run tests: python -m unittest -v groups

import unittest
import materials
import shapes
from matrices import identity

class Group(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)
        self.contents = []

    def __len__(self):
        return len(self.contents)

def group():
    return Group()

class GroupTestCase(unittest.TestCase):
    def test_a_group_is_a_shape(self):
        g = group()

        self.assertTrue(isinstance(g, shapes.Shape))

    def test_creating_a_group(self):
        g = group()

        self.assertEqual(g.transform, identity())
        self.assertEqual(len(g), 0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
