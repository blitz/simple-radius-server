mod packet;
mod parser;
mod password;
mod response;
mod serialize;

use log::{debug, info};
use packet::{Attribute, Code, Packet};

#[derive(Debug)]
struct AccessRequest {
    username: String,
    authenticator: [u8; 16],
    hidden_password: Vec<u8>,
}

/// Find the first occurrence of a given attribute in a attribute list.
fn find_attribute<F, T>(attrs: &[Attribute], f: F) -> Option<T>
where
    F: FnMut(&Attribute) -> Option<T>,
{
    attrs.iter().filter_map(f).next()
}

// TODO We could return &String here and avoid heap allocation with
// proper lifetime annotations.
fn find_username(attrs: &[Attribute]) -> Option<String> {
    find_attribute(attrs, |a| {
        if let Attribute::UserName(s) = a {
            Some(s.clone())
        } else {
            None
        }
    })
}

// TODO This looks suspiciously similar to find_username. There should
// be a way to unify.
fn find_password(attrs: &[Attribute]) -> Option<Vec<u8>> {
    find_attribute(attrs, |a| {
        if let Attribute::UserPassword(p) = a {
            Some(p.clone())
        } else {
            None
        }
    })
}

fn to_access_request(packet: &Packet) -> Option<AccessRequest> {
    if let Packet {
        code: Code::AccessRequest,
        identifier: _,
        authenticator,
        attributes,
    } = packet
    {
        let username = find_username(&attributes)?;
        let hidden_password = find_password(&attributes)?;

        Some(AccessRequest {
            username,
            authenticator: authenticator.clone(),
            hidden_password,
        })
    } else {
        None
    }
}

/// Receive a raw RADIUS packet and generate a response.
pub fn process<F>(secret: &str, packet: &[u8], authenticate: F) -> Option<Vec<u8>>
where
    F: Fn(&str, &str) -> bool,
{
    let packet = parser::parse(packet).ok()?;
    let access_request = to_access_request(&packet)?;

    let password = password::unhide(
        secret,
        &access_request.authenticator,
        &access_request.hidden_password,
    )?;

    Some(serialize::serialize(&response::access_response(
        secret,
        &packet,
        if authenticate(&access_request.username, &password) {
            response::Type::AuthSuccess
        } else {
            response::Type::AuthFailure
        },
    ))?)
}
