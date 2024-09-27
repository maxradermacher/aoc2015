use std::fs;

fn main() {
    let input = fs::read_to_string("08.txt").unwrap();
    {
        let mut result = 0;
        for line in input.lines() {
            let unquoted_length = unquote(line);
            result += line.len() - unquoted_length;
        }
        println!("{}", result);
    }
    {
        let mut result = 0;
        for line in input.lines() {
            let quoted_length = quote(line);
            result += quoted_length - line.len();
        }
        println!("{}", result);
    }
}

fn quote(value: &str) -> usize {
    2 + value.chars().count() + value.chars().filter(|c| *c == '\\' || *c == '"').count()
}

fn unquote(value: &str) -> usize {
    let mut result = 0;
    let bytes = value.as_bytes();
    let mut index = 1;
    while index < (bytes.len() - 1) {
        match bytes[index] {
            b'\\' => {
                match bytes[index + 1] {
                    b'"' | b'\\' => {
                        result += 1;
                        index += 2;
                    },
                    b'x' => {
                        result += 1;
                        index += 4;
                    }
                    _ => { panic!("invalid escape sequence"); }
                }
            },
            _ => {
                result += 1;
                index += 1;
            },
        }
        assert!(index < bytes.len());
    }
    result
}
