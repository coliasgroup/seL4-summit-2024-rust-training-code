#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

from harness import Simulation

with Simulation.from_args() as sim:
    sim.child.expect('Hello, World!', timeout=3)
    sim.child.expect('panicked', timeout=1)
    sim.child.expect('uh oh', timeout=1)
    sim.child.expect('TEST_PASS', timeout=1)
