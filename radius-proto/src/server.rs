//! This is a simple test program for the RADIUS protocol parsing.
//!
//! Send data to it using:
//! echo "User-Name=test,User-Password=mypass" | radclient -P udp localhost:1812 auth secret

use clap::{crate_version, App, Arg, ArgMatches};
use log::{debug, error, info, warn};
use std::net::{SocketAddr, UdpSocket};
use std::process::{Command, Stdio};

use radius::process;

struct Config {
    listen_addr: SocketAddr,
    secret: String,
    auth_helper: String,
}

fn server_loop(config: Config) -> std::io::Result<()> {
    info!("Listening on {}.", config.listen_addr);

    let socket = UdpSocket::bind(config.listen_addr)?;

    loop {
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 4096];
        let (packet_len, src_addr) = socket.recv_from(&mut buf)?;

        debug!("{}: received {} bytes", src_addr, packet_len);

        let response = process(&config.secret, &buf[..packet_len], |user, pass| {
            info!("{}: trying to authenticate as user '{}'", src_addr, user);

            let child = Command::new(&config.auth_helper)
                .arg(user)
                .arg(pass)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .output()
                .expect("Failed to execute authentication helper");
            let status = child.status;

            if child.stderr.len() > 0 {
                warn!(
                    "{}: auth-helper '{}' complained: {}",
                    src_addr,
                    config.auth_helper,
                    String::from_utf8_lossy(&child.stderr).trim_end()
                );
            }
            info!(
                "{}: auth-helper '{}' {} the request ({})",
                src_addr,
                config.auth_helper,
                if status.success() {
                    "accepted"
                } else {
                    "rejected"
                },
                status
            );

            status.success()
        });

        match response {
            Err(e) => error!("{}: Ignoring packet: {}", src_addr, e),
            Ok(Some(data)) => {
                debug!("{}: sending {} bytes", src_addr, data.len());
                socket.send_to(&data, src_addr)?;
            }
            Ok(None) => {}
        }
    }
}

fn io_error(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, msg)
}

fn parse_args(m: ArgMatches) -> std::io::Result<Config> {
    let listen_address = m
        .value_of("listen")
        .unwrap()
        .parse()
        .map_err(|_| io_error("Invalid IP address"))?;

    let listen_port = m
        .value_of("port")
        .unwrap()
        .parse()
        .map_err(|_| io_error("Invalid port number"))?;

    Ok(Config {
        listen_addr: SocketAddr::new(listen_address, listen_port),
        secret: m.value_of("secret").unwrap().to_string(),
        auth_helper: m.value_of("auth-helper").unwrap().to_string(),
    })
}

fn main() -> std::io::Result<()> {
    let m = App::new("Simple RADIUS Server")
        .version(crate_version!())
        .long_about(
	    concat!("\nThis program is a minimal implementation of a RADIUS server that is able to respond to access requests ",
		    "from a network access server (e.g. a Wifi router). It does _not_ implement the full RADIUS spec, but instead ",
		    "tries to be as simple as possible while still being useful.")
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Enable verbose output. Can be specified multiple times."),
        )
        .arg(
            Arg::with_name("port")
                .short("p").long("port")
                .value_name("port")
		.default_value("1812")
                .help("Select the UDP port that the server will listen on."),
        )
        .arg(
            Arg::with_name("listen")
                .short("l").long("listen")
                .value_name("address")
		.default_value("0.0.0.0")
                .help("The IP address the server listens on"),
        )
	.arg(
            Arg::with_name("secret")
                .required(true)
                .help("The shared secret between the RADIUS client and server."),
        )
        .arg(
	    Arg::with_name("auth-helper")
		.required(true)
		.help(concat!("The path to the executable that performs the actual authentication. It is executed ",
			      "for each authentication request. It receives the username and password as parameters. ",
			      "Depending on the exit code the authentication request will be accepted or rejected."))
	)
        .get_matches();

    let verbose = m.occurrences_of("verbosity") as usize;

    stderrlog::new()
        .module(module_path!())
        .verbosity(verbose)
        .init()
        .unwrap();

    match parse_args(m).and_then(|config| server_loop(config)) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
