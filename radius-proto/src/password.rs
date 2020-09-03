//! # RADIUS Protocol Parser
//!
//! This module implements the MD5-based (ouch!) password hiding algorithm.

use md5::Context;

/// Remove any trailing zero bytes.
fn truncate_zero_padding(v: &mut Vec<u8>) {
    let nonzero = v.iter().rposition(|x| *x != 0);

    v.truncate(match nonzero {
        Some(nonzero_idx) => nonzero_idx + 1,
        None => 0,
    });
}

/// Decode a password using according to the RADIUS specification.
///
/// See Section 5.2 User-Password for the detailed algorithm.
pub fn unhide(secret: &str, authenticator: &[u8; 16], hidden: &[u8]) -> Option<String> {
    let mut md5_ctx = Context::new();
    let md5_ctx_old;

    md5_ctx.consume(secret.as_bytes());
    md5_ctx_old = md5_ctx.clone();
    md5_ctx.consume(authenticator);

    let mut revealed = Vec::new();

    for hidden_block in hidden.chunks(16) {
        let digest = md5_ctx.compute();

        md5_ctx = md5_ctx_old.clone();
        md5_ctx.consume(hidden_block);

        revealed.extend(hidden_block.iter().zip(digest.iter()).map(|(a, b)| a ^ b));
    }

    truncate_zero_padding(&mut revealed);
    String::from_utf8(revealed).ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn unhide_short_works() {
        let authenticator = [
            93, 47, 235, 29, 179, 103, 83, 63, 45, 40, 174, 159, 52, 103, 66, 219,
        ];
        let hidden = [
            203, 148, 156, 169, 212, 124, 106, 127, 87, 211, 97, 131, 76, 102, 155, 136,
        ];
        let secret = "secret";

        assert_eq!(
            super::unhide(secret, &authenticator, &hidden),
            Some(String::from("mypass"))
        );
    }

    #[test]
    fn unhide_long_works() {
        let authenticator = [
            220, 89, 137, 91, 5, 47, 128, 214, 230, 213, 60, 160, 213, 148, 14, 73,
        ];
        let hidden = [
            103, 251, 92, 58, 78, 1, 105, 179, 78, 60, 214, 180, 110, 169, 213, 196, 228, 60, 209,
            189, 76, 5, 214, 49, 255, 124, 47, 54, 163, 171, 216, 209,
        ];
        let secret = "secret";

        assert_eq!(
            super::unhide(secret, &authenticator, &hidden),
            Some(String::from("thisisaverylongpassword"))
        );
    }
}
