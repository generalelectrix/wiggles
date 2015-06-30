"""Generation and control of not-before-seen wiggles.
"""

from setuptools import setup

doclines = __doc__.split('\n')

setup(name='wiggles',
      version='0.1',
      description = doclines[0],
      long_description = '\n'.join(doclines[2:]),
      url='http://github.com/generalelectrix/wiggles',
      author='Chris Macklin <chris@imaginaryphotons.com>, Josh Erickson <ponderosa@imaginaryphotons.com>',
      license='GPL2',
      packages=['wiggles'],
      install_requires=['pysimpledmx', 'numpy', 'pyqtgraph']
      )