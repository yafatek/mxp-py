#!/usr/bin/env python3
"""
MXP Python SDK - High-performance protocol for agent-to-agent communication.

Example:
    >>> import mxp
    >>> agent = mxp.Agent.connect("mxp://localhost:9000")
    >>> await agent.register("my-agent", capabilities=["chat", "compute"])
"""

__version__ = "0.1.0"

from .mxp import (
    Message,
    MessageType,
    Agent,
    Transport,
    MXPError,
)

__all__ = [
    "Message",
    "MessageType", 
    "Agent",
    "Transport",
    "MXPError",
]
