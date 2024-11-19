# to run tests: python -m unittest -v matrix

import unittest
from tuple import flequal, Tuple
#import math

class Matrix():
    def __init__(self, rows, columns):
        self.rows = rows
        self.columns = columns
        self.data = [[0 for x in range(columns)] for x in range(rows)]

    def transpose(self):
        result = matrix()
        for row in range(self.rows):
            for col in range(self.columns):
                result.data[col][row] = self.data[row][col]
        return result

    def __eq__(self, other):
        result = True
        if isinstance(other, self.__class__) and \
                self.rows == other.rows and \
                self.columns == other.columns:
                    for row in range(self.rows):
                        for col in range(self.columns):
                            if not flequal(self.__getitem__((row,col)), other[row,col]):
                                result = False
        else:
            result = False
        return result

    def __ne__(self, other):
        return not self.__eq__(other)

    #def __add__(self, other):
        #return Tuple(self.x + other.x,
                     #self.y + other.y,
                     #self.z + other.z,
                     #self.w + other.w)

    #def __sub__(self, other):
        #return Tuple(self.x - other.x,
                     #self.y - other.y,
                     #self.z - other.z,
                     #self.w - other.w)

    #def __neg__(self):
        #return Tuple(-self.x, -self.y, -self.z, -self.w)

    def __mul__(self, rhs):         
        if isinstance(rhs, Matrix):      # only concerned with two 4x4 matrices here
            m = matrix()
            for row in range(self.rows):
                for col in range(self.columns):
                    m.data[row][col] = self.data[row][0] * rhs[0, col] + \
                                       self.data[row][1] * rhs[1, col] + \
                                       self.data[row][2] * rhs[2, col] + \
                                       self.data[row][3] * rhs[3, col]
            return m
        elif isinstance(rhs, Tuple):
            result = Tuple(0, 0, 0, 0)

            r1 = self.data[0]
            row1 = Tuple(r1[0], r1[1], r1[2], r1[3])
            result.x = row1.dot(rhs)

            r2 = self.data[1]
            row2 = Tuple(r2[0], r2[1], r2[2], r2[3])
            result.y = row2.dot(rhs)

            r3 = self.data[2]
            row3 = Tuple(r3[0], r3[1], r3[2], r3[3])
            result.z = row3.dot(rhs)

            r4 = self.data[3]
            row4 = Tuple(r4[0], r4[1], r4[2], r4[3])
            result.w = row4.dot(rhs)

            return result

    def __rmul__(self, lhs):
        return self.__mul__(lhs)

    #def __truediv__(self, rhs):
        #if isinstance(rhs, float) or isinstance(rhs, int):
            #return Tuple(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)

    def __getitem__(self, w):
        return self.data[w[0]][w[1]]

    def __str__(self):
        result = ""
        for x in range(self.rows):
            for y in range(self.columns):
                result += str(self.data[x][y])
            result += "\n"
        return result

def matrix(rows=4, columns=4):
    return Matrix(rows, columns)

def identity():
    result = matrix()
    result.data[0] = [1, 0, 0, 0]
    result.data[1] = [0, 1, 0, 0]
    result.data[2] = [0, 0, 1, 0]
    result.data[3] = [0, 0, 0, 1]
    return result

