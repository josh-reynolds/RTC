# to run tests: python -m unittest -v obj_files

import unittest

class ObjFileParser:
    def __init__(self, ignored):
        self.ignored = ignored

def parse_obj_file(file):
    return ObjFileParser(len(file))

class ObjFileTestCase(unittest.TestCase):
    def test_ignoring_unrecognized_lines(self):
        gibberish = ["There was a young lady named Bright\n",
                     "who traveled much faster than light.\n",
                     "She set out one day\n",
                     "in a relative way,\n",
                     "and came back the previous night.\n"]

        parser = parse_obj_file(gibberish)

        self.assertEqual(parser.ignored, 5)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
