#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

import time

from harness import Simulation

with Simulation.from_args() as sim:
    sim.child.expect('Hello, World!', timeout=1)
    sim.child.expect('echo> ', timeout=1)
    time.sleep(1)
    sim.child.send('abc')
    sim.child.expect('\[a\]\[b\]\[c\]', timeout=5)
    print()
