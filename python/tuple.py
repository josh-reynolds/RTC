# to run tests: python -m unittest -v tuple
import unittest

class Tuple():
    def __init__(self, x, y, z, w):
        self.x = x
        self.y = y
        self.z = z
        self.w = w
    
    def isPoint(self):
        return self.w == 1.0

    def isVector(self):
        return self.w == 0.0

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.__dict__ == other.__dict__
        else:
            return False

    def __ne__(self, other):
        return not self.__eq__(other)

def point(x, y, z):
    return Tuple(x, y, z, 1)


class TupleTestCase(unittest.TestCase):
    def test_tuple_with_w_1_is_point(self):
        a = Tuple(4.3, -4.2, 3.1, 1.0)
        self.assertEqual(a.x,  4.3)
        self.assertEqual(a.y, -4.2)
        self.assertEqual(a.z,  3.1)
        self.assertEqual(a.w,  1.0)
        self.assertTrue(a.isPoint())
        self.assertFalse(a.isVector())

    def test_tuple_with_w_0_is_vector(self):
        a = Tuple(4.3, -4.2, 3.1, 0.0)
        self.assertEqual(a.x,  4.3)
        self.assertEqual(a.y, -4.2)
        self.assertEqual(a.z,  3.1)
        self.assertEqual(a.w,  0.0)
        self.assertFalse(a.isPoint())
        self.assertTrue(a.isVector())

    def test_point_creates_tuples_with_w_1(self):
        a = point(4, -4, 3)
        self.assertTrue(a.isPoint())
        self.assertFalse(a.isVector())
        self.assertEqual(a, Tuple(4, -4, 3, 1))

