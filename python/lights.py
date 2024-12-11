# to run tests: python -m unittest -v lights

import unittest
from tuples import point
from colors import color

class Light:
    def __init__(self, position, intensity):
        self.position = position
        self.intensity = intensity

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return (self.position == other.position and
                    self.intensity == other.intensity)
        else:
            return False

def point_light(position, intensity):
    return Light(position, intensity)

class LightsTestCase(unittest.TestCase):
    def test_a_point_light_has_a_position_and_intensity(self):
        position = point(0, 0, 0)
        intensity = color(1, 1, 1)
        
        light = point_light(position, intensity)

        self.assertEqual(light.position, point(0, 0, 0))
        self.assertEqual(light.intensity, color(1, 1, 1))

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
