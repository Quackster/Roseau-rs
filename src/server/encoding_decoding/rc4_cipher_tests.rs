use super::*;

#[test]
fn enciphers_plaintext_to_uppercase_hex() {
    let mut cipher = Rc4Cipher::new("Key");

    assert_eq!(cipher.encipher_hex(b"Plaintext"), b"BBF316E8D940AF0AD3");
}

#[test]
fn stream_deciphers_hex_in_chunks() {
    let mut encoder = Rc4Cipher::new("1");
    let encrypted = encoder.encipher_hex(b"0014KEYENCRYPTED 1");
    let mut decoder = Rc4HexStreamDecoder::new("1");

    let mut decoded = decoder.push_hex(&encrypted[..7]);
    decoded.extend(decoder.push_hex(&encrypted[7..]));

    assert_eq!(decoded, b"0014KEYENCRYPTED 1");
}

#[test]
fn decodes_v1_secret_payload_to_rc4_key() {
    assert_eq!(secret_decode("ABAB"), "1");
}
