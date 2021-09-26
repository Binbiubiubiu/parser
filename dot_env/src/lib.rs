mod string;

use pest_derive::Parser;
use std::collections::HashMap;
use string::RemoveQuoted;

pub use pest::Parser;

#[derive(Parser)]
#[grammar = "dot_env.pest"]
struct PestParser;

type StdError = Box<dyn std::error::Error>;



pub fn parse<'a>(input: &'a str) -> Result<HashMap<&'a str, String>,StdError> {
    let file = PestParser::parse(Rule::file, input)
        .expect("success parse")
        .next()
        .expect("success iterator file");
    let mut env_var = HashMap::new();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::pair => {
                let mut current_key = "";
                for field in record.into_inner() {
                    match field.as_rule() {
                        Rule::key => {
                            current_key = field.as_str();
                        }
                        Rule::value => {
                            env_var.insert(current_key.clone(), field.as_str().remove_quoted());
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

