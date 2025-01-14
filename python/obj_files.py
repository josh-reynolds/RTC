# to run tests: python -m unittest -v obj_files

import unittest
import tuples
import groups
import triangles
import smooth_triangles

class ObjFileParser:
    def __init__(self):
        self.ignored = 0
        self.vertices = ['']   # vertex array needs to be 1-based, inserting dummy zero entry
        self.normals = ['']    # same here, needs dummy entry
        self.default_group = groups.group()

    def obj_to_group(self):
        return self.default_group

def parse_obj_file(file):
    parser = ObjFileParser()
    current_group = parser.default_group
    for line in file:
        tokens = line.split()
        if tokens and tokens[0] == 'v':
            parser.vertices.append(tuples.point(float(tokens[1]), 
                                                float(tokens[2]), 
                                                float(tokens[3])))
        elif tokens and tokens[0] == 'vn':
            parser.normals.append(tuples.vector(float(tokens[1]),
                                                float(tokens[2]),
                                                float(tokens[3])))
        elif tokens and tokens[0] == 'f' and len(tokens) >= 4:
            vertices = ['']    # need dummy entry here too
            normals  = ['']    # and here...
            if '/' in line:
                for token in tokens[1:]:
                    values = token.split('/')
                    vertices.append(parser.vertices[int(values[0])])
                    # by design, parser ignores texture data (values[1]), even if present
                    normals.append(parser.normals[int(values[2])])
            else:
                for token in tokens[1:]:
                    vertices.append(parser.vertices[int(token)])
    
            if len(tokens) == 4:
                if len(normals) > 1:
                    tri = smooth_triangles.smooth_triangle(vertices[1], vertices[2], vertices[3],
                                                           normals[1], normals[2], normals[3])
                else:
                    tri = triangles.triangle(vertices[1], vertices[2], vertices[3])
                current_group.add_child(tri)
            else:
                tris = fan_triangulate(vertices, normals)
                for tri in tris:
                    current_group.add_child(tri)
        elif tokens and tokens[0] == 'g':
            new_group = groups.group()     # assumption: groups are contiguous blocks in OBJ file
            current_group = new_group      # also assuming grouping is only one level deep
            parser.default_group.add_child(current_group)
        else:
            parser.ignored += 1

    return parser

def fan_triangulate(vertices, normals):
    tris = []

    for i in range(2, len(vertices)-1):
        if len(normals) > 1:
            tri = smooth_triangles.smooth_triangle(vertices[1], vertices[i], vertices[i+1],
                                                   normals[1], normals[i], normals[i+1])
        else:
            tri = triangles.triangle(vertices[1], vertices[i], vertices[i+1])
        tris.append(tri)

    return tris

