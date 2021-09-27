use json::*;
use std::fs;

#[test]
fn test_parse_dot_env_file() {
    let unparsed_file = fs::read_to_string("tests/data.json").expect("cannot read file");

    let json: JSONValue = parse(&unparsed_file).expect("unsuccessful parse");
    assert_eq!(serialize_jsonvalue(&json), unparsed_file);
}
