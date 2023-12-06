use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

const DATA: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            other => panic!("Found unexpected color '{}' found", other),
        }
    }
}

#[derive(Debug, Default)]
struct Game {
    id: u32,
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Add<Color> for Game {
    type Output = Game;

    fn add(self, color: Color) -> Game {
        // Game {
        //     id: self.id,
        //     reds: self.count(color),
        // }
        todo!()
    }
}

impl From<(u32, Vec<(u32, Color)>)> for Game {
    fn from((id, colors): (u32, Vec<(u32, Color)>)) -> Self {
        // colors.into_iter().fold(init, f)
        todo!()
    }
}

impl Game {
    fn count(&self, color: Color) -> u32 {
        match color {
            Color::Red => self.reds,
            Color::Green => self.greens,
            Color::Blue => self.blues,
        }
    }
}

fn parse_game_id(i: &str) -> IResult<&str, u32> {
    delimited(tag("Game "), map_res(digit1, str::parse), tag(": "))(i)
}

fn parse_color(i: &str) -> IResult<&str, (u32, Color)> {
    separated_pair(
        map_res(digit1, str::parse),
        tag(" "),
        map(alpha1, Color::from),
    )(i)
}

fn parse_colors(i: &str) -> IResult<&str, Vec<(u32, Color)>> {
    separated_list1(alt((tag(", "), tag("; "))), parse_color)(i)
}

fn parse_line(i: &str) -> IResult<&str, (u32, Vec<(u32, Color)>)> {
    tuple((parse_game_id, parse_colors))(i)
}

fn main() {
    let result = parse_colors("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    println!("result = {:#?}", result);

    let res = parse_game_id("Game 1: foo");
    println!("foo = {:#?}", res);
}

#[test]
fn parses_color() {
    let input = "1 blue";
    assert_eq!(parse_color(input), Ok(("", (1, Color::Blue))));
}

#[test]
fn parses_game_id() {
    assert_eq!(
        parse_game_id("Game 1: 3 blue, 4 red"),
        Ok(("3 blue, 4 red", 1))
    );
}

#[test]
fn parses_colors() {
    let input = "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    assert_eq!(
        parse_colors(input),
        Ok((
            "",
            vec![
                (1, Color::Blue,),
                (2, Color::Green,),
                (3, Color::Green,),
                (4, Color::Blue,),
                (1, Color::Red,),
                (1, Color::Green,),
                (1, Color::Blue,),
            ],
        ),)
    );
}

#[test]
fn parses_line() {
    let line = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(
        line,
        Ok((
            "",
            (
                1,
                vec![
                    (3, Color::Blue,),
                    (4, Color::Red,),
                    (1, Color::Red,),
                    (2, Color::Green,),
                    (6, Color::Blue,),
                    (2, Color::Green,),
                ],
            ),
        ))
    );
}
