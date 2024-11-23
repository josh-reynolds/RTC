from matrix import matrix

a = matrix()
a.data[0] = [ 3, -9,  7,  3]
a.data[1] = [ 3, -8,  2, -9]
a.data[2] = [-4,  4,  4,  1]
a.data[3] = [-6,  5, -1,  1]

b = matrix()
b.data[0] = [ 8,  2,  2,  2]
b.data[1] = [ 3, -1,  7,  0]
b.data[2] = [ 7,  0,  5,  4]
b.data[3] = [ 6, -2,  0,  5]

c = a * b
