# to run tests: python -m unittest -v groups

import unittest
import materials
import shapes
from matrices import identity
from rays import ray
from tuples import point, vector

class Group(shapes.Shape):
    def __init__(self):
        shapes.Shape.__init__(self)
        self.contents = []

    def __len__(self):
        return len(self.contents)

    def __contains__(self, item):
        for current_item in self.contents:
            if item == current_item:
                return True
        return False
    
    def add_child(self, child):
        self.contents.append(child)
        child.parent = self

    def local_intersect(self, r):
        xs = []
        return xs

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

    def test_adding_child_to_group(self):
        g = group()
        s = shapes.test_shape()

        g.add_child(s)

        self.assertEqual(len(g), 1)
        self.assertTrue(s in g)
        self.assertEqual(s.parent, g)

    def test_intersecting_a_ray_with_an_empty_group(self):
        g = group()
        r = ray(point(0, 0, 0), vector(0, 0, 1))

        xs = g.local_intersect(r)

        self.assertEqual(len(xs), 0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
