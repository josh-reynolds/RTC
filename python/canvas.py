# to run tests: python -m unittest -v color

import unittest
from color import color

class Canvas():
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.pixels = [[color(0,0,0) for x in range(width)] for x in range(height)]

    def write_pixel(self, x, y, color):
        self.pixels[y][x] = color

    def pixel_at(self, x, y):
        return self.pixels[y][x]

    def to_ppm(self):
        result = []
        result.append("P3")
        result.append("{0} {1}".format(self.width, self.height))
        result.append("255")
        for row in self.pixels:
            line = ""
            for pixel in row:
                line += str(clamp(pixel.red)) + " "
                line += str(clamp(pixel.green)) + " "
                line += str(clamp(pixel.blue)) + " "
            if len(line) <= 70:          # 70 is max line length for some PPM readers
                result.append(line)
            else:
               result += splitline(line)
        result.append("\n")
        return result

    def __str__(self):
        result = ""
        for x in range(self.height):
            for y in range(self.width):
                result += str(self.pixels[x][y])
            result += "\n"
        return result

#    def __getitem__(self, w):
#        return self.pixels[w]

def canvas(width, height):
    return Canvas(width, height)

def clamp(value):
    result = round(value * 255)
    return max(0, min(255, result))

def splitline(line, length=70, separator=" ", chunksize=3):
    result = []
    index = line.find(separator, length - chunksize, length) + 1
    result.append(line[:index])
    if len(line[index:]) > length:
        result += splitline(line[index:])
    else:
        result.append(line[index:])
    return result


class CanvasTestCase(unittest.TestCase):
    def test_creating_a_canvas(self):
        c = canvas(10, 20)
        self.assertEqual(c.width, 10)
        self.assertEqual(c.height, 20)

        default = color(0,0,0)
        for row in c.pixels:
            for pixel in row:
                self.assertEqual(pixel, default)

    def test_writing_pixels_to_canvas(self):
        c = canvas(10, 20)
        red = color(1, 0, 0)
        c.write_pixel(2, 3, red)
        self.assertEqual(c.pixel_at(2, 3), red)

    def test_constructing_ppm_header(self):
        c = canvas(5, 3)
        ppm = c.to_ppm()
        self.assertEqual(ppm[0], "P3")
        self.assertEqual(ppm[1], "5 3")
        self.assertEqual(ppm[2], "255")

    def test_clamp_constrains_pixel_values(self):
        n1 = clamp(1.5)
        self.assertEqual(n1, 255)
        n2 = clamp(1)
        self.assertEqual(n2, 255)
        n3 = clamp(0.5)
        self.assertEqual(n3, 128)
        n4 = clamp(0)
        self.assertEqual(n4, 0)
        n5 = clamp(-0.5)
        self.assertEqual(n5, 0)

    def test_constructing_ppm_pixel_data(self):
        c = canvas(5, 3)
        c.write_pixel(0, 0, color(1.5, 0, 0))
        c.write_pixel(2, 1, color(0, 0.5, 0))
        c.write_pixel(4, 2, color(-0.5, 0, 1))
        ppm = c.to_ppm()
        self.assertEqual(len(ppm), 7)
        self.assertEqual(ppm[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ")
        self.assertEqual(ppm[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 ")
        self.assertEqual(ppm[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 ")

    def test_splitting_long_lines_in_ppm_data(self):
        c = canvas(10, 2)
        col = color(1, 0.8, 0.6)
        for x in range(10):
            for y in range(2):
                c.write_pixel(x, y, col)
        ppm = c.to_ppm()
        self.assertEqual(len(ppm), 8)
        self.assertEqual(ppm[3], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 ")
        self.assertEqual(ppm[4], "153 255 204 153 255 204 153 255 204 153 255 204 153 ")
        self.assertEqual(ppm[5], "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 ")
        self.assertEqual(ppm[6], "153 255 204 153 255 204 153 255 204 153 255 204 153 ")

    def test_ppm_terminated_by_newline(self):
        c = canvas(5, 3)
        ppm = c.to_ppm()
        self.assertEqual(len(ppm), 7)
        self.assertEqual(ppm[6], "\n")


# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
