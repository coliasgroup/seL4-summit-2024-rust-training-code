#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

from harness import Simulation

with Simulation.from_args() as sim:
    sim.child.expect('server: initializing', timeout=1)
    sim.child.expect('server: region_a = 0x2000000', timeout=1)
    sim.child.expect('server: region_b = 0x2400000', timeout=1)
    sim.child.expect('client: initializing', timeout=1)
    sim.child.expect('client: region_a = 0x2000000', timeout=1)
    sim.child.expect('client: region_b = 0x2400000', timeout=1)
