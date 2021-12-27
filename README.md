# JSON Parser using pest

https://pest.rs/book/examples/json.html

## Usage

_Cargo.toml_

```toml
[dependencies]
pest_json_parser = { git = "https://github.com/ssssota/pest-json-parser.git" }
```

_example_

```rust
use pest_json_parser::parse_json;
use pest_json_parser::stringify_json;
use pest_json_parser::JSONValue;

fn main() {
  let json: JSONValue = parse_json("{ \"key\": \"value\" }").expect("unsuccessful parse");
  println!("{}", stringify_json(&json));
}
```
