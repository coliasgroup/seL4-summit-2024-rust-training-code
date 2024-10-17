#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

from harness import Simulation

with Simulation.from_args() as sim:
    sim.child.expect('server: initializing', timeout=1)
    sim.child.expect('server: region_a = VolatileRef', timeout=1)
    sim.child.expect('server: region_b = VolatileRef', timeout=1)
    sim.child.expect('client: initializing', timeout=1)
    sim.child.expect('client: region_a = VolatileRef', timeout=1)
    sim.child.expect('client: region_b = VolatileRef', timeout=1)
