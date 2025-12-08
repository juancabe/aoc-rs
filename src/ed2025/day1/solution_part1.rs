use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

use crate::ed2025::day1::{Count, MODULE, Movement};

pub fn count_zero_times<S: AsRef<str>, const MODULE: usize>(
    lines: impl Iterator<Item = io::Result<S>>,
) -> usize {
    let mut times: usize = 0;
    let mut count = Count::<100>(50);
    for line in lines {
        match Movement::from_str(line.unwrap().as_ref()).unwrap() {
            Movement::Left(n) => count.move_left(n),
            Movement::Right(n) => count.move_right(n),
        };

        if count.get_inner() == 0 {
            times += 1
        }
    }
    times
}

pub fn parse_sol() -> usize {
    let file = File::open("./src/ed2025/day1/input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    count_zero_times::<_, { MODULE }>(lines)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_zero_times() {
        let lines = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(count_zero_times::<_, MODULE>(lines.lines().map(Ok)), 3);
    }

    #[test]
    fn test_parse_sol() {
        assert_eq!(parse_sol(), 1154);
    }
}
