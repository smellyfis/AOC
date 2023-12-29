#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::similar_names)]

use glam::I64Vec3;
use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

use itertools::Itertools;

use num::rational::Ratio;

type Cord = Ratio<i128>;

#[derive(Debug, Clone, Copy)]
struct Stones {
    pub start: I64Vec3,
    pub velocity: I64Vec3,
}
impl Stones {
    fn cross_xy(&self, other: &Self) -> Option<(Cord, Cord)> {
        let pax = Cord::from(i128::from(self.start.x));
        let pay = Cord::from(i128::from(self.start.y));
        let pbx = Cord::from(i128::from(other.start.x));
        let pby = Cord::from(i128::from(other.start.y));
        let vax = Cord::from(i128::from(self.velocity.x));
        let vay = Cord::from(i128::from(self.velocity.y));
        let vbx = Cord::from(i128::from(other.velocity.x));
        let vby = Cord::from(i128::from(other.velocity.y));
        let slope1 = vay / vax;
        let slope2 = vby / vbx;
        let denom = slope1 - slope2;
        if denom == num::Zero::zero() {
            return None;
        }
        let x = (pax * slope1 - pbx * slope2 + pby - pay) / denom;
        let y = slope1 * (x - pax) + pay;
        let t1 = (x - pax) / vax;
        let t2 = (x - pbx) / vbx;
        if t1 < num::zero() || t2 < num::zero() {
            return None;
        }
        Some((x, y))
    }

    fn cross_xz(&self, other: &Self) -> Option<(Cord, Cord)> {
        let pax = Cord::from(i128::from(self.start.x));
        let paz = Cord::from(i128::from(self.start.z));
        let pbx = Cord::from(i128::from(other.start.x));
        let pbz = Cord::from(i128::from(other.start.z));
        let vax = Cord::from(i128::from(self.velocity.x));
        let vaz = Cord::from(i128::from(self.velocity.z));
        let vbx = Cord::from(i128::from(other.velocity.x));
        let vbz = Cord::from(i128::from(other.velocity.z));
        let slope1 = vaz / vax;
        let slope2 = vbz / vbx;
        let denom = slope1 - slope2;
        if denom == num::Zero::zero() {
            return None;
        }
        let x = (pax * slope1 - pbx * slope2 + pbz - paz) / denom;
        let z = slope1 * (x - pax) + paz;
        let t1 = (x - pax) / vax;
        let t2 = (x - pbx) / vbx;
        if t1 < num::zero() || t2 < num::zero() {
            return None;
        }
        Some((x, z))
    }
    fn stone_to_tuples(&self) -> Stone {
        let Stones {
            start:
                I64Vec3 {
                    x: p_x,
                    y: p_y,
                    z: p_z,
                },
            velocity:
                I64Vec3 {
                    x: v_x,
                    y: v_y,
                    z: v_z,
                },
        } = *self;
        (
            (
                Cord::from(i128::from(p_x)),
                Cord::from(i128::from(p_y)),
                Cord::from(i128::from(p_z)),
            ),
            (
                Cord::from(i128::from(v_x)),
                Cord::from(i128::from(v_y)),
                Cord::from(i128::from(v_z)),
            ),
        )
    }
}

type Stone = ((Cord, Cord, Cord), (Cord, Cord, Cord));

