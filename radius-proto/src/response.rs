//! # Access Request Responder
//!
//! This module implements a fully generic responder to RADIUS
//! packets.

use byteorder::{BigEndian, ByteOrder};
use md5::Context;

use super::packet::{Code, Packet};

pub enum Type {
    AuthSuccess,
    AuthFailure,
}

fn u16_to_be(v: u16) -> [u8; 2] {
    let mut b = [0; 2];

    BigEndian::write_u16(&mut b, v);

    b
}

fn response_authenticator(
    secret: &str,
    code: Code,
    identifier: u8,
    length: u16,
    request_authenticator: &[u8; 16],
) -> [u8; 16] {
    // ResponseAuth =
    // MD5(Code+ID+Length+RequestAuth+Attributes+Secret) where +
    // denotes concatenation.

    let mut ctx = Context::new();

    ctx.consume(&[code.into(), identifier]);
    ctx.consume(u16_to_be(length));
    ctx.consume(request_authenticator);

    // We don't have attributes yet.

    ctx.consume(secret.as_bytes());

    ctx.compute().into()
}

/// Create a response packet for an authentication request.
pub fn access_response(secret: &str, request: &Packet, response_type: Type) -> Packet {
    assert_eq!(request.code, Code::AccessRequest);

    let code = match response_type {
        Type::AuthSuccess => Code::AccessAccept,
        Type::AuthFailure => Code::AccessReject,
    };

    let header_len = 20;

    let authenticator = response_authenticator(
        secret,
        code,
        request.identifier,
        header_len,
        &request.authenticator,
    );

    Packet {
        code,
        identifier: request.identifier,
        authenticator: authenticator.clone(),
        attributes: Vec::new(),
    }
}
