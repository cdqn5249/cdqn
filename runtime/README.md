<!-- BaDaaS License: This document is governed by the BaDaaS license. -->

# /runtime

This directory represents the **operational environment** of a CDQN node.

It cleanly separates the project's source code from its compiled, running form. This directory contains the executables, configuration, and live data required for a node to function.

### Components:
*   `/bin` - Compiled binaries, like the `cdqnRuntime` executable.
*   `/config` - Node configuration files.
*   `/data` - Persistent state, such as the HLC state.
*   `/logs` - Runtime logs.
*   `/exchange` - The live message exchange channel.