class MatrixTestCase(unittest.TestCase):
    def test_matrix_creation(self):
        m = matrix()
        m.data[0] = [   1,      2,     3,     4]
        m.data[1] = [ 5.5,    6.5,   7.5,   8.5]
        m.data[2] = [   9,     10,    11,    12]
        m.data[3] = [13.5,   14.5,  15.5,  16.5]

        self.assertEqual(m.data[0][0],    1)
        self.assertEqual(m.data[0][3],    4)
        self.assertEqual(m.data[1][0],  5.5)
        self.assertEqual(m.data[1][2],  7.5)
        self.assertEqual(m.data[2][2],   11)
        self.assertEqual(m.data[3][0], 13.5)
        self.assertEqual(m.data[3][2], 15.5)

    def test_matrix_access(self):
        m = matrix()
        m.data[0] = [   1,      2,     3,     4]
        m.data[1] = [ 5.5,    6.5,   7.5,   8.5]
        m.data[2] = [   9,     10,    11,    12]
        m.data[3] = [13.5,   14.5,  15.5,  16.5]

        self.assertEqual(m[0,0],    1)
        self.assertEqual(m[0,3],    4)
        self.assertEqual(m[1,0],  5.5)
        self.assertEqual(m[1,2],  7.5)
        self.assertEqual(m[2,2],   11)
        self.assertEqual(m[3,0], 13.5)
        self.assertEqual(m[3,2], 15.5)

    def test_creation_2_by_2_matrix(self):
        m = matrix(2,2)
        m.data[0] = [-3,  5]
        m.data[1] = [ 1, -2]

        self.assertEqual(m[0,0], -3)
        self.assertEqual(m[0,1],  5)
        self.assertEqual(m[1,0],  1)
        self.assertEqual(m[1,1], -2)
        
    def test_creation_3_by_3_matrix(self):
        m = matrix(3,3)
        m.data[0] = [-3,  5,  0]
        m.data[1] = [ 1, -2, -7]
        m.data[2] = [ 0,  1,  1]

        self.assertEqual(m[0,0], -3)
        self.assertEqual(m[1,1], -2)
        self.assertEqual(m[2,2],  1)

    def test_matrix_equality_with_identical_matrices(self):
        a = matrix()
        a.data[0] = [1, 2, 3, 4]
        a.data[1] = [5, 6, 7, 8]
        a.data[2] = [9, 8, 7, 6]
        a.data[3] = [5, 4, 3, 2]

        b = matrix()
        b.data[0] = [1, 2, 3, 4]
        b.data[1] = [5, 6, 7, 8]
        b.data[2] = [9, 8, 7, 6]
        b.data[3] = [5, 4, 3, 2]

        self.assertEqual(a, b)

    def test_matrix_equality_with_different_matrices(self):
        a = matrix()
        a.data[0] = [1, 2, 3, 4]
        a.data[1] = [5, 6, 7, 8]
        a.data[2] = [9, 8, 7, 6]
        a.data[3] = [5, 4, 3, 2]

        b = matrix()
        b.data[0] = [2, 3, 4, 5]
        b.data[1] = [6, 7, 8, 9]
        b.data[2] = [8, 7, 6, 5]
        b.data[3] = [4, 3, 2, 1]

        self.assertNotEqual(a, b)

    def test_matrix_equality_with_different_size_matrices(self):
        a = matrix()
        a.data[0] = [1, 2, 3, 4]
        a.data[1] = [5, 6, 7, 8]
        a.data[2] = [9, 8, 7, 6]
        a.data[3] = [5, 4, 3, 2]

        b = matrix(3,3)
        b.data[0] = [1, 2, 3]
        b.data[1] = [5, 6, 7]
        b.data[2] = [9, 8, 7]
        
        self.assertNotEqual(a, b)

    def test_multiplying_two_matrices(self):
        a = matrix()
        a.data[0] = [1, 2, 3, 4]
        a.data[1] = [5, 6, 7, 8]
        a.data[2] = [9, 8, 7, 6]
        a.data[3] = [5, 4, 3, 2]

        b = matrix()
        b.data[0] = [-2, 1, 2,  3]
        b.data[1] = [ 3, 2, 1, -1]
        b.data[2] = [ 4, 3, 6,  5]
        b.data[3] = [ 1, 2, 7,  8]

        result = matrix()
        result.data[0] = [20, 22,  50,  48]
        result.data[1] = [44, 54, 114, 108]
        result.data[2] = [40, 58, 110, 102]
        result.data[3] = [16, 26,  46,  42]

        self.assertEqual(a * b, result)

    def test_multiplying_matrix_by_tuple(self):
        a = matrix()
        a.data[0] = [1, 2, 3, 4]
        a.data[1] = [2, 4, 4, 2]
        a.data[2] = [8, 6, 4, 1]
        a.data[3] = [0, 0, 0, 1]

        b = Tuple(1, 2, 3, 1)

        self.assertEqual(a * b, Tuple(18, 24, 33, 1))

    def test_multiplying_matrix_by_identity(self):
        a = matrix()
        a.data[0] = [0, 1,  2,  4]
        a.data[1] = [1, 2,  4,  8]
        a.data[2] = [2, 4,  8, 16]
        a.data[3] = [4, 8, 16, 32]

        self.assertEqual(a * identity(), a)

    def test_multiplying_tuple_by_identity(self):
        a = Tuple(1, 2, 3, 4)

        self.assertEqual(identity() * a, a)

    def test_transposing_a_matrix(self):
        a = matrix()
        a.data[0] = [0, 9, 3, 0]
        a.data[1] = [9, 8, 0, 8]
        a.data[2] = [1, 8, 5, 3]
        a.data[3] = [0, 0, 5, 8]

        result = matrix()
        result.data[0] = [0, 9, 1, 0]
        result.data[1] = [9, 8, 8, 0]
        result.data[2] = [3, 0, 5, 5]
        result.data[3] = [0, 8, 3, 8]

        self.assertEqual(a.transpose(), result)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
