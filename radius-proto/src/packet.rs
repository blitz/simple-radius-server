//! # RADIUS Protocol Definitions
//!
//! This module contains data types to describe the RADIUS protocol
//! wire format.

/// An attribute in a RADIUS packet.
#[derive(Debug)]
pub enum Attribute {
    UserName(String),
    UserPassword(Vec<u8>),
    Unknown(u8, Vec<u8>),
}

/// The type field in a RADIUS packet.
#[derive(Debug, Copy, Clone)]
pub enum Code {
    AccessRequest,
    AccessAccept,
    AccessReject,
    Unknown(u8),
}

impl From<Code> for u8 {
    fn from(code: Code) -> Self {
        match code {
            Code::AccessRequest => 1,
            Code::AccessAccept => 2,
            Code::AccessReject => 3,
            Code::Unknown(c) => c,
        }
    }
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        u8::from(*self) == u8::from(*other)
    }
}

impl Eq for Code {}

/// The length of the authenticator field in the RADIUS packet header.
pub const AUTHENTICATOR_LEN: usize = 16;

#[derive(Debug)]
pub struct Packet {
    pub code: Code,
    pub identifier: u8,
    pub authenticator: [u8; AUTHENTICATOR_LEN],
    pub attributes: Vec<Attribute>,
}
