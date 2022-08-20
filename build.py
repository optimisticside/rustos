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
    print("The requests and xbstrap libraries are required", file=sys.stderr)
    sys.exit(1)

import requests
import xbstrap
import chalk


BUILD_DIR = "build"


def run_command(args, **kwargs):
    """Runs a command and returns the command's output, along with its standard-out and
    standard-error files.
    """
    output = subprocess.run(args, **kwargs)
    return output.returncode, output.stdout, output.stderr


def build_kernel(args):
    command = "build"
    command_args = ["--package", "kernel", "--target", f".cargo/{args.target}.json"]

    if not args.debug:
        command_args.append("release")

    if args.test:
        command = "test"
        command_args.append("--no-run")
    elif args.check:
        command = "check"
    elif args.document:
        command = "doc"

    return build


def main():
    pass


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
