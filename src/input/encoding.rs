use std::borrow::Cow;

/// Detected or specified encoding for input data.
#[derive(Debug, Clone, PartialEq)]
pub enum Encoding {
    Utf8,
    Latin1,
    Utf16Le,
    Utf16Be,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Utf8
    }
}

/// Detects encoding from a byte order mark (BOM) or heuristic scan.
pub fn detect_encoding(buf: &[u8]) -> Encoding {
    if buf.starts_with(&[0xFF, 0xFE]) {
        return Encoding::Utf16Le;
    }
    if buf.starts_with(&[0xFE, 0xFF]) {
        return Encoding::Utf16Be;
    }
    if buf.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Encoding::Utf8;
    }
    // Heuristic: if high bytes are rare, treat as UTF-8
    let high_byte_count = buf.iter().filter(|&&b| b > 0x7F).count();
    if high_byte_count == 0 || buf.is_empty() {
        return Encoding::Utf8;
    }
    // Naïve Latin-1 fallback
    Encoding::Latin1
}

/// Decodes a raw byte slice into a UTF-8 string using the given encoding.
/// Returns a `Cow` to avoid allocation when the input is already valid UTF-8.
pub fn decode_line<'a>(bytes: &'a [u8], encoding: &Encoding) -> Cow<'a, str> {
    match encoding {
        Encoding::Utf8 => {
            match std::str::from_utf8(bytes) {
                Ok(s) => Cow::Borrowed(s),
                Err(_) => Cow::Owned(String::from_utf8_lossy(bytes).into_owned()),
            }
        }
        Encoding::Latin1 => {
            // Latin-1: each byte maps directly to its Unicode code point.
            let s: String = bytes.iter().map(|&b| b as char).collect();
            Cow::Owned(s)
        }
        Encoding::Utf16Le => {
            let pairs: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            Cow::Owned(String::from_utf16_lossy(&pairs))
        }
        Encoding::Utf16Be => {
            let pairs: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            Cow::Owned(String::from_utf16_lossy(&pairs))
        }
    }
}

/// Strips a leading BOM from a byte slice if present.
pub fn strip_bom(buf: &[u8]) -> &[u8] {
    if buf.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &buf[3..]
    } else if buf.starts_with(&[0xFF, 0xFE]) || buf.starts_with(&[0xFE, 0xFF]) {
        &buf[2..]
    } else {
        buf
    }
}
