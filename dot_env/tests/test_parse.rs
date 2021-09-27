use dot_env::*;
use std::fs;

#[test]
fn test_parse_dot_env_file() {
    let env_str = fs::read_to_string("tests/.env").unwrap();
    let parse_result = parse(&env_str);

    assert!(parse_result.is_ok(), "should return an HashMap");

    let env_var = parse_result.unwrap();
    assert_eq!(env_var["BASIC"], "basic", "sets basic environment variable");

    assert_eq!(
        env_var["AFTER_LINE"], "after_line",
        "reads after a skipped line"
    );
    assert_eq!(
        env_var["EMPTY"], "",
        "defaults empty values to empty string"
    );
    assert_eq!(
        env_var["SINGLE_QUOTES"], "single_quotes",
        "escapes single quoted values"
    );
    assert_eq!(
        env_var["SINGLE_QUOTES_SPACED"], "    single quotes    ",
        "respects surrounding spaces in single quotes"
    );
    assert_eq!(
        env_var["DOUBLE_QUOTES"], "double_quotes",
        "escapes double quoted values"
    );
    assert_eq!(
        env_var["DOUBLE_QUOTES_SPACED"], "    double quotes    ",
        "respects surrounding spaces in double quotes"
    );
    assert_eq!(
        env_var["EXPAND_NEWLINES"], "expand\nnew\nlines",
        "expands newlines but only if double quoted"
    );
    assert_eq!(
        env_var["DONT_EXPAND_UNQUOTED"], "dontexpand\\nnewlines",
        "expands newlines but only if double quoted"
    );
    assert_eq!(
        env_var["DONT_EXPAND_SQUOTED"], "dontexpand\\nnewlines",
        "expands newlines but only if double quoted"
    );
    assert_eq!(env_var.get("COMMENTS"), None, "ignores commented lines");
    assert_eq!(
        env_var["EQUAL_SIGNS"], "equals==",
        "respects equals signs in values"
    );
    assert_eq!(
        env_var["RETAIN_INNER_QUOTES"], "{\"foo\": \"bar\"}",
        "retains inner quotes"
    );
    assert_eq!(
        env_var["RETAIN_LEADING_DQUOTE"], "\"retained",
        "retains leading double quote"
    );
    assert_eq!(
        env_var["RETAIN_LEADING_SQUOTE"], "'retained",
        "retains leading single quote"
    );
    assert_eq!(
        env_var["RETAIN_TRAILING_DQUOTE"], "retained\"",
        "reatins trailing double quote"
    );
    assert_eq!(
        env_var["RETAIN_TRAILING_SQUOTE"], "retained'",
        "retains trailing single quote"
    );
    assert_eq!(
        env_var["RETAIN_INNER_QUOTES_AS_STRING"], "{\"foo\": \"bar\"}",
        "retains inner quotes"
    );
    assert_eq!(
        env_var["TRIM_SPACE_FROM_UNQUOTED"], "some spaced out string",
        "retains spaces in string"
    );
    assert_eq!(
        env_var["USERNAME"], "therealnerdybeast@example.tld",
        "parses email addresses completely"
    );
    assert_eq!(
        env_var["SPACED_KEY"], "parsed",
        "parses keys and values surrounded by spaces"
    );
}
