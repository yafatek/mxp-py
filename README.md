# MXP Python Bindings

[![CI](https://github.com/yafatek/mxp-py/workflows/CI/badge.svg)](https://github.com/yafatek/mxp-py/actions)
[![PyPI](https://img.shields.io/pypi/v/mxp.svg)](https://pypi.org/project/mxp/)
[![Python Versions](https://img.shields.io/pypi/pyversions/mxp.svg)](https://pypi.org/project/mxp/)

High-performance Python bindings for the [MXP (Mesh eXchange Protocol)](https://getmxp.xyz).

## Features

- ⚡ **Sub-millisecond latency** - Zero-copy message encoding/decoding
- 🔒 **Type-safe** - Full Python type hints
- 🚀 **Fast** - Rust implementation with Python bindings via PyO3
- 📦 **Easy** - Simple pip install, works on Linux, macOS, and Windows

## Installation

```bash
pip install mxp
```

## Quick Start

```python
import mxp

# Create a message
msg = mxp.Message(mxp.MessageType.Call, b"Hello, Agent!")

# Get message properties
print(f"Message ID: {msg.message_id}")
print(f"Trace ID: {msg.trace_id}")
print(f"Payload: {bytes(msg.payload)}")

# Encode to bytes (zero-copy)
encoded = msg.encode()

# Decode from bytes
decoded = mxp.Message.decode(encoded)
assert decoded.message_type == mxp.MessageType.Call
```

## Message Types

MXP supports the following message types:

- `AgentRegister` - Register agent with mesh
- `AgentDiscover` - Discover agents by capability
- `AgentHeartbeat` - Keep-alive / health check
- `Call` - Synchronous RPC call
- `Response` - Response to Call
- `Event` - Async event (fire-and-forget)
- `StreamOpen` - Open new stream
- `StreamChunk` - Stream data chunk
- `StreamClose` - Close stream
- `Ack` - Acknowledgment
- `Error` - Error response

## Performance

MXP is built for speed:

```python
import time
import mxp

# Benchmark encoding/decoding
msg = mxp.Message(mxp.MessageType.Call, b"x" * 1024)

start = time.perf_counter()
for _ in range(100000):
    encoded = msg.encode()
    decoded = mxp.Message.decode(encoded)
elapsed = time.perf_counter() - start

print(f"Throughput: {200000 / elapsed:.0f} ops/sec")
# Expected: >1M ops/sec
```

## Development

### Building from source

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Or build release wheel
maturin build --release
```

### Running tests

```bash
pytest tests/
```

## Protocol

MXP uses a 32-byte header + payload + 8-byte checksum format. For full protocol details, see [SPEC.md](https://github.com/yafatek/mxp-protocol/blob/main/SPEC.md).

## License

MIT OR Apache-2.0

## Links

- **MXP Protocol:** [getmxp.xyz](https://getmxp.xyz)
- **Protocol Spec:** [github.com/yafatek/mxp-protocol](https://github.com/yafatek/mxp-protocol)
- **Issues:** [github.com/yafatek/mxp-py/issues](https://github.com/yafatek/mxp-py/issues)
