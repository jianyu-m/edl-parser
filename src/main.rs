
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;
fn main() {
    let successful_parse = CSVParser::parse(Rule::edl, "{trusted{public int hello();}}");
    println!("{:?}", successful_parse);
}
