use std::fs;

fn main() {
    let input = fs::read_to_string("10.txt").unwrap().trim().to_string();
    {
        let mut output = input.clone();
        for _ in 0..40 {
            output = compute(output);
        }
        println!("{}", output.len());
    }
    {
        let mut output = input.clone();
        for _ in 0..50 {
            output = compute(output);
        }
        println!("{}", output.len());
    }
}

fn compute(input: String) -> String {
    let mut result = String::new();
    let mut input = input.chars().peekable();
    while let Some(target) = input.next() {
        let mut count = 1;
        while input.peek() == Some(&target) {
            input.next();
            count += 1;
        }
        assert!(count < 10);
        result.push((b'0' + count) as char);
        result.push(target);
    }
    result
}
