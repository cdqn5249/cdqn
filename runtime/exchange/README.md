<!-- BaDaaS License: This document is governed by the BaDaaS license. -->

# /runtime/exchange

This directory is the live, asynchronous message channel for the CDQN node. It functions as the primary inbox/outbox for all KDU-based communication.

*   **`incoming-*.kdu`** files are created by clients and pushed here to be processed by the `kdu-handler`.
*   **`response-*.kdu`** files are created by the `kdu-handler` and pushed here for clients to retrieve.

This directory is automatically purged by a housekeeping process to prevent clutter.