class ObjFileTestCase(unittest.TestCase):
    def test_ignoring_unrecognized_lines(self):
        gibberish = ["There was a young lady named Bright\n",
                     "who traveled much faster than light.\n",
                     "She set out one day\n",
                     "in a relative way,\n",
                     "and came back the previous night.\n"]

        parser = parse_obj_file(gibberish)

        self.assertEqual(parser.ignored, 5)

    def test_vertex_records(self):
        file = ["v -1 1 0\n",
                "v -1.0000 0.5000 0.0000\n",
                "v 1 0 0\n",
                "v 1 1 0\n"]

        parser = parse_obj_file(file)

        self.assertEqual(parser.vertices[1], tuples.point(-1, 1, 0))
        self.assertEqual(parser.vertices[2], tuples.point(-1, 0.5, 0))
        self.assertEqual(parser.vertices[3], tuples.point(1, 0, 0))
        self.assertEqual(parser.vertices[4], tuples.point(1, 1, 0))

    def test_parsing_triangle_faces(self):
        file = ["v -1 1 0\n",
                "v -1 0 0\n",
                "v 1 0 0\n",
                "v 1 1 0\n",
                "\n",
                "f 1 2 3\n",
                "f 1 3 4\n"]

        parser = parse_obj_file(file)
        g = parser.default_group
        t1 = g.contents[0]
        t2 = g.contents[1]

        self.assertEqual(t1.p1, parser.vertices[1])
        self.assertEqual(t1.p2, parser.vertices[2])
        self.assertEqual(t1.p3, parser.vertices[3])
        self.assertEqual(t2.p1, parser.vertices[1])
        self.assertEqual(t2.p2, parser.vertices[3])
        self.assertEqual(t2.p3, parser.vertices[4])

    def test_triangulating_polygons(self):
        file = ["v -1 1 0\n",
                "v -1 0 0\n",
                "v 1 0 0\n",
                "v 1 1 0\n",
                "v 0 2 0\n",
                "\n",
                "f 1 2 3 4 5\n"]

        parser = parse_obj_file(file)
        g = parser.default_group
        t1 = g.contents[0]
        t2 = g.contents[1]
        t3 = g.contents[2]

        self.assertEqual(t1.p1, parser.vertices[1])
        self.assertEqual(t1.p2, parser.vertices[2])
        self.assertEqual(t1.p3, parser.vertices[3])
        self.assertEqual(t2.p1, parser.vertices[1])
        self.assertEqual(t2.p2, parser.vertices[3])
        self.assertEqual(t2.p3, parser.vertices[4])
        self.assertEqual(t3.p1, parser.vertices[1])
        self.assertEqual(t3.p2, parser.vertices[4])
        self.assertEqual(t3.p3, parser.vertices[5])

    def test_triangles_in_groups(self):
        file = ["v -1 1 0",
                "v -1 0 0",
                "v 1 0 0",
                "v 1 1 0",
                "",
                "g FirstGroup",
                "f 1 2 3",
                "g SecondGroup",
                "f 1 3 4"]

        parser = parse_obj_file(file)
        g = parser.default_group
        g1 = g.contents[0]
        g2 = g.contents[1]
        t1 = g1.contents[0]
        t2 = g2.contents[0]

        self.assertEqual(t1.p1, parser.vertices[1])
        self.assertEqual(t1.p2, parser.vertices[2])
        self.assertEqual(t1.p3, parser.vertices[3])
        self.assertEqual(t2.p1, parser.vertices[1])
        self.assertEqual(t2.p2, parser.vertices[3])
        self.assertEqual(t2.p3, parser.vertices[4])

    def test_triangles_in_groups(self):
        file = ["v -1 1 0",
                "v -1 0 0",
                "v 1 0 0",
                "v 1 1 0",
                "",
                "g FirstGroup",
                "f 1 2 3",
                "g SecondGroup",
                "f 1 3 4"]

        parser = parse_obj_file(file)
        g1 = parser.default_group.contents[0]
        g2 = parser.default_group.contents[1]

        g = parser.obj_to_group()

        self.assertEqual(g.contents[0], g1)
        self.assertEqual(g.contents[1], g2)

    def test_vertex_normal_records(self):
        file = ["vn 0 0 1",
                "vn 0.707 0 -0.707",
                "vn 1 2 3"]

        parser = parse_obj_file(file)

        self.assertEqual(parser.normals[1], tuples.vector(0, 0, 1))
        self.assertEqual(parser.normals[2], tuples.vector(0.707, 0, -0.707))
        self.assertEqual(parser.normals[3], tuples.vector(1, 2, 3))

    def test_faces_with_normals(self):
        file = ["v 0 1 0",
                "v -1 0 0",
                "v 1 0 0",
                "",
                "vn -1 0 0",
                "vn 1 0 0",
                "vn 0 1 0",
                "",
                "f 1//3 2//1 3//2",
                "f 1/0/3 2/102/1 3/14/2"]

        parser = parse_obj_file(file)
        g = parser.default_group
        t1 = g.contents[0]
        t2 = g.contents[1]

        self.assertEqual(t1.p1, parser.vertices[1])
        self.assertEqual(t1.p2, parser.vertices[2])
        self.assertEqual(t1.p3, parser.vertices[3])
        self.assertEqual(t1.n1, parser.normals[3])
        self.assertEqual(t1.n2, parser.normals[1])
        self.assertEqual(t1.n3, parser.normals[2])
        self.assertEqual(t2, t1)


# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
