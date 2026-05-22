#[cfg(test)]
mod tests {
    use super::super::encoding::*;

    #[test]
    fn test_detect_utf8_bom() {
        let buf = &[0xEF, 0xBB, 0xBF, b'h', b'i'];
        assert_eq!(detect_encoding(buf), Encoding::Utf8);
    }

    #[test]
    fn test_detect_utf16le_bom() {
        let buf = &[0xFF, 0xFE, 0x41, 0x00];
        assert_eq!(detect_encoding(buf), Encoding::Utf16Le);
    }

    #[test]
    fn test_detect_utf16be_bom() {
        let buf = &[0xFE, 0xFF, 0x00, 0x41];
        assert_eq!(detect_encoding(buf), Encoding::Utf16Be);
    }

    #[test]
    fn test_detect_ascii_is_utf8() {
        let buf = b"hello world";
        assert_eq!(detect_encoding(buf), Encoding::Utf8);
    }

    #[test]
    fn test_detect_latin1_heuristic() {
        let buf = &[0x48, 0xE9, 0x6C, 0x6C, 0xF6]; // Héllö in Latin-1
        assert_eq!(detect_encoding(buf), Encoding::Latin1);
    }

    #[test]
    fn test_decode_utf8_valid() {
        let bytes = b"hello";
        let result = decode_line(bytes, &Encoding::Utf8);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_decode_utf8_invalid_falls_back() {
        let bytes = &[0xFF, 0xFE, 0x41]; // invalid UTF-8
        let result = decode_line(bytes, &Encoding::Utf8);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_decode_latin1() {
        let bytes = &[0x48, 0xE9, 0x6C, 0x6C, 0xF6];
        let result = decode_line(bytes, &Encoding::Latin1);
        assert_eq!(result, "H\u{e9}ll\u{f6}");
    }

    #[test]
    fn test_decode_utf16le() {
        // "AB" in UTF-16 LE
        let bytes = &[0x41, 0x00, 0x42, 0x00];
        let result = decode_line(bytes, &Encoding::Utf16Le);
        assert_eq!(result, "AB");
    }

    #[test]
    fn test_decode_utf16be() {
        // "AB" in UTF-16 BE
        let bytes = &[0x00, 0x41, 0x00, 0x42];
        let result = decode_line(bytes, &Encoding::Utf16Be);
        assert_eq!(result, "AB");
    }

    #[test]
    fn test_strip_bom_utf8() {
        let buf = &[0xEF, 0xBB, 0xBF, b'x'];
        assert_eq!(strip_bom(buf), &[b'x']);
    }

    #[test]
    fn test_strip_bom_utf16le() {
        let buf = &[0xFF, 0xFE, 0x41, 0x00];
        assert_eq!(strip_bom(buf), &[0x41, 0x00]);
    }

    #[test]
    fn test_strip_bom_none() {
        let buf = b"plain";
        assert_eq!(strip_bom(buf), b"plain");
    }
}
