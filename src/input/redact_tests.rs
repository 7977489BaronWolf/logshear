//! Tests for the redaction module.

#[cfg(test)]
mod tests {
    use crate::input::redact::Redactor;

    fn make_redactor() -> Redactor {
        let mut r = Redactor::new("[REDACTED]");
        r.add_field_names(["password", "token", "secret"]);
        r.add_pattern(r"\b\d{4}-\d{4}-\d{4}-\d{4}\b").unwrap(); // credit card-like
        r.add_pattern(r"\b[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}\b").unwrap(); // email
        r
    }

    #[test]
    fn redact_sensitive_field_by_name() {
        let r = make_redactor();
        assert_eq!(r.redact_field("password", "hunter2"), "[REDACTED]");
        assert_eq!(r.redact_field("token", "abc123"), "[REDACTED]");
    }

    #[test]
    fn non_sensitive_field_passes_through() {
        let r = make_redactor();
        assert_eq!(r.redact_field("level", "info"), "info");
    }

    #[test]
    fn redact_credit_card_pattern_in_value() {
        let r = make_redactor();
        let input = "charged 1234-5678-9012-3456 today";
        let output = r.redact_value(input);
        assert!(!output.contains("1234-5678-9012-3456"));
        assert!(output.contains("[REDACTED]"));
    }

    #[test]
    fn redact_email_pattern_in_value() {
        let r = make_redactor();
        let input = "user alice@example.com logged in";
        let output = r.redact_value(input);
        assert!(!output.contains("alice@example.com"));
        assert!(output.contains("[REDACTED]"));
    }

    #[test]
    fn no_match_returns_original() {
        let r = make_redactor();
        let input = "no sensitive data here";
        assert_eq!(r.redact_value(input), input);
    }

    #[test]
    fn should_redact_field_check() {
        let r = make_redactor();
        assert!(r.should_redact_field("secret"));
        assert!(!r.should_redact_field("message"));
    }

    #[test]
    fn custom_placeholder() {
        let mut r = Redactor::new("***");
        r.add_field_names(["api_key"]);
        assert_eq!(r.redact_field("api_key", "xyz"), "***");
        assert_eq!(r.placeholder(), "***");
    }

    #[test]
    fn invalid_pattern_returns_error() {
        let mut r = Redactor::new("[REDACTED]");
        assert!(r.add_pattern(r"[").is_err());
    }
}
