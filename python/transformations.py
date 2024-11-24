# to run tests: python -m unittest -v transformations

import unittest
import math
from tuple import point, vector
from matrix import identity

def translation(dx, dy, dz):
    result = identity()
    result.data[0][3] = dx
    result.data[1][3] = dy
    result.data[2][3] = dz
    return result

def scaling(dx, dy, dz):
    result = identity()
    result.data[0][0] = dx
    result.data[1][1] = dy
    result.data[2][2] = dz
    return result

def rotation_x(radians):
    result = identity()
    result.data[1][1] = math.cos(radians)
    result.data[1][2] = -math.sin(radians)
    result.data[2][1] = math.sin(radians)
    result.data[2][2] = math.cos(radians)
    return result

def rotation_y(radians):
    result = identity()
    result.data[0][0] = math.cos(radians)
    result.data[0][2] = math.sin(radians)
    result.data[2][0] = -math.sin(radians)
    result.data[2][2] = math.cos(radians)
    return result

def rotation_z(radians):
    result = identity()
    #result.data[0][0] = math.cos(radians)
    #result.data[0][2] = math.sin(radians)
    #result.data[2][0] = -math.sin(radians)
    #result.data[2][2] = math.cos(radians)
    return result

class TransformationsTestCase(unittest.TestCase):
    def test_multiplying_by_a_translation_matrix(self):
        transform = translation(5, -3, 2)
        p = point(-3, 4, 5)

        self.assertEqual(transform * p, point(2, 1, 7))

    def test_multiplying_by_inverse_of_a_translation_matrix(self):
        transform = translation(5, -3, 2)
        inv = transform.inverse()
        p = point(-3, 4, 5)

        self.assertEqual(inv * p, point(-8, 7, 3))

    def test_translation_does_not_affect_vectors(self):
        transform = translation(5, -3, 2)
        v = vector(-3, 4, 5)

        self.assertEqual(transform * v, v)

    def test_a_scaling_matrix_applied_to_a_point(self):
        transform = scaling(2, 3, 4)
        p = point(-4, 6, 8)

        self.assertEqual(transform * p, point(-8, 18, 32))

    def test_a_scaling_matrix_applied_to_a_vector(self):
        transform = scaling(2, 3, 4)
        v = vector(-4, 6, 8)

        self.assertEqual(transform * v, vector(-8, 18, 32))

    def test_multiplying_by_inverse_of_scaling_matrix(self):
        transform = scaling(2, 3, 4)
        inv = transform.inverse()
        v = vector(-4, 6, 8)

        self.assertEqual(inv * v, vector(-2, 2, 2))

    def test_reflection_is_negative_scaling(self):
        transform = scaling(-1, 1, 1)
        v = vector(2, 3, 4)

        self.assertEqual(transform * v, vector(-2, 3, 4))

    def test_rotating_a_point_around_x_axis(self):
        p = point(0, 1, 0)

        half_quarter = rotation_x(math.pi / 4)
        full_quarter = rotation_x(math.pi / 2)

        self.assertEqual(half_quarter * p, point(0, math.sqrt(2)/2, math.sqrt(2)/2))
        self.assertEqual(full_quarter * p, point(0, 0, 1))

    def test_inverse_of_x_rotation_rotates_in_opposite_direction(self):
        p = point(0, 1, 0)

        half_quarter = rotation_x(math.pi / 4)
        inver = half_quarter.inverse()

        self.assertEqual(inver * p, point(0, math.sqrt(2)/2, -math.sqrt(2)/2))

    def test_rotating_a_point_around_y_axis(self):
        p = point(0, 0, 1)

        half_quarter = rotation_y(math.pi / 4)
        full_quarter = rotation_y(math.pi / 2)

        self.assertEqual(half_quarter * p, point(math.sqrt(2)/2, 0, math.sqrt(2)/2))
        self.assertEqual(full_quarter * p, point(1, 0, 0))

    def test_rotating_a_point_around_z_axis(self):
        p = point(0, 1, 0)

        half_quarter = rotation_z(math.pi / 4)
        full_quarter = rotation_z(math.pi / 2)

        self.assertEqual(half_quarter * p, point(-math.sqrt(2)/2, math.sqrt(2)/2, 0))
        self.assertEqual(full_quarter * p, point(-1, 0, 0))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
