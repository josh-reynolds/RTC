# to run tests: python -m unittest -v obj_files

import unittest
import tuples
import groups
import triangles

class ObjFileParser:
    def __init__(self):
        self.ignored = 0
        self.vertices = ['']   # vertex array needs to be 1-based, inserting dummy zero entry
        self.default_group = groups.group()

def parse_obj_file(file):
    parser = ObjFileParser()
    for line in file:
        tokens = line.split()
        if tokens and tokens[0] == 'v':
            parser.vertices.append(tuples.point(float(tokens[1]), 
                                                float(tokens[2]), 
                                                float(tokens[3])))
        elif tokens and tokens[0] == 'f' and len(tokens) >= 4:
            vertices = ['']    # need dummy entry here too
            for token in tokens[1:]:
                vertices.append(parser.vertices[int(token)])

            if len(tokens) == 4:
                tri = triangles.triangle(vertices[1], vertices[2], vertices[3])
                parser.default_group.add_child(tri)
            else:
                tris = fan_triangulate(vertices)
                for tri in tris:
                    parser.default_group.add_child(tri)
        else:
            parser.ignored += 1

    return parser

def fan_triangulate(vertices):
    tris = []

    for i in range(2, len(vertices)-1):
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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
