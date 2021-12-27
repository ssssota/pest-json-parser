use pest_json_parser::parse_json;
use pest_json_parser::stringify_json;
use pest_json_parser::JSONValue;

use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("data.json").expect("cannot read file");

    let json: JSONValue = parse_json(&unparsed_file).expect("unsuccessful parse");

    println!("{}", stringify_json(&json));
}
