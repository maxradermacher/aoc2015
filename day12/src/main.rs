use std::fs;
use serde_json::Value;

fn main() {
    let input = fs::read_to_string("12.txt").unwrap();
    let value: Value = serde_json::from_str(&input).unwrap();
    {
        let mut total = 0;
        sum_value(&value, None, &mut total);
        println!("{}", total);
    }
    {
        let mut total = 0;
        sum_value(&value, Some("red"), &mut total);
        println!("{}", total);
    }
}

fn sum_value(value: &Value, ignore_value: Option<&str>, total: &mut i64) {
    match value {
        Value::Number(number) => {
            *total += number.as_i64().unwrap();
        }
        Value::Array(values) => {
            for value in values {
                sum_value(value, ignore_value, total);
            }
        }
        Value::Object(values) => {
            if let Some(ignore_value) = ignore_value {
                if values.values().any(|v| if let Value::String(s) = v { s == ignore_value } else { false }) {
                    return;
                }
            }
            for (_, value) in values {
                sum_value(value, ignore_value, total);
            }
        }
        _ => {},
    }
}
