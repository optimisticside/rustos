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


def build_cargo_workspace(directory, command, args, cargo="cargo"):
    code, _, _ = run_command([cargo, command, *args], cwd=directory)

    if code != 0:
        return None

    _, stdout, _ = run_command(
        [cargo, command, *args, "--message-format=json"],
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        cwd=directory,
    )


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

    if args.features:
        command_args += ["--features", ",".join(args.features)]

    return build_cargo_workspace("src", command, command_args)


def parse_args():
    parser = argparse.ArgumentParser(
        description="Builds the RusTOS kernel and userland"
    )

    check_test = parser.add_mutually_exclusive_group()

    check_test.add_argument(
        "--clean",
        default=False,
        action="store_true",
        help="removes the build artifacts",
    )

    check_test.add_argument(
        "--test", default=False, action="store_true", help="runs the test suite"
    )

    check_test.add_argument(
        "--document",
        default=False,
        action="store_true",
        help="generates the documentation for the kernel",
    )

    parser.add_argument(
        "--debug",
        default=False,
        action="store_true",
        help="builds the kernel and userland in debug mode",
    )

    parser.add_argument(
        "--only-run",
        default=False,
        action="store_true",
        help="runs RusTOS without re-building"
    )

    return parser.parse_args()


def main():
    args = parse_args()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
