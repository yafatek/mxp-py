"""Test MXP Python bindings."""
import mxp


def test_message_creation():
    """Test creating a message."""
    msg = mxp.Message(mxp.MessageType.Call, b"test payload")
    assert msg.message_type == mxp.MessageType.Call
    assert bytes(msg.payload) == b"test payload"


def test_message_encode_decode():
    """Test message encode/decode roundtrip."""
    original = mxp.Message(mxp.MessageType.Call, b"test data")
    
    # Encode
    encoded = original.encode()
    assert isinstance(encoded, bytes)
    
    # Decode
    decoded = mxp.Message.decode(encoded)
    assert decoded.message_type == original.message_type
    assert bytes(decoded.payload) == bytes(original.payload)


def test_message_ids():
    """Test that messages have unique IDs."""
    msg1 = mxp.Message(mxp.MessageType.Call, b"first")
    msg2 = mxp.Message(mxp.MessageType.Call, b"second")
    
    # Message IDs should be different
    assert msg1.message_id != msg2.message_id
    # Trace IDs should be different
    assert msg1.trace_id != msg2.trace_id


def test_transport_config():
    """Test transport configuration."""
    config = mxp.TransportConfig(buffer_size=4096, pool_size=512)
    assert config.buffer_size == 4096
    assert config.pool_size == 512


def test_transport_creation():
    """Test creating a transport."""
    transport = mxp.Transport()
    assert transport is not None
    
    # With config
    config = mxp.TransportConfig(buffer_size=8192, pool_size=256)
    transport2 = mxp.Transport.with_config(config)
    assert transport2 is not None
