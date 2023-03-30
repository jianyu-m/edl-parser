
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;
fn main() {
    let edls = fs::read_to_string("test.edl")
        .expect("Should have been able to read the file");
    let successful_parse = CSVParser::parse(Rule::edl, &edls);
    println!("{:?}", successful_parse);
}
