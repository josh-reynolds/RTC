import unittest

def suite():
    suite = unittest.TestSuite()
    loader = unittest.TestLoader()
    suite.addTests(loader.loadTestsFromName('cameras'))
    suite.addTests(loader.loadTestsFromName('canvases'))
    suite.addTests(loader.loadTestsFromName('checkers'))
    suite.addTests(loader.loadTestsFromName('colors'))
    suite.addTests(loader.loadTestsFromName('gradients'))
    suite.addTests(loader.loadTestsFromName('intersections'))
    suite.addTests(loader.loadTestsFromName('lights'))
    suite.addTests(loader.loadTestsFromName('materials'))
    suite.addTests(loader.loadTestsFromName('matrices'))
    suite.addTests(loader.loadTestsFromName('patterns'))
    suite.addTests(loader.loadTestsFromName('planes'))
    suite.addTests(loader.loadTestsFromName('rays'))
    suite.addTests(loader.loadTestsFromName('rings'))
    suite.addTests(loader.loadTestsFromName('shapes'))
    suite.addTests(loader.loadTestsFromName('spheres'))
    suite.addTests(loader.loadTestsFromName('stripes'))
    suite.addTests(loader.loadTestsFromName('transformations'))
    suite.addTests(loader.loadTestsFromName('tuples'))
    suite.addTests(loader.loadTestsFromName('utils'))
    suite.addTests(loader.loadTestsFromName('worlds'))
    return suite

# ---------------------------------------------------------------------------
if __name__ == '__main__':
    runner = unittest.TextTestRunner()
    runner.run(suite())



