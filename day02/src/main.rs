use nom;
use std::fs;

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<i32>>> {
    nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::separated_list1(
            nom::bytes::complete::tag("x"),
            nom::character::complete::i32,
        ),
    )(input)
}

fn main() {
    let input = fs::read_to_string("02.txt").unwrap();
    let (_, mut boxes) = parse(&input).unwrap();
    for bx in &mut boxes {
        bx.sort();
    }

    let mut paper = 0;
    for bx in &boxes {
        paper += 3 * bx[0] * bx[1] + 2 * bx[1] * bx[2] + 2 * bx[0] * bx[2];
    }
    println!("{}", paper);

    let mut ribbon = 0;
    for bx in &boxes {
        ribbon += 2 * bx[0] + 2 * bx[1] + bx[0] * bx[1] * bx[2];
    }
    println!("{}", ribbon);
}
