#![warn(clippy::all, clippy::pedantic)]

use glam::DVec3;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    number,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Stones {
    pub start: DVec3,
    pub velocity: DVec3,
}
impl Stones {
    fn cross(&self, other: &Self) -> Option<DVec3> {
        // x1 + v_x1 *t = x_n
        // t= (x_n - x_1)/v_x1
        // (y_n - y_1)/v_y1 = (x_n - x_1)/v_x1
        // y_n = v_y1 * (x_n -x_1)/v_x1 + y_1
        // (v_y1/v_x1) * x_n - x_1 * (v_y1/v_x1) + y_1 = (v_y2/v_x2) * x_n -x_2 * (v_y2/v_x2) + y_2
        // x_n * ((v_y1/v_x1) - (v_y2/v_x2)) = x_1 * (v_y1/v_x1) - x_2 * (v_y2/v_x2) + y_2 - y_1
        // x1 + v1 *t == x2 + v2*t
        // (x1-x2)/(v2-v1) = t
        let slope1 = self.velocity.y / self.velocity.x;
        let slope2 = other.velocity.y / other.velocity.x;
        let denom = slope1 - slope2;
        let x =
            (self.start.x * slope1 - other.start.x * slope2 + other.start.y - self.start.y) / denom;
        let y = slope1 * (x - self.start.x) + self.start.y;
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        let t1 = (x - self.start.x) / self.velocity.x;
        let t2 = (x - other.start.x) / other.velocity.x;
        if t1 < 0.0 || t2 < 0.0 {
            return None;
        }
        Some(DVec3::new(x, y, 0.0))
    }
}

/// day 24 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str, min: f64, max: f64) -> String {
    let (_, stones) = parse_input(input).expect("Aoc should have valid input");
    stones
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            let [a, b] = pair[..] else { return None };
            a.cross(b).and_then(|cross_position| {
                (cross_position.x <= max
                    && cross_position.y <= max
                    && cross_position.x >= min
                    && cross_position.y >= min)
                    .then_some(cross_position)
            })
        })
        .count()
        .to_string()
}

fn parse_tuple(input: &str) -> IResult<&str, DVec3> {
    let (input, x) = number::complete::double(input)?;
    let (input, _) = tuple((tag(","), complete::space0))(input)?;
    let (input, y) = number::complete::double(input)?;
    let (input, _) = tuple((tag(","), complete::space0))(input)?;
    let (input, z) = number::complete::double(input)?;
    Ok((input, DVec3::new(x, y, z)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Stones>> {
    separated_list1(
        complete::line_ending,
        separated_pair(
            parse_tuple,
            tuple((complete::space0, tag("@"), complete::space0)),
            parse_tuple,
        )
        .map(|(start, velocity)| Stones { start, velocity }),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_works() {
        let result = part1(INPUT, 7.0, 27.0);
        assert_eq!(result, "2".to_string());
    }
}
