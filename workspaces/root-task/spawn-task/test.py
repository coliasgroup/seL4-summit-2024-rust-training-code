#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

from harness import Simulation

with Simulation.from_args() as sim:
    sim.child.expect('In root task', timeout=1)
    sim.child.expect('In child task', timeout=1)
    sim.child.expect('TEST_PASS', timeout=1)
