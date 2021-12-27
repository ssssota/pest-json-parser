extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod deserialize;
mod serialize;

pub use ast::JSONValue;
pub use deserialize::parse_json;
pub use serialize::stringify_json;

#[cfg(test)]
mod deserialize_tests {
    use super::parse_json;
    use super::JSONValue::*;

    #[test]
    fn deserialize_null() {
        assert_eq!(parse_json("null").unwrap(), Null);
        assert_eq!(parse_json(" null ").unwrap(), Null);
        assert!(parse_json(" n u l l ").is_err());
        assert!(parse_json("Null").is_err());
        assert!(parse_json("NULL").is_err());
    }

    #[test]
    fn deserialize_boolean() {
        assert_eq!(parse_json("true").unwrap(), Boolean(true));
        assert_eq!(parse_json("false").unwrap(), Boolean(false));
        assert_eq!(parse_json(" true ").unwrap(), Boolean(true));
        assert_eq!(parse_json(" false ").unwrap(), Boolean(false));
        assert!(parse_json("True").is_err());
        assert!(parse_json("FALSE").is_err());
    }

    #[test]
    fn deserialize_number() {
        assert_eq!(parse_json("1").unwrap(), Number(1.0));
        assert_eq!(parse_json("0.1").unwrap(), Number(0.1));
        assert_eq!(parse_json("0.001").unwrap(), Number(0.001));
        assert_eq!(parse_json("10.5").unwrap(), Number(10.5));
        assert_eq!(parse_json("1.05e-2").unwrap(), Number(0.0105));
        assert_eq!(parse_json("-100").unwrap(), Number(-100.0));
        assert_eq!(parse_json("-0.125").unwrap(), Number(-0.125));
        assert!(parse_json("01").is_err());
        assert!(parse_json(".1").is_err());
        assert!(parse_json("100.").is_err());
    }

    #[test]
    fn deserialize_string() {
        assert_eq!(parse_json("\"test\"").unwrap(), String("test"));
        assert_eq!(parse_json("\"null\"").unwrap(), String("null"));
        assert_eq!(parse_json("\"true\"").unwrap(), String("true"));
        assert_eq!(parse_json("\"1\"").unwrap(), String("1"));
        assert_eq!(parse_json("\"l\nf\"").unwrap(), String("l\nf"));
        assert_eq!(parse_json("\"👍\"").unwrap(), String("👍"));
        assert_eq!(parse_json("\"\\\"\"").unwrap(), String("\\\""));
    }

    #[test]
    fn deserialize_array() {
        assert_eq!(parse_json("[]").unwrap(), Array(vec![]));
        assert_eq!(
            parse_json("[null, true, 1, \"test\"]").unwrap(),
            Array(vec![Null, Boolean(true), Number(1.0), String("test")])
        );
        assert_eq!(
            parse_json("[[[]]]").unwrap(),
            Array(vec![Array(vec![Array(vec![])])])
        );
        assert!(parse_json("[1,]").is_err());
    }

    #[test]
    fn deserialize_object() {
        assert_eq!(parse_json("{}").unwrap(), Object(vec![]));
        assert_eq!(
            parse_json(
                r#"{
                    "key": "value",
                    "num": 100,
                    "bool": false,
                    "null": null,
                    "arr": ["str", 1.5e+10]
                }"#
            )
            .unwrap(),
            Object(vec![
                ("key", String("value")),
                ("num", Number(100.0)),
                ("bool", Boolean(false)),
                ("null", Null),
                ("arr", Array(vec![String("str"), Number(1.5e+10)]))
            ])
        );
        assert!(parse_json("{\"n\":1,}").is_err());
    }
}
