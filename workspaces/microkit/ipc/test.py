#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

from harness import Simulation

with Simulation.from_args() as sim:
    sim.child.expect('client: initializing', timeout=1)
    sim.child.expect('server: initializing', timeout=1)
    sim.child.expect('client: TEST_PASS', timeout=1)
