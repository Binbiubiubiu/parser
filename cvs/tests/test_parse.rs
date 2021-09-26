use cvs::*;
use std::fs;

#[test]
fn test_parse_dot_env_file() {
    let env_str = fs::read_to_string("tests/foo.csv").unwrap();
    let parse_result = parse( &env_str);

    assert!(parse_result.is_ok(),"should return an HashMap");

    let env_var = parse_result.unwrap();
    let expected:Vec<Vec<f64>> = vec![
        vec![65279.0,1179403647.0,1463895090.0],
        vec![3.1415927,2.7182817,1.618034],
        vec![-40.0,-273.15],
        vec![13.0,42.0],
        vec![65537.0],
    ];
    assert_eq!(env_var, expected);
}