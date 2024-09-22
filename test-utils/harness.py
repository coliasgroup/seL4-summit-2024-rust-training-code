#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

import argparse
import pexpect
import sys
import time

EXIT_TIMEOUT = .5

class Simulation:
    def __init__(self, child):
        self.child = child

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.flush_read(timeout=EXIT_TIMEOUT)

    @classmethod
    def from_args(cls):
        parser = argparse.ArgumentParser()
        parser.add_argument('cmd', nargs=argparse.REMAINDER)
        args = parser.parse_args()
        child = pexpect.spawn(args.cmd[0], args.cmd[1:], encoding='utf-8')
        child.logfile = sys.stdout
        return cls(child)

    def flush_read(self, timeout=0):
        while True:
            try:
                self.child.read_nonblocking(timeout=timeout)
            except pexpect.TIMEOUT:
                break

    def simple_test(self, timeout=20):
        self.child.expect('TEST_PASS', timeout=timeout)
