//! PyO3 bindings for MXP protocol
//!
//! This library exposes the MXP Rust implementation to Python via PyO3.

use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use pyo3::types::PyBytes;

// Re-export the core MXP types using :: prefix to avoid ambiguity
use ::mxp::{Message as RustMessage, MessageType as RustMessageType};

/// Python exception for MXP errors
pyo3::create_exception!(mxp, MXPError, PyException);

/// Message types available in MXP protocol
#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageType {
    AgentRegister = 0x01,
    AgentDiscover = 0x02,
    AgentHeartbeat = 0x03,
    Call = 0x10,
    Response = 0x11,
    Event = 0x12,
    StreamOpen = 0x20,
    StreamChunk = 0x21,
    StreamClose = 0x22,
    Ack = 0xF0,
    Error = 0xF1,
}

impl From<MessageType> for RustMessageType {
    fn from(msg_type: MessageType) -> Self {
        match msg_type {
            MessageType::AgentRegister => RustMessageType::AgentRegister,
            MessageType::AgentDiscover => RustMessageType::AgentDiscover,
            MessageType::AgentHeartbeat => RustMessageType::AgentHeartbeat,
            MessageType::Call => RustMessageType::Call,
            MessageType::Response => RustMessageType::Response,
            MessageType::Event => RustMessageType::Event,
            MessageType::StreamOpen => RustMessageType::StreamOpen,
            MessageType::StreamChunk => RustMessageType::StreamChunk,
            MessageType::StreamClose => RustMessageType::StreamClose,
            MessageType::Ack => RustMessageType::Ack,
            MessageType::Error => RustMessageType::Error,
        }
    }
}

impl From<RustMessageType> for MessageType {
    fn from(msg_type: RustMessageType) -> Self {
        match msg_type {
            RustMessageType::AgentRegister => MessageType::AgentRegister,
            RustMessageType::AgentDiscover => MessageType::AgentDiscover,
            RustMessageType::AgentHeartbeat => MessageType::AgentHeartbeat,
            RustMessageType::Call => MessageType::Call,
            RustMessageType::Response => MessageType::Response,
            RustMessageType::Event => MessageType::Event,
            RustMessageType::StreamOpen => MessageType::StreamOpen,
            RustMessageType::StreamChunk => MessageType::StreamChunk,
            RustMessageType::StreamClose => MessageType::StreamClose,
            RustMessageType::Ack => MessageType::Ack,
            RustMessageType::Error => MessageType::Error,
        }
    }
}

/// MXP Message
#[pyclass]
pub struct Message {
    inner: RustMessage,
}

#[pymethods]
impl Message {
    /// Create a new message
    #[new]
    #[pyo3(signature = (msg_type, payload))]
    fn new(msg_type: MessageType, payload: &[u8]) -> Self {
        let rust_type: RustMessageType = msg_type.into();
        Self {
            inner: RustMessage::new(rust_type, payload),
        }
    }

    /// Get the message type
    #[getter]
    fn message_type(&self) -> PyResult<MessageType> {
        self.inner.message_type()
            .map(|mt| mt.into())
            .ok_or_else(|| MXPError::new_err("Invalid message type"))
    }

    /// Get the message ID
    #[getter]
    fn message_id(&self) -> u64 {
        self.inner.message_id()
    }

    /// Get the trace ID
    #[getter]
    fn trace_id(&self) -> u64 {
        self.inner.trace_id()
    }

    /// Get the payload
    #[getter]
    fn payload<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, self.inner.payload().as_ref())
    }

    /// Encode message to bytes
    fn encode<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let encoded = self.inner.encode();
        PyBytes::new_bound(py, &encoded)
    }

    /// Decode message from bytes
    #[staticmethod]
    fn decode(data: &[u8]) -> PyResult<Self> {
        let decoded = RustMessage::decode(data.to_vec())
            .map_err(|e| MXPError::new_err(format!("Decode error: {}", e)))?;
        Ok(Self { inner: decoded })
    }

    fn __repr__(&self) -> String {
        format!(
            "Message(type={:?}, id={}, trace_id={}, payload_len={})",
            self.inner.message_type(),
            self.inner.message_id(),
            self.inner.trace_id(),
            self.inner.payload().len()
        )
    }
}

/// Transport configuration
#[pyclass]
#[derive(Clone)]
pub struct TransportConfig {
    buffer_size: usize,
    pool_size: usize,
}

#[pymethods]
impl TransportConfig {
    #[new]
    #[pyo3(signature = (buffer_size=2048, pool_size=1024))]
    fn new(buffer_size: usize, pool_size: usize) -> Self {
        Self {
            buffer_size,
            pool_size,
        }
    }

    #[getter]
    fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    #[getter]
    fn pool_size(&self) -> usize {
        self.pool_size
    }
}

/// MXP Transport
#[pyclass]
pub struct Transport {
    // This will hold the actual transport implementation
    // For now, this is a placeholder structure
}

#[pymethods]
impl Transport {
    /// Create a new transport with default configuration
    #[new]
    fn new() -> Self {
        Self {}
    }

    /// Create a transport with custom configuration
    #[staticmethod]
    fn with_config(_config: TransportConfig) -> Self {
        // TODO: Use config to initialize transport
        Self {}
    }

    fn __repr__(&self) -> String {
        "Transport()".to_string()
    }
}

/// MXP Agent (placeholder for future async implementation)
#[pyclass]
pub struct Agent {
    // This will hold the agent runtime
}

#[pymethods]
impl Agent {
    /// Connect to an MXP endpoint (placeholder)
    #[staticmethod]
    fn connect(_url: &str) -> PyResult<Self> {
        // TODO: Implement async connection
        Ok(Self {})
    }

    fn __repr__(&self) -> String {
        "Agent()".to_string()
    }
}

/// Python module definition
#[pymodule]
fn mxp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MessageType>()?;
    m.add_class::<Message>()?;
    m.add_class::<Transport>()?;
    m.add_class::<TransportConfig>()?;
    m.add_class::<Agent>()?;
    m.add("MXPError", m.py().get_type_bound::<MXPError>())?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
