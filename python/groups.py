# to run tests: python -m unittest -v groups

import unittest
import materials
import shapes

class Group(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)

def group():
    return Group()

class GroupTestCase(unittest.TestCase):
    def test_a_group_is_a_shape(self):
        g = group()

        self.assertTrue(isinstance(g, shapes.Shape))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
