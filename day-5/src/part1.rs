#![warn(clippy::all, clippy::pedantic)]

pub fn part1(_input: &str) -> String {
    "Not Finished".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "Not Finished".to_string());
    }
}
