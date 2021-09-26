mod json;
mod string;

use pest_derive::Parser;
use std::collections::HashMap;
use string::{RemoveQuoted, SplitDot};

pub use pest::Parser;

pub use json::JsonValue;

#[derive(Parser)]
#[grammar = "ini.pest"]
struct PestParser;

type StdError = Box<dyn std::error::Error>;

pub fn parse<'a>(input: &'a str) -> Result<JsonValue, StdError> {
    let file = PestParser::parse(Rule::file, input)
        .expect("success parse")
        .next()
        .expect("success iterator file");
    let mut env_var = JsonValue::Object(HashMap::new());
    let mut scope: Vec<String> = Vec::new();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::title => {
                scope = record.as_str().split_dot();
            }
            Rule::pair => {
                let mut current_key = String::new();
                for field in record.into_inner() {
                    match field.as_rule() {
                        Rule::key => {
                            current_key = field.as_str().remove_quoted();
                        }
                        Rule::value => {
                            let path = [scope.clone(), vec![current_key.clone()]].concat();
                            env_var.set_paths(
                                path.as_slice(),
                                &JsonValue::String(field.as_str().remove_quoted()),
                            );
                        }
                        _ => unreachable!()
                    }
                }
            }
            _ => unreachable!()
        }
    }

    Ok(env_var)
}
