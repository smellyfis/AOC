#![warn(clippy::all, clippy::pedantic)]

/// day 15 part 1 of aoc 2023
///
/// # Arguments
/// - input the input for today's puzzle
///
/// # Panics
/// panics whne it cannot parse the input OR when ever the number of game numbers is greater than
#[must_use]
pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|x| {
            x.split(',')
                .map(unhash)
                .sum::<usize>()
                .to_string()
        })
        .next()
        .unwrap()
}

fn unhash(hash: &str) -> usize {
    hash.chars()
        .fold(0, |acc, x| (acc + (x as usize)) * 17 % 256)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    #[case("kqzb-\n", 127)]
    #[case("dx-", 153)]
    fn hash_test(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(unhash(input), expected);
    }

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "1320".to_string());
    }
}
