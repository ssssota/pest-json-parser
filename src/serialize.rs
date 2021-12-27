use crate::ast::JSONValue;

pub fn stringify_json(val: &JSONValue) -> String {
    use JSONValue::*;

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, stringify_json(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(stringify_json).collect();
            format!("[{}]", contents.join(","))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", b),
        Null => format!("null"),
    }
}
