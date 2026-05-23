use super::*;
use regex::Regex;

#[test]
fn test_replace_simple() {
    let rules = vec![TransformRule::Replace {
        from: "ERROR".to_string(),
        to: "ERR".to_string(),
    }];
    let t = LineTransformer::new(rules);
    assert_eq!(t.transform("ERROR: disk full"), "ERR: disk full");
}

#[test]
fn test_replace_no_match() {
    let rules = vec![TransformRule::Replace {
        from: "WARN".to_string(),
        to: "W".to_string(),
    }];
    let t = LineTransformer::new(rules);
    assert_eq!(t.transform("INFO: all good"), "INFO: all good");
}

#[test]
fn test_regex_replace() {
    let rules = vec![TransformRule::RegexReplace {
        pattern: Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
        replacement: "<DATE>".to_string(),
    }];
    let t = LineTransformer::new(rules);
    assert_eq!(
        t.transform("2024-01-15 ERROR something"),
        "<DATE> ERROR something"
    );
}

#[test]
fn test_uppercase() {
    let t = LineTransformer::new(vec![TransformRule::Uppercase]);
    assert_eq!(t.transform("hello world"), "HELLO WORLD");
}

#[test]
fn test_lowercase() {
    let t = LineTransformer::new(vec![TransformRule::Lowercase]);
    assert_eq!(t.transform("INFO: DISK FULL"), "info: disk full");
}

#[test]
fn test_trim() {
    let t = LineTransformer::new(vec![TransformRule::Trim]);
    assert_eq!(t.transform("  padded line  "), "padded line");
}

#[test]
fn test_chained_rules() {
    let rules = vec![
        TransformRule::Trim,
        TransformRule::Replace {
            from: "ERROR".to_string(),
            to: "ERR".to_string(),
        },
        TransformRule::Lowercase,
    ];
    let t = LineTransformer::new(rules);
    assert_eq!(t.transform("  ERROR: disk full  "), "err: disk full");
}

#[test]
fn test_empty_transformer() {
    let t = LineTransformer::default();
    assert!(t.is_empty());
    assert_eq!(t.transform("unchanged"), "unchanged");
}

#[test]
fn test_regex_replace_multiple_matches() {
    let rules = vec![TransformRule::RegexReplace {
        pattern: Regex::new(r"\b\d+\b").unwrap(),
        replacement: "NUM".to_string(),
    }];
    let t = LineTransformer::new(rules);
    assert_eq!(t.transform("retry 3 of 5 failed"), "retry NUM of NUM failed");
}
