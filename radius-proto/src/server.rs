//! This is a simple test program for the RADIUS protocol parsing.
//!
//! Send data to it using:
//! echo "User-Name=test,User-Password=mypass" | radclient -P udp localhost:1812 auth secret

use std::net::UdpSocket;

use radius::process;

const RADIUS_SECRET: &str = "secret";

const RADIUS_EXPECTED_USER: &str = "test";
const RADIUS_EXPECTED_PASSWORD: &str = "mypass";

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:1812")?;

    loop {
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 4096];
        let (packet_len, src_addr) = socket.recv_from(&mut buf)?;

        let response: Option<Vec<u8>> = process(RADIUS_SECRET, &buf[..packet_len], |user, pass| {
            user == RADIUS_EXPECTED_USER && pass == RADIUS_EXPECTED_PASSWORD
        });

        if let Some(data) = response {
            socket.send_to(&data, src_addr)?;
        }
    }
}
