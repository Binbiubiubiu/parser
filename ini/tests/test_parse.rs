use ini::*;
use std::fs;

#[test]
fn test_parse_dot_env_file() {
    let env_str = fs::read_to_string("tests/foo.ini").unwrap();
    let parse_result = parse(&env_str);

    assert!(parse_result.is_ok(), "should return ok");

    let env_var = parse_result.unwrap().to_string();
    fs::write("tests/foo.json", env_var).unwrap();
}
