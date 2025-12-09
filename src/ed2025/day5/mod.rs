pub const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

pub mod part1 {

    pub struct IdRange {
        start: usize,
        end: usize,
    }

    pub fn solve(input: &str) -> usize {
        let mut fresh_ids: Vec<IdRange> = vec![];
        let mut are_fresh = 0;

        for s_line in input.lines() {
            let line = s_line.trim();

            if line.is_empty() {
                continue;
            }

            let mut line = line.split("-");
            let first: usize = line.next().unwrap().parse().unwrap();

            match line.next() {
                Some(second) => {
                    let end = second.parse().unwrap();
                    assert!(end >= first, "Line {s_line}");
                    fresh_ids.push(IdRange { start: first, end });
                }
                None => {
                    for IdRange { start, end } in &fresh_ids {
                        if first >= *start && first <= *end {
                            are_fresh += 1;
                            break;
                        }
                    }
                }
            }
        }

        are_fresh
    }

    #[cfg(test)]
    mod test {

        use crate::ed2025::day5::EXAMPLE;

        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 3);

            let input = std::fs::read_to_string("./src/ed2025/day5/input.txt").unwrap();
            assert_eq!(solve(&input), 821);
        }
    }
}

pub mod part2 {
    use std::cmp::max;

    pub struct IdRange {
        start: usize,
        end: usize,
    }

    pub fn solve(input: &str) -> usize {
        let mut fresh_ids: Vec<IdRange> = vec![];
        let mut all_fresh = 0;

        for s_line in input.lines() {
            let line = s_line.trim();

            if line.is_empty() {
                continue;
            }

            let mut line = line.split("-");
            let first: usize = line.next().unwrap().parse().unwrap();

            if let Some(second) = line.next() {
                let end = second.parse().unwrap();
                assert!(end >= first, "Line {s_line}");
                fresh_ids.push(IdRange { start: first, end });
            }
        }

        fresh_ids.sort_by_key(|IdRange { start, end: _ }| *start);

        let mut last_end = 0;
        for IdRange { start, end } in fresh_ids {
            let start = max(last_end, start);
            if end >= start {
                all_fresh += end - start + 1;
                last_end = end + 1;
            }
        }

        all_fresh
    }

    #[cfg(test)]
    mod test {

        use crate::ed2025::day5::EXAMPLE;

        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 14);

            let input = std::fs::read_to_string("./src/ed2025/day5/input.txt").unwrap();
            assert_eq!(solve(&input), 344771884978261);
        }
    }
}
