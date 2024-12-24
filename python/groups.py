# to run tests: python -m unittest -v groups

import unittest
import materials
import shapes
from matrices import identity
from rays import ray
from tuples import point, vector
from spheres import sphere
from transformations import translation, scaling

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
        for item in self.contents:
            xs += item.intersect(r)
        xs.sort(key=lambda x: x.t)
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

    def test_intersecting_a_ray_with_a_nonempty_group(self):
        g = group()

        s1 = sphere()
        g.add_child(s1)

        s2 = sphere()
        s2.set_transform(translation(0, 0, -3))
        g.add_child(s2)

        s3 = sphere()
        s3.set_transform(translation(5, 0, 0))
        g.add_child(s3)

        r = ray(point(0, 0, -5), vector(0, 0, 1))
        xs = g.local_intersect(r)

        self.assertEqual(len(xs), 4)
        self.assertEqual(xs[0].object, s2)
        self.assertEqual(xs[1].object, s2)
        self.assertEqual(xs[2].object, s1)
        self.assertEqual(xs[3].object, s1)

    def test_intersecting_a_transformed_group(self):
        g = group()
        g.set_transform(scaling(2, 2, 2))

        s = sphere()
        s.set_transform(translation(5, 0, 0))
        g.add_child(s)

        r = ray(point(10, 0, -10), vector(0, 0, 1))
        xs = g.intersect(r)

        self.assertEqual(len(xs), 2)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