fn get_missing_stone(stone1: Stone, stone2: Stone, stone3: Stone) -> Stone {
    let ((p_ax, p_ay, p_az), (v_ax, v_ay, v_az)) = stone1;
    let ((p_bx, p_by, p_bz), (v_bx, v_by, v_bz)) = stone2;
    let ((p_cx, p_cy, p_cz), (v_cx, v_cy, v_cz)) = stone3;
    //
    //setting up the syystem of equations
    //println!("{v_az} - {v_cz} = {:?}", v_az - v_cz);
    let mut equations = [
        [
            Cord::default(),
            v_az - v_cz,
            v_cy - v_ay,
            Cord::default(),
            p_cz - p_az,
            p_ay - p_cy,
            p_ay * v_az - p_az * v_ay - p_cy * v_cz + p_cz * v_cy,
        ],
        [
            v_az - v_cz,
            Cord::default(),
            v_cx - v_ax,
            p_cz - p_az,
            Cord::default(),
            p_ax - p_cx,
            p_ax * v_az - p_az * v_ax - p_cx * v_cz + p_cz * v_cx,
        ],
        [
            v_cy - v_ay,
            v_ax - v_cx,
            Cord::default(),
            p_ay - p_cy,
            p_cx - p_ax,
            Cord::default(),
            p_ay * v_ax - p_ax * v_ay - p_cy * v_cx + p_cx * v_cy,
        ],
        [
            Cord::default(),
            v_bz - v_cz,
            v_cy - v_by,
            Cord::default(),
            p_cz - p_bz,
            p_by - p_cy,
            p_by * v_bz - p_bz * v_by - p_cy * v_cz + p_cz * v_cy,
        ],
        [
            v_bz - v_cz,
            Cord::default(),
            v_cx - v_bx,
            p_cz - p_bz,
            Cord::default(),
            p_bx - p_cx,
            p_bx * v_bz - p_bz * v_bx - p_cx * v_cz + p_cz * v_cx,
        ],
        [
            v_cy - v_by,
            v_bx - v_cx,
            Cord::default(),
            p_by - p_cy,
            p_cx - p_bx,
            Cord::default(),
            p_by * v_bx - p_bx * v_by - p_cy * v_cx + p_cx * v_cy,
        ],
    ];

    //println!("{equations:?}");
    //gausian elimination
    for i in 0..6 {
        //need to  make sure i,i is not zero
        let none_zero_index = (i..6)
            .find(|&row| equations[row][i] != Cord::default())
            .unwrap();
        //perform swap if need be
        if i != none_zero_index {
            (equations[i], equations[none_zero_index]) = (equations[none_zero_index], equations[i]);
        }

        // make i,i 1 and adjust the row
        let current_leader = equations[i][i];
        equations[i][i] = Cord::from_integer(1);
        for pos in &mut equations[i][(i + 1)..] {
            *pos /= current_leader;
        }

        //gausean elimination
        for row in (i + 1)..6 {
            let mult = equations[row][i];
            if mult != Cord::default() {
                equations[row][i] = Cord::default();
                for col in (i + 1)..7 {
                    equations[row][col] -= equations[i][col] * mult;
                }
            }
        }
    }
    //upper triangle done now to make it identity
    //but doing a trick to turn col 6
    for i in (0..6).rev() {
        for row in 0..i {
            //equality row
            equations[row][6] -= equations[i][6] * equations[row][i];
            equations[row][i] = Cord::default();
        }
    }
    (
        (equations[0][6], equations[1][6], equations[2][6]),
        (equations[3][6], equations[4][6], equations[5][6]),
    )
}

/// day 24 part 2 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part2(input: &str) -> String {
    let (_, stones) = parse_input(input).expect("Aoc should have valid input");
    let iteresting_stones = stones
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            let [a, b] = pair[..] else {
                return None;
            };
            match (a.cross_xy(b), a.cross_xz(b)) {
                (None, None) => false,
                (None, Some(_)) | (Some(_), None) => true,
                (Some((x1, _)), Some((x2, _))) => x1 != x2,
            }
            .then_some(*b)
        })
        .skip(4)
        .take(3)
        .collect::<Vec<_>>();
    let (position, _) = get_missing_stone(
        iteresting_stones[0].stone_to_tuples(),
        iteresting_stones[1].stone_to_tuples(),
        iteresting_stones[2].stone_to_tuples(),
    );

    (position.0.to_integer() + position.1.to_integer() + position.2.to_integer()).to_string()
}

fn parse_tuple(input: &str) -> IResult<&str, I64Vec3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = tuple((tag(","), complete::space0))(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = tuple((tag(","), complete::space0))(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, I64Vec3::new(x, y, z)))
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
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "47".to_string());
    }
}
