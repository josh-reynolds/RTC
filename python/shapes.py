# to run tests: python -m unittest -v shapes

import unittest
#import math
#from rays import ray
#from tuple import point, vector
#import intersections
from matrix import identity
from transformations import translation
#from materials import material

class Shape:
    def __init__(self):
        self.transform = identity()
        #self.material = material()

    #def __eq__(self, other):
        #if isinstance(other, self.__class__):
            #return (self.transform == other.transform and
                    #self.material == other.material)
        #else:
            #return False
#
    #def intersect(self, r):
        #result = []
        #r2 = r.transform(self.transform.inverse())
        #sphere_to_ray = r2.origin - point(0, 0, 0)
#
        #a = r2.direction.dot(r2.direction)
        #b = 2 * r2.direction.dot(sphere_to_ray)
        #c = sphere_to_ray.dot(sphere_to_ray) - 1
#
        #discriminant = (b ** 2) - (4 * a * c)
#
        #if discriminant >= 0:
            #dsqrt = math.sqrt(discriminant)
            #t1 = (-b - dsqrt) / (2 * a)
            #t2 = (-b + dsqrt) / (2 * a)
            #result.append(intersections.intersection(t1, self))
            #result.append(intersections.intersection(t2, self))
#
        #return result
    
    def set_transform(self, t):
        self.transform = t

    #def normal_at(self, pt):
        #object_point = self.transform.inverse() * pt
        #object_normal = object_point - point(0, 0, 0)
        #world_normal = self.transform.inverse().transpose() * object_normal
        #world_normal.w = 0
        #return world_normal.normalize()

def test_shape():
    return Shape()

class ShapeTestCase(unittest.TestCase):
    def test_a_shapes_default_transformation(self):
        s = test_shape()

        self.assertEqual(s.transform, identity())

    def test_assigning_a_transformation(self):
        s = test_shape()

        s.set_transform(translation(2, 3, 4))

        self.assertEqual(s.transform, translation(2, 3, 4))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
