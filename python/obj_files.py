# to run tests: python -m unittest -v obj_files

import unittest
import tuples

class ObjFileParser:
    def __init__(self):
        self.ignored = 0
        self.vertices = ['']   # vertex array needs to be 1-based, inserting dummy zero entry

def parse_obj_file(file):
    parser = ObjFileParser()
    for line in file:
        if line[0] == 'v':
            tokens = line.split()
            parser.vertices.append(tuples.point(float(tokens[1]), 
                                                float(tokens[2]), 
                                                float(tokens[3])))
        else:
            parser.ignored += 1

    return parser

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

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
