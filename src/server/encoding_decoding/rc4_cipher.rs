#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rc4Cipher {
    sbox: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4Cipher {
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        let key = key.as_ref();
        assert!(!key.is_empty(), "RC4 key must not be empty");

        let mut sbox = [0; 256];
        for (index, value) in sbox.iter_mut().enumerate() {
            *value = index as u8;
        }

        let mut j = 0u8;
        for i in 0..256 {
            j = j.wrapping_add(sbox[i]).wrapping_add(key[i % key.len()]);
            sbox.swap(i, usize::from(j));
        }

        Self { sbox, i: 0, j: 0 }
    }

    pub fn apply(&mut self, byte: u8) -> u8 {
        byte ^ self.next_byte()
    }

    pub fn apply_all(&mut self, bytes: &[u8]) -> Vec<u8> {
        bytes.iter().map(|byte| self.apply(*byte)).collect()
    }

    pub fn encipher_hex(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(bytes.len() * 2);

        for byte in bytes {
            let encrypted = self.apply(*byte);
            output.push(hex_digit(encrypted >> 4));
            output.push(hex_digit(encrypted & 0x0f));
        }

        output
    }

    fn next_byte(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.sbox[usize::from(self.i)]);

        self.sbox.swap(usize::from(self.i), usize::from(self.j));

        let index = self.sbox[usize::from(self.i)].wrapping_add(self.sbox[usize::from(self.j)]);
        self.sbox[usize::from(index)]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rc4HexStreamDecoder {
    cipher: Rc4Cipher,
    pending_nibble: Option<u8>,
}

impl Rc4HexStreamDecoder {
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        Self {
            cipher: Rc4Cipher::new(key),
            pending_nibble: None,
        }
    }

    pub fn push_hex(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(bytes.len() / 2);

        for byte in bytes {
            let Some(nibble) = parse_hex_digit(*byte) else {
                continue;
            };

            if let Some(high) = self.pending_nibble.take() {
                output.push(self.cipher.apply((high << 4) | nibble));
            } else {
                self.pending_nibble = Some(nibble);
            }
        }

        output
    }
}

pub fn secret_decode(key: &str) -> String {
    let mut l = key.len();
    if l % 2 == 1 {
        l -= 1;
    }

    let table = &key[..key.len() / 2];
    let encoded = &key[key.len() / 2..l];
    let mut checksum = 0usize;

    for (index, character) in encoded.chars().enumerate() {
        let mut value = table
            .find(character)
            .map(|offset| offset as isize)
            .unwrap_or(-1);

        if value % 2 == 0 {
            value *= 2;
        }
        if index % 3 == 0 {
            value *= 3;
        }
        if value < 0 {
            value = (encoded.len() % 2) as isize;
        }

        checksum += value as usize;
    }

    checksum.to_string()
}

fn parse_hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn hex_digit(value: u8) -> u8 {
    match value {
        0..=9 => b'0' + value,
        10..=15 => b'A' + value - 10,
        _ => unreachable!("hex digit out of range"),
    }
}

#[cfg(test)]
#[path = "rc4_cipher_tests.rs"]
mod tests;
