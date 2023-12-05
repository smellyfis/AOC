#![warn(clippy::all, clippy::pedantic)]

#[must_use] pub fn part2(_input: &str) -> String {
    "Not Finished".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "Not Finished".to_string());
    }
}
