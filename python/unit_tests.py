import unittest

def suite():
    suite = unittest.TestSuite()
    loader = unittest.TestLoader()
    suite.addTests(loader.loadTestsFromName('canvas'))
    suite.addTests(loader.loadTestsFromName('color'))
    suite.addTests(loader.loadTestsFromName('intersections'))
    suite.addTests(loader.loadTestsFromName('matrix'))
    suite.addTests(loader.loadTestsFromName('rays'))
    suite.addTests(loader.loadTestsFromName('spheres'))
    suite.addTests(loader.loadTestsFromName('transformations'))
    suite.addTests(loader.loadTestsFromName('tuple'))
    return suite

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    runner = unittest.TextTestRunner()
    runner.run(suite())



