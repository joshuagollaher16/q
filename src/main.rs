use pest::Parser;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
struct QParser;

fn main() {

    let program = std::fs::read_to_string("main.q").expect("\"main.q\" does not exist");

    let result = QParser::parse(Rule::file, program.as_str()).unwrap();
    let tokens = result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
