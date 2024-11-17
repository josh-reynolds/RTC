# to run tests: python -m unittest -v matrix

import unittest
#import math

#EPSILON = 0.00001

#def flequal(a, b):
    #if abs(a - b) < EPSILON:
        #return True
    #return False

class Matrix():
    def __init__(self, rows, columns):
        self.rows = rows
        self.columns = columns
        self.data = [[0 for x in range(columns)] for x in range(rows)]

    #def __eq__(self, other):
        #if isinstance(other, self.__class__):
            #return flequal(self.x, other.x) and \
                   #flequal(self.y, other.y) and \
                   #flequal(self.z, other.z) and \
                   #flequal(self.w, other.w)
        #else:
            #return False

    #def __ne__(self, other):
        #return not self.__eq__(other)

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

    #def __mul__(self, rhs):
        #if isinstance(rhs, float) or isinstance(rhs, int):
            #return Tuple(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)

    #def __rmul__(self, lhs):
        #return self.__mul__(lhs)

    #def __truediv__(self, rhs):
        #if isinstance(rhs, float) or isinstance(rhs, int):
            #return Tuple(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)

#    def __getitem__(self, w):
#        return self.pixels[w]

    def __str__(self):
        result = ""
        for x in range(self.rows):
            for y in range(self.columns):
                result += str(self.data[x][y])
            result += "\n"
        return result

def matrix(rows=4, columns=4):
    return Matrix(rows, columns)

class MatrixTestCase(unittest.TestCase):
    def test_matrix_creation(self):
        m = matrix()
        m.data[0] = [  1,   2,   3,   4]
        m.data[1] = [5.5, 6.5, 7.5, 8.5]
        m.data[2] = [  9,  10,  11,  12]
        m.data[3] = [13.5,   14.5,  15.5,  16.5]
        self.assertEqual(m.data[0][0],    1)
        self.assertEqual(m.data[0][3],    4)
        self.assertEqual(m.data[1][0],  5.5)
        self.assertEqual(m.data[1][2],  7.5)
        self.assertEqual(m.data[2][2],   11)
        self.assertEqual(m.data[3][0], 13.5)
        self.assertEqual(m.data[3][2], 15.5)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
