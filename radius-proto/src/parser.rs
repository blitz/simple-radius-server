//! # RADIUS Protocol Parser
//!
//! This module parses byte slices into easy-to-use protocol data types.

use nom::bytes::complete::take;
use nom::error::{make_error, ErrorKind};
use nom::multi::many0;
use nom::number::complete::{be_u16, be_u8};
use nom::sequence::tuple;
use nom::{Err, IResult};
use std::convert::TryInto;

use super::error::Error;
use super::packet::*;

fn parse_authenticator(i: &[u8]) -> IResult<&[u8], [u8; 16]> {
    let (i, authenticator_slice) = take(AUTHENTICATOR_LEN)(i)?;

    Ok((i, authenticator_slice.try_into().unwrap()))
}

fn parse_string(i: &[u8]) -> Result<String, Err<(&[u8], ErrorKind)>> {
    // TODO This should indicate some kind of parse failure if the
    // input is not real UTF-8.
    let s = String::from_utf8(i.to_vec()).map_err(|_| Err::Error(make_error(i, ErrorKind::Eof)))?;

    Ok(s)
}

fn parse_attribute_no_header(
    attr_type: u8,
    i: &[u8],
) -> Result<Attribute, Err<(&[u8], ErrorKind)>> {
    match attr_type {
        1 => Ok(Attribute::UserName(parse_string(i)?)),
        2 => Ok(Attribute::UserPassword(i.to_vec())),
        _ => Ok(Attribute::Unknown(attr_type, Vec::from(i))),
    }
}

fn parse_attribute(i: &[u8]) -> IResult<&[u8], Attribute> {
    let (i, (attr_type, length)) = tuple((be_u8, be_u8))(i)?;

    if length < 2 {
        // TODO This should indicate some kind of parse failure.
        return Err(Err::Error(make_error(i, ErrorKind::Eof)));
    }

    let (i, bytes) = take(length - 2)(i)?;

    Ok((i, parse_attribute_no_header(attr_type, bytes)?))
}

fn parse_attributes(i: &[u8]) -> IResult<&[u8], Vec<Attribute>> {
    many0(parse_attribute)(i)
}

fn parse_code(i: &[u8]) -> IResult<&[u8], Code> {
    be_u8(i).map(|(i, v)| {
        (
            i,
            match v {
                1 => Code::AccessRequest,
                2 => Code::AccessAccept,
                3 => Code::AccessReject,
                _ => Code::Unknown(v),
            },
        )
    })
}

fn parse_internal(i: &[u8]) -> IResult<&[u8], Packet> {
    // tuple takes as argument a tuple of parsers and will return
    // a tuple of their results
    let (i, (code, identifier, length, authenticator)) =
        tuple((parse_code, be_u8, be_u16, parse_authenticator))(i)?;
    let header_len = 20;

    // The length value should be at least as large as the header itself.
    if length < header_len {
        // TODO This should indicate some kind of parse failure.
        return Err(Err::Error(make_error(i, ErrorKind::Eof)));
    }

    let (i, attribute_bytes) = take(length - header_len)(i)?;
    let (_, attributes) = parse_attributes(attribute_bytes)?;

    Ok((
        i,
        Packet {
            code,
            identifier,
            authenticator,
            attributes,
        },
    ))
}

/// Parse a RADIUS packet.
pub fn parse(i: &[u8]) -> Result<Packet, Error> {
    let (_, packet) = parse_internal(i)
        .map_err(|e| Error::new(&format!("Failed to parse RADIUS packet: {}", e)))?;

    Ok(packet)
}
