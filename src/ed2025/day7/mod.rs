pub const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

pub mod part1 {

    #[allow(unused_imports)]
    use super::*;

    pub fn solve(input: &str) -> usize {
        let mut lines = input.lines();

        let first_beam_p = lines
            .next()
            .unwrap()
            .chars()
            .position(|s| s == 'S')
            .unwrap();

        let mut splits = 0;

        // This vec is sorted
        let mut beam_positions = vec![first_beam_p];

        let mut to_push_beams: Vec<usize> = vec![];

        for line in lines {
            let splitters = line
                .char_indices()
                .filter(|(_, c)| *c == '^')
                .map(|(i, _)| i);

            for bp in beam_positions.iter_mut() {
                if splitters.clone().any(|e| e == *bp) {
                    splits += 1;

                    if *bp > 0 {
                        to_push_beams.push(*bp - 1);
                    }
                    *bp += 1;
                }
            }

            for bp in &to_push_beams {
                beam_positions.push(*bp);
            }

            to_push_beams.clear();

            beam_positions.sort();
            beam_positions.dedup();
        }

        splits
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 21);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day7/input.txt").unwrap()),
                1592
            )
        }
    }
}

pub mod part2 {

    #[allow(unused_imports)]
    use super::*;

    pub fn solve(input: &str) -> usize {
        let mut lines = input.lines();

        let first_beam_p = lines
            .next()
            .unwrap()
            .chars()
            .position(|s| s == 'S')
            .unwrap();

        let lines: Vec<&str> = lines.collect();

        let fill_iter = std::iter::repeat_n(0usize, lines[0].len());

        let mut timelines = Vec::from_iter(fill_iter.clone());
        timelines[first_beam_p] = 1;

        let mut next_row_timelines = Vec::from_iter(fill_iter);

        for line in lines {
            next_row_timelines.clone_from_slice(&timelines);
            for (idx, _) in line.char_indices().filter(|(_, c)| *c == '^') {
                next_row_timelines[idx + 1] += next_row_timelines[idx];
                next_row_timelines[idx - 1] += next_row_timelines[idx];
                next_row_timelines[idx] = 0;
            }
            timelines.clone_from_slice(&next_row_timelines);
        }

        timelines.iter().sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 40);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day7/input.txt").unwrap()),
                17921968177009
            )
        }
    }
}
