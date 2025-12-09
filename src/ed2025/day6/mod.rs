use std::str::FromStr;

pub const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Sum,
    Mul,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Sum),
            "*" => Ok(Self::Mul),
            _ => Err(format!("String: [{s}] is not valid Operation")),
        }
    }
}

pub mod part1 {

    use super::*;

    pub fn parse_input(input: &str) -> Vec<Vec<String>> {
        input
            .lines()
            .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
            .collect()
    }

    pub fn solve(input: &[Vec<String>]) -> u128 {
        let mut sum: u128 = 0;
        let op_row = input.len() - 1;

        let n_cols = input[0].len();
        let n_rows = input.len();

        for col in 0..n_cols {
            let op = Operation::from_str(&input[op_row][col]).unwrap();
            let mut col_res: u128 = input[0][col].parse().unwrap();

            #[allow(clippy::needless_range_loop)]
            for row in 1..n_rows - 1 {
                let val: u128 = input[row][col].parse().unwrap();
                match op {
                    Operation::Sum => col_res += val,
                    Operation::Mul => col_res *= val,
                }
            }

            sum += col_res
        }

        sum
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse() {
            let input = parse_input(EXAMPLE);
            assert_eq!(input.len(), 4);
            assert_eq!(input[0].len(), 4);
        }

        #[test]
        fn test_solve() {
            let input = parse_input(EXAMPLE);
            assert_eq!(solve(&input), 4277556);

            let input =
                parse_input(&std::fs::read_to_string("./src/ed2025/day6/input.txt").unwrap());
            assert_eq!(solve(&input), 5784380717354);
        }
    }
}

pub mod part2 {

    use super::*;

    #[derive(Clone, Debug)]
    pub struct Set {
        nums: Vec<Vec<char>>,
        op: Option<Operation>,
    }

    impl Set {
        pub fn new_iter(&mut self, op: Operation) {
            self.op = Some(op)
        }

        pub fn new(n_lines: usize) -> Self {
            Self {
                nums: Vec::from_iter(std::iter::repeat_n(Vec::new(), n_lines - 1)),
                op: None,
            }
        }

        pub fn clear(&mut self) {
            for num_vec in &mut self.nums {
                num_vec.clear();
            }
            self.op = None
        }

        pub fn is_valid(&self) -> bool {
            self.nums
                .iter()
                .all(|n| !n.is_empty() && n.len() == self.nums[0].len())
                && self.op.is_some()
        }

        pub fn is_empty(&self) -> bool {
            self.nums.iter().all(|n| n.is_empty()) && self.op.is_none()
        }

        pub fn add_chars(&mut self, chars: &[char]) {
            assert_eq!(chars.len(), self.nums.len());
            for (index, char) in chars.iter().enumerate() {
                self.nums[index].push(*char);
            }
        }

        pub fn operate(&self) -> u128 {
            assert!(self.is_valid());
            let mut res = None;
            let n_cols = self.nums[0].len();
            let n_rows = self.nums.len();
            let op = self.op.unwrap();

            for col in 0..n_cols {
                let mut col_acc: Vec<char> = vec![];
                for row in 0..n_rows {
                    let to_parse = self.nums[row][col];

                    if !to_parse.is_ascii_digit() {
                        continue;
                    }

                    col_acc.push(to_parse);
                }

                // INFO: I know parsing the numbers from a String is not efficient, this should
                // be done in the parse_input function with math (i.e., multiplying by 10)
                let num = col_acc
                    .iter()
                    .collect::<String>()
                    .parse::<u128>()
                    .expect("All cols should have a number");

                if let Some(res) = &mut res {
                    match op {
                        Operation::Sum => *res += num,
                        Operation::Mul => *res *= num,
                    }
                } else {
                    res = Some(num);
                }
            }

            res.unwrap()
        }
    }

    pub fn parse_input(input: &str) -> Vec<Set> {
        let mut sets = vec![];

        let lines = input.lines().map(|l| l.chars().collect::<Vec<char>>());
        let line_len = lines.clone().next().unwrap().len();
        let lines: Vec<Vec<char>> = lines.collect();
        let mut chars = vec![];
        let mut actual_set: Set = Set::new(lines.len());

        for i in 0..line_len {
            // Clear chars for next iteration
            chars.clear();

            // Populate chars
            for line in &lines[..lines.len() - 1] {
                chars.push(line[i]);
            } // and operator
            let operator = lines.last().unwrap().get(i);

            let operator = operator.unwrap();

            // Need new set? op should be present
            if actual_set.is_empty() {
                actual_set.clear();
                actual_set.new_iter(
                    Operation::from_str(&operator.to_string()).expect("op should be present"),
                );
            }

            // End of set
            if chars.iter().all(|c| c.is_whitespace()) {
                assert!(actual_set.is_valid());
                sets.push(actual_set.clone());
                actual_set.clear();
                continue;
            }

            // Add chars
            actual_set.add_chars(&chars);
        }

        sets.push(actual_set);

        sets
    }

    pub fn solve(input: &[Set]) -> u128 {
        input.iter().map(|s| s.operate()).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse_input() {
            let sets = parse_input(EXAMPLE);
            assert_eq!(sets.len(), 4);
        }

        #[test]
        fn test_solve() {
            let sets = parse_input(EXAMPLE);
            let res = solve(&sets);
            assert_eq!(res, 3263827);

            let sets =
                parse_input(&std::fs::read_to_string("./src/ed2025/day6/input.txt").unwrap());
            let res = solve(&sets);
            assert_eq!(res, 7996218225744);
        }
    }
}
