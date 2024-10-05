#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

from harness import Simulation

with Simulation.from_args() as sim:
    sim.child.expect('In primary thread', timeout=1)
    sim.child.expect('In secondary thread', timeout=1)
    sim.child.expect('TEST_PASS', timeout=1)
