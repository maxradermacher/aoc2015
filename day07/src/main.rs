use nom;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Input<'a> {
    Constant(u16),
    Wire(&'a str),
}

#[derive(Debug)]
enum Expression<'a> {
    Constant(Input<'a>),
    And(Input<'a>, Input<'a>),
    Or(Input<'a>, Input<'a>),
    LShift(Input<'a>, u16),
    RShift(Input<'a>, u16),
    Not(Input<'a>),
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    nom::branch::alt((
        nom::combinator::map(
            nom::character::complete::u16,
            |v| { Input::Constant(v) },
        ),
        nom::combinator::map(
            nom::character::complete::alpha1,
            |v| { Input::Wire(v) },
        ),
    ))(input)
}

fn parse_constant(input: &str) -> nom::IResult<&str, Expression> {
    nom::combinator::map(
        parse_input,
        |v| { Expression::Constant(v) },
    )(input)
}

fn parse_and_or(input: &str) -> nom::IResult<&str, Expression> {
    nom::combinator::map(
        nom::sequence::tuple((
            parse_input,
            nom::character::complete::space1,
            nom::branch::alt((
                nom::bytes::complete::tag("AND"),
                nom::bytes::complete::tag("OR"),
            )),
            nom::character::complete::space1,
            parse_input,
        )),
        |(lhs, _, op, _, rhs)| {
            match op {
                "AND" => Expression::And(lhs, rhs),
                "OR" => Expression::Or(lhs, rhs),
                _ => panic!("not possible"),
            }
        }
    )(input)
}

fn parse_shift(input: &str) -> nom::IResult<&str, Expression> {
    nom::combinator::map(
        nom::sequence::tuple((
            parse_input,
            nom::character::complete::space1,
            nom::branch::alt((
                nom::bytes::complete::tag("LSHIFT"),
                nom::bytes::complete::tag("RSHIFT"),
            )),
            nom::character::complete::space1,
            nom::character::complete::u16,
        )),
        |(lhs, _, op, _, rhs)| {
            match op {
                "LSHIFT" => Expression::LShift(lhs, rhs),
                "RSHIFT" => Expression::RShift(lhs, rhs),
                _ => panic!("not possible"),
            }
        },
    )(input)
}

fn parse_not(input: &str) -> nom::IResult<&str, Expression> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::bytes::complete::tag("NOT"),
            nom::character::complete::space1,
            parse_input,
        )),
        |(_, _, rhs)| { Expression::Not(rhs) },
    )(input)
}
fn parse_expression(input: &str) -> nom::IResult<&str, Expression> {
    nom::branch::alt((
        parse_not,
        parse_constant,
        parse_and_or,
        parse_shift,
    ))(input)
}

fn parse(input: &str) -> nom::IResult<&str, (Expression, &str)> {
    nom::sequence::separated_pair(
        parse_expression,
        nom::bytes::complete::tag(" -> "),
        nom::character::complete::alpha1
    )(input)
}

fn main() {
    let input = fs::read_to_string("07.txt").unwrap();
    let mut state: HashMap<&str, u16> = HashMap::new();
    loop {
        for line in input.lines() {
            println!("{}", line);
            let (_, (expression, variable)) = parse(line).unwrap();
            println!("{:?}", expression);
        }
        if let Some(signal) = state.get(&"a") {
            println!("{}", signal);
            break;
        }
        println!("No match!");
        break;
    }
}
