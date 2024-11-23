# to run tests: python -m unittest -v transformations

import unittest
from tuple import point, vector
from matrix import identity

def translation(dx, dy, dz):
    result = identity()
    result.data[0][3] += dx
    result.data[1][3] += dy
    result.data[2][3] += dz
    return result

def scaling(dx, dy, dz):
    return identity()

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
