use std::default;

use nom::{branch::alt, bytes::complete::tag, error::Error, Parser};

#[derive(Debug, Clone, Copy)]
enum Shape {
    Bar,
    Plus,
    El,
    Staff,
    Square,
}

impl Shape {
    pub fn iter() -> ShapeGenerator {
        ShapeGenerator::new()
    }
}

struct ShapeGenerator {
    last_shape: Shape,
}
impl ShapeGenerator {
    fn new() -> Self {
        Self {
            last_shape: Shape::Square,
        }
    }
}

impl Iterator for ShapeGenerator {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_shape = match self.last_shape {
            Shape::Bar => Shape::Plus,
            Shape::Plus => Shape::El,
            Shape::El => Shape::Staff,
            Shape::Staff => Shape::Square,
            Shape::Square => Shape::Bar,
        };
        Some(self.last_shape)
    }
}

enum Jet {
    Left,
    Right,
}

impl<'a> Parser<&'a str, Self, Error<&'a str>> for Jet {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, Self, Error<&'a str>> {
        alt((tag("<").map(|_| Self::Left), tag(">").map(|_| Self::Right)))(input)
    }
}
fn main() {
    let shapes = Shape::iter().take(10).collect::<Vec<_>>();
    println!("{shapes:#?}");
}
