#![warn(clippy::all, clippy::pedantic)]

/// Day 1 Part 2 of AOC2023
///
/// # Arguments
/// - puzzle input
///
/// # Panics
/// this panics if there is no numbers in a line
pub fn part2(input: &str) -> String {
    let values = input.lines().map(parse_line).collect::<Vec<Vec<u32>>>();
    println!("{values:?}");
    values
        .iter()
        .map(|v| v.first().expect("There is always at least one number") * 10 + v.last().expect("there is always at least one number"))
        .sum::<u32>()
        .to_string()
}

fn parse_line(line: &str) -> Vec<u32> {
    (0..line.len())
        .filter_map(|index| {
            let reduced_line = &line[index..];
            let result = if reduced_line.starts_with("one") {
                Some(1)
            } else if reduced_line.starts_with("two") {
                Some(2)
            } else if reduced_line.starts_with("three") {
                Some(3)
            } else if reduced_line.starts_with("four") {
                Some(4)
            } else if reduced_line.starts_with("five") {
                Some(5)
            } else if reduced_line.starts_with("six") {
                Some(6)
            } else if reduced_line.starts_with("seven") {
                Some(7)
            } else if reduced_line.starts_with("eight") {
                Some(8)
            } else if reduced_line.starts_with("nine") {
                Some(9)
            } else if reduced_line.starts_with("zero") {
                Some(0)
            } else {
                reduced_line.chars().next().expect("there is alwayss a character").to_digit(10)
            };

            result
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "281".to_string());
    }
}
