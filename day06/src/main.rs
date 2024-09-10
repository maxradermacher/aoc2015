use nom;
use std::fs;

#[derive(Debug)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

fn parse_action(input: &str) -> nom::IResult<&str, Action> {
    nom::combinator::map(
        nom::branch::alt((
            nom::bytes::complete::tag("toggle"),
            nom::bytes::complete::tag("turn on"),
            nom::bytes::complete::tag("turn off"),
        )),
        |v: &str| match v {
            "toggle" => Action::Toggle,
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            _ => panic!("must match"),
        },
    )(input)
}
fn parse_point(input: &str) -> nom::IResult<&str, (i32, i32)> {
    nom::sequence::separated_pair(
        nom::character::complete::i32,
        nom::bytes::complete::tag(","),
        nom::character::complete::i32,
    )(input)
}

fn parse(input: &str) -> nom::IResult<&str, (Action, ((i32, i32), (i32, i32)))> {
    nom::sequence::separated_pair(
        parse_action,
        nom::character::complete::space1,
        nom::sequence::separated_pair(
            parse_point,
            nom::bytes::complete::tag(" through "),
            parse_point,
        ),
    )(input)
}

fn main() {
    let input = fs::read_to_string("06.txt").unwrap();
    {
        let mut state = vec![false; 1_000_000];
        for line in input.lines() {
            let (_, (action, (p1, p2))) = parse(line).unwrap();
            for x in p1.0..=p2.0 {
                for y in p1.1..=p2.1 {
                    let pos = usize::try_from(y * 1000 + x).unwrap();
                    match action {
                        Action::Toggle => state[pos] = !state[pos],
                        Action::TurnOff => state[pos] = false,
                        Action::TurnOn => state[pos] = true,
                    }
                }
            }
        }
        let mut result = 0;
        for is_lit in state {
            if is_lit {
                result += 1;
            }
        }
        println!("{}", result);
    }
    {
        let mut state = vec![0u16; 1_000_000];
        for line in input.lines() {
            let (_, (action, (p1, p2))) = parse(line).unwrap();
            for x in p1.0..=p2.0 {
                for y in p1.1..=p2.1 {
                    let pos = usize::try_from(y * 1000 + x).unwrap();
                    match action {
                        Action::Toggle => state[pos] += 2,
                        Action::TurnOff => state[pos] = state[pos].max(1) - 1,
                        Action::TurnOn => state[pos] += 1,
                    }
                }
            }
        }
        let mut result = 0u32;
        for brightness in state {
            result += brightness as u32;
        }
        println!("{}", result);
    }
}
