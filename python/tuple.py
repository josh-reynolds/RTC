# to run tests: python -m unittest -v tuple

# TO_DO: implement EPSILON and float equals

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

    def __add__(self, other):
        return Tuple(self.x + other.x,
                     self.y + other.y,
                     self.z + other.z,
                     self.w + other.w)

    def __sub__(self, other):
        return Tuple(self.x - other.x,
                     self.y - other.y,
                     self.z - other.z,
                     self.w - other.w)

    def __neg__(self):
        return Tuple(-self.x, -self.y, -self.z, -self.w)

    def __mul__(self, rhs):
        if isinstance(rhs, float) or isinstance(rhs, int):
            return Tuple(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)


def point(x, y, z):
    return Tuple(x, y, z, 1)

def vector(x, y, z):
    return Tuple(x, y, z, 0)

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

    def test_vector_creates_tuples_with_w_0(self):
        a = vector(4, -4, 3)
        self.assertFalse(a.isPoint())
        self.assertTrue(a.isVector())
        self.assertEqual(a, Tuple(4, -4, 3, 0))

    def test_adding_two_tuples(self):
        a1 = Tuple(3, -2, 5, 1)
        a2 = Tuple(-2, 3, 1, 0)
        self.assertEqual(a1 + a2, Tuple(1, 1, 6, 1))
    
    def test_subtracting_two_points(self):
        p1 = point(3, 2, 1)
        p2 = point(5, 6, 7)
        self.assertEqual(p1 - p2, vector(-2, -4, -6))

    def test_subtracting_vector_from_point(self):
        p = point(3, 2, 1)
        v = vector(5, 6, 7)
        self.assertEqual(p - v, point(-2, -4, -6))
        
    def test_subtracting_two_vectors(self):
        v1 = vector(3, 2, 1)
        v2 = vector(5, 6, 7)
        self.assertEqual(v1 - v2, vector(-2, -4, -6))

    def test_subtracting_vector_from_zero_vector(self):
        zero = vector(0, 0, 0)
        v = vector(1, -2, 3)
        self.assertEqual(zero - v, vector(-1, 2, -3))

    def test_negating_a_tuple(self):
        a = Tuple(1, -2, 3, -4)
        self.assertEqual(-a, Tuple(-1, 2, -3, 4))

    def test_multiply_tuple_by_scalar(self):
        a = Tuple(1, -2, 3, -4)
        self.assertEqual(a * 3.5, Tuple(3.5, -7, 10.5, -14))

    def test_multiply_tuple_by_fraction(self):
        a = Tuple(1, -2, 3, -4)
        self.assertEqual(a * 0.5, Tuple(0.5, -1, 1.5, -2))

