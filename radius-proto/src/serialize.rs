//! # RADIUS Packet Serialization
//!
//! This module contains the serialization code for RADIUS packets.

use byteorder::{BigEndian, WriteBytesExt};

use super::packet::*;

fn serialize_attribute(_attribute: &Attribute) -> Vec<u8> {
    unimplemented!("Serializing attributes is not implemented yet");
}

/// Serialize a RADIUS packet to a byte vector.
///
/// This operation can fail if the packet is malformed or too long.
pub fn serialize(packet: &Packet) -> Option<Vec<u8>> {
    let attribute_blob: Vec<u8> = packet
        .attributes
        .iter()
        .map(serialize_attribute)
        .flatten()
        .collect();

    let header_size: usize = 20;
    let total_size = header_size + attribute_blob.len();

    if total_size > u16::MAX as usize {
        // Attributes are too large.
        return None;
    }

    let mut encoded = Vec::new();

    encoded.write_u8(packet.code.into()).ok()?;
    encoded.write_u8(packet.identifier).ok()?;
    encoded.write_u16::<BigEndian>(total_size as u16).ok()?;
    encoded.extend_from_slice(&packet.authenticator);

    encoded.extend_from_slice(&attribute_blob);

    assert_eq!(encoded.len(), total_size);
    Some(encoded)
}
