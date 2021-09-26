use pest_derive::Parser;

pub use pest::Parser;

#[derive(Parser)]
#[grammar = "cvs.pest"]
struct PestParser;

type StdError = Box<dyn std::error::Error>;



pub fn parse<'a>(input: &'a str) -> Result<Vec<Vec<f64>>,StdError> {
    let file = PestParser::parse(Rule::file, input)
        .expect("success parse")
        .next()
        .expect("success iterator file");
    let mut env_var = Vec::new();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                let mut nums = vec![];

                for field in record.into_inner() {
                   let  n = field.as_str().parse::<f64>().unwrap();
                   nums.push(n);
                }
                env_var.push(nums);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(env_var)
}

