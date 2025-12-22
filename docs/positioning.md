# MXP Python SDK Positioning

MXP Python SDK brings the MXP Protocol to Python. It targets teams who want protocol-native agent messaging without rewriting everything in Rust.

## Why we built it

- Python is the most common language for agent workflows.
- We need a binary, traceable, wire-compatible protocol in Python.
- We want parity with the Rust core to enable cross-language agents.

## Who it is for

- Python services that need fast agent messaging.
- Teams experimenting with agent systems in Python.
- Integrations that need to speak MXP without a Rust dependency.

## How to build full agents

Use the MXP Agents Runtime for a production-ready runtime that speaks MXP end-to-end:

- https://github.com/yafatek/mxp-agents-runtime
