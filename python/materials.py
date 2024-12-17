# to run tests: python -m unittest -v materials

import unittest
import math
from colors import color, BLACK, WHITE
from utils import flequal
from lights import point_light
from tuples import point, vector
import spheres
import stripes

# sample refractive indices:
#  vacuum  - 1
#  air     - 1.00029
#  water   - 1.333
#  glass   - 1.52
#  diamond - 2.417

class Material:
    def __init__(self):
        self.color = WHITE
        self.ambient = 0.1        # range 0 - 1
        self.diffuse = 0.9        # range 0 - 1
        self.specular = 0.9       # range 0 - 1
        self.shininess = 200.0    # range 10 - 200
        self.pattern = None
        self.reflective = 0.0
        self.transparency = 0.0
        self.refractive_index = 1.0

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return ((self.color == other.color) and
                    flequal(self.ambient, other.ambient) and
                    flequal(self.diffuse, other.diffuse) and
                    flequal(self.specular, other.specular) and
                    flequal(self.shininess, other.shininess))
        else:
            return False

def material():
    return Material()

def lighting(material, obj, light, position, eyev, normalv, in_shadow):
    if material.pattern:
        col = material.pattern.pattern_at_shape(obj, position)
    else:
        col = material.color

    effective_color = col * light.intensity
    lightv = (light.position - position).normalize()
    ambient = effective_color * material.ambient

    light_dot_normal = lightv.dot(normalv)
    if light_dot_normal < 0:
        diffuse = BLACK
        specular = BLACK
    else:
        diffuse = effective_color * material.diffuse * light_dot_normal
        reflectv = (-lightv).reflect(normalv)
        reflect_dot_eye = reflectv.dot(eyev)

        if reflect_dot_eye < 0:
            specular = BLACK
        else:
            factor = reflect_dot_eye ** material.shininess
            specular = light.intensity * material.specular * factor

    if in_shadow:
        return ambient
    else:
        return ambient + diffuse + specular


class MaterialTestCase(unittest.TestCase):
    def test_the_default_material(self):
        m = material()

        self.assertEqual(m.color, WHITE)
        self.assertEqual(m.ambient, 0.1)
        self.assertEqual(m.diffuse, 0.9)
        self.assertEqual(m.specular, 0.9)
        self.assertEqual(m.shininess, 200.0)

    def test_lighting_with_eye_between_light_and_surface(self):
        m = material()
        light = point_light(point(0, 0, -10), WHITE)
        position = point(0, 0, 0)
        eyev = vector(0, 0, -1)
        normalv = vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, color(1.9, 1.9, 1.9))

    def test_lighting_with_eye_between_light_and_surface_and_offset_45_degrees(self):
        m = material()
        light = point_light(point(0, 0, -10), WHITE)
        position = point(0, 0, 0)
        eyev = vector(0, math.sqrt(2)/2, -math.sqrt(2)/2)
        normalv = vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, WHITE)

    def test_lighting_with_eye_opposite_surface_and_light_offset_45_degrees(self):
        m = material()
        light = point_light(point(0, 10, -10), WHITE)
        position = point(0, 0, 0)
        eyev = vector(0, 0, -1)
        normalv = vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, color(0.7364, 0.7364, 0.7364))

    def test_lighting_with_eye_in_path_of_reflection_vector(self):
        m = material()
        light = point_light(point(0, 10, -10), WHITE)
        position = point(0, 0, 0)
        eyev = vector(0, -math.sqrt(2)/2, -math.sqrt(2)/2)
        normalv = vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, color(1.6364, 1.6364, 1.6364))

    def test_lighting_with_light_behind_surface(self):
        m = material()
        light = point_light(point(0, 0, 10), WHITE)
        position = point(0, 0, 0)
        eyev = vector(0, 0, -1)
        normalv = vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, color(0.1, 0.1, 0.1))

    def test_lighting_with_surface_in_shadow(self):
        m = material()
        light = point_light(point(0, 0, -10), WHITE)
        position = point(0, 0, 0)
        eyev = vector(0, 0, -1)
        normalv = vector(0, 0, -1)
        in_shadow = True

        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, in_shadow)

        self.assertEqual(result, color(0.1, 0.1, 0.1))

    def test_lighting_with_a_pattern_applied(self):
        m = material()
        m.pattern = stripes.stripe_pattern(WHITE, BLACK)
        m.ambient = 1
        m.diffuse = 0
        m.specular = 0

        light = point_light(point(0, 0, -10), WHITE)
        eyev = vector(0, 0, -1)
        normalv = vector(0, 0, -1)

        c1 = lighting(m, spheres.sphere(), light, point(0.9, 0, 0), eyev, normalv, False)
        c2 = lighting(m, spheres.sphere(), light, point(1.1, 0, 0), eyev, normalv, False)

        self.assertEqual(c1, WHITE)
        self.assertEqual(c2, BLACK)

    def test_reflectivity_for_default_material(self):
        m = material()

        self.assertEqual(m.reflective, 0.0)

    def test_transparency_and_refractive_index_for_default_material(self):
        m = material()

        self.assertEqual(m.transparency, 0.0)
        self.assertEqual(m.refractive_index, 1.0)

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    unittest.main()
