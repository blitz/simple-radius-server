# A Simple RADIUS Server

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/blitz/simple-radius-server/Test)
![GitHub](https://img.shields.io/github/license/blitz/simple-radius-server)

This repository contains a simple [RADIUS](https://tools.ietf.org/html/rfc2865) server written in
[Rust](https://www.rust-lang.org/). It only supports authentication
and uses external programs to make a decision on whether to allow or
reject authentication requests.

## Building

Release builds can be generated via [Nix](https://nixos.org/):

```sh
% nix-build
```

Cargo works as well.

## Usage

To start a server that accepts all RADIUS requests use the following command:

```sh
% simple-radius-server -vvv shared-secret /usr/bin/true
DEBUG - 127.0.0.1:36002: received 44 bytes
INFO - 127.0.0.1:36002: trying to authenticate as user 'anna'
INFO - 127.0.0.1:36002: auth-helper '/run/current-system/sw/bin/true' accepted the request (exit code: 0)
DEBUG - 127.0.0.1:36002: sending 20 bytes
...
```

This is obviously not a good idea in practice. To use an external authenticator, you need to write a script:

```sh
% cat auth.sh
#!/bin/sh

set -eu

USER=$1
PASSWORD=$2

if [ "$USER" == "klaus" -a "$PASSWORD" == "verysecure" ]; then
    exit 0
else
    exit 1
fi

% simple-radius-server -vvv shared-secret ./auth.sh
INFO - Listening on 0.0.0.0:1812.
DEBUG - 127.0.0.1:46184: received 45 bytes
INFO - 127.0.0.1:46184: trying to authenticate as user 'klaus'
INFO - 127.0.0.1:46184: auth-helper './foo.sh' accepted the request (exit code: 0)
DEBUG - 127.0.0.1:46184: sending 20 bytes
DEBUG - 127.0.0.1:42769: received 45 bytes
INFO - 127.0.0.1:42769: trying to authenticate as user 'klaus'
INFO - 127.0.0.1:42769: auth-helper './foo.sh' rejected the request (exit code: 1)
DEBUG - 127.0.0.1:42769: sending 20 bytes
...
```

[FreeRADIUS](https://freeradius.org/) comes with the `radclient`
command line tool to send RADIUS messages:

```sh
echo "User-Name=test,User-Password=mypass" | radclient -P udp localhost:1812 auth secret
```
