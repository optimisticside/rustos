import argparse
import json
import os
import platform
import shutil
import subprocess
import sys
import tarfile

from typing import List


# Make sure all needed third-party libraries are installed.
try:
    import requests
    import xbstrap
    import chalk
except ImportError:
    print('The requests and xbstrap libraries are required', file=sys.stderr)
    sys.exit(1)

import requests
import xbstrap
import chalk


BUILD_DIR = 'build'

