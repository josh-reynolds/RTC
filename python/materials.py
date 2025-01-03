# to run tests: python -m unittest -v materials

import unittest
import math
import colors
import utils
import lights
import tuples
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
        self.color = colors.WHITE
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
                    utils.flequal(self.ambient, other.ambient) and
                    utils.flequal(self.diffuse, other.diffuse) and
                    utils.flequal(self.specular, other.specular) and
                    utils.flequal(self.shininess, other.shininess))
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
        diffuse = colors.BLACK
        specular = colors.BLACK
    else:
        diffuse = effective_color * material.diffuse * light_dot_normal
        reflectv = (-lightv).reflect(normalv)
        reflect_dot_eye = reflectv.dot(eyev)

        if reflect_dot_eye < 0:
            specular = colors.BLACK
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

        self.assertEqual(m.color, colors.WHITE)
        self.assertEqual(m.ambient, 0.1)
        self.assertEqual(m.diffuse, 0.9)
        self.assertEqual(m.specular, 0.9)
        self.assertEqual(m.shininess, 200.0)

    def test_lighting_with_eye_between_light_and_surface(self):
        m = material()
        light = lights.point_light(tuples.point(0, 0, -10), colors.WHITE)
        position = tuples.point(0, 0, 0)
        eyev = tuples.vector(0, 0, -1)
        normalv = tuples.vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, colors.color(1.9, 1.9, 1.9))

    def test_lighting_with_eye_between_light_and_surface_and_offset_45_degrees(self):
        m = material()
        light = lights.point_light(tuples.point(0, 0, -10), colors.WHITE)
        position = tuples.point(0, 0, 0)
        eyev = tuples.vector(0, math.sqrt(2)/2, -math.sqrt(2)/2)
        normalv = tuples.vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, colors.WHITE)

    def test_lighting_with_eye_opposite_surface_and_light_offset_45_degrees(self):
        m = material()
        light = lights.point_light(tuples.point(0, 10, -10), colors.WHITE)
        position = tuples.point(0, 0, 0)
        eyev = tuples.vector(0, 0, -1)
        normalv = tuples.vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, colors.color(0.7364, 0.7364, 0.7364))

    def test_lighting_with_eye_in_path_of_reflection_vector(self):
        m = material()
        light = lights.point_light(tuples.point(0, 10, -10), colors.WHITE)
        position = tuples.point(0, 0, 0)
        eyev = tuples.vector(0, -math.sqrt(2)/2, -math.sqrt(2)/2)
        normalv = tuples.vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, colors.color(1.6364, 1.6364, 1.6364))

    def test_lighting_with_light_behind_surface(self):
        m = material()
        light = lights.point_light(tuples.point(0, 0, 10), colors.WHITE)
        position = tuples.point(0, 0, 0)
        eyev = tuples.vector(0, 0, -1)
        normalv = tuples.vector(0, 0, -1)
        
        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, False)

        self.assertEqual(result, colors.color(0.1, 0.1, 0.1))

    def test_lighting_with_surface_in_shadow(self):
        m = material()
        light = lights.point_light(tuples.point(0, 0, -10), colors.WHITE)
        position = tuples.point(0, 0, 0)
        eyev = tuples.vector(0, 0, -1)
        normalv = tuples.vector(0, 0, -1)
        in_shadow = True

        result = lighting(m, spheres.sphere(), light, position, eyev, normalv, in_shadow)

        self.assertEqual(result, colors.color(0.1, 0.1, 0.1))

    def test_lighting_with_a_pattern_applied(self):
        m = material()
        m.pattern = stripes.stripe_pattern(colors.WHITE, colors.BLACK)
        m.ambient = 1
        m.diffuse = 0
        m.specular = 0

        light = lights.point_light(tuples.point(0, 0, -10), colors.WHITE)
        eyev = tuples.vector(0, 0, -1)
        normalv = tuples.vector(0, 0, -1)

        c1 = lighting(m, spheres.sphere(), light, tuples.point(0.9, 0, 0), eyev, normalv, False)
        c2 = lighting(m, spheres.sphere(), light, tuples.point(1.1, 0, 0), eyev, normalv, False)

        self.assertEqual(c1, colors.WHITE)
        self.assertEqual(c2, colors.BLACK)

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
