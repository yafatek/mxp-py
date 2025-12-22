# MXP Python SDK Comparisons

This SDK provides MXP protocol support in Python. It complements agent frameworks and orchestration tools.

## MXP vs JSON over HTTP

- Binary encoding reduces overhead and latency.
- Built-in tracing fields travel with every message.
- Native streaming fits token-based responses better than polling.

## MXP vs WebSocket JSON

- Strongly typed message headers and checksums.
- Wire-compatible across languages.

## Using with agent frameworks

Use MXP for transport and interoperability, and use your framework for orchestration:

- Framework handles prompts and tools.
- MXP Python handles encoding, decoding, and transport.
