pub mod part1 {

    pub type ParsedInput = Vec<Vec<u32>>;

    pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
        input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect()
            })
            .collect()
    }

    pub fn solution(input: &ParsedInput) -> u32 {
        input
            .iter()
            .map(|bank| {
                let (max_p, max) = bank[..bank.len() - 1]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by_key(|(_p, e)| **e)
                    .unwrap();
                *max * 10 + bank[max_p + 1..].iter().max().unwrap()
            })
            .sum()
    }

    #[cfg(test)]
    mod test {

        use std::fs;

        use super::*;

        const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
        #[test]
        fn test_parse_input() {
            let res = parse_input(EXAMPLE);

            assert_eq!(res.len(), 4);
            assert_eq!(res[0].len(), 15);
            assert_eq!(res[0][0], 9)
        }

        #[test]
        fn test_solution() {
            let input = parse_input(EXAMPLE);
            let sol = solution(&input);
            assert_eq!(sol, 357);

            let input = fs::read_to_string("./src/ed2025/day3/input.txt").unwrap();
            let input = parse_input(&input);

            let sol = solution(&input);
            assert_eq!(sol, 17405);
        }
    }
}

pub mod part2 {

    pub type ParsedInput = Vec<Vec<u32>>;

    pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
        input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect()
            })
            .collect()
    }
    const MAX_BATTERIES: usize = 12;

    /// Algorithm description:
    /// for b bank
    /// 1. Look for the biggest battery on the first index possible on b -> (pos, val)
    ///    where pos < b.len() - 12 (*1) && pos > last_pos
    /// 3. Look for the biggest battery on the first index possible on b -> (pos, val)
    ///    where pos < b.len() - 11 && pos > last_pos
    ///    ...
    fn biggest_slice(bank: &[u32]) -> Vec<u32> {
        let mut res = vec![];
        let mut last_pos = -1;

        for i in 0..MAX_BATTERIES {
            let (pos, val) = bank[.. 
                // (*1)
                bank.len() - (MAX_BATTERIES - i - 1)]
                .iter()
                .enumerate()
                .filter(|(pos, _)| *pos as i32 > last_pos)
                .rev()
                .max_by_key(|(_p, val)| **val)
                .unwrap();

            res.push(*val);
            last_pos = pos as i32;
        }

        res
    }

    pub fn solution(input: &ParsedInput) -> u128 {
        let mut sum: u128 = 0;

        for bank in input {
            let bs = biggest_slice(bank);

            let bank_sum: u128 = bs
                .into_iter()
                .rev()
                .enumerate()
                .map(|(pos, elem)| (elem as u128) * 10u128.pow(pos as u32))
                .sum();

            dbg!(bank_sum);

            sum += bank_sum;
        }

        sum
    }

    #[cfg(test)]
    mod test {

        use std::fs;

        use super::*;

        const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
        #[test]
        fn test_parse_input() {
            let res = parse_input(EXAMPLE);

            assert_eq!(res.len(), 4);
            assert_eq!(res[0].len(), 15);
            assert_eq!(res[0][0], 9)
        }

        #[test]
        fn test_solution() {
            let input = parse_input(EXAMPLE);
            let sol = solution(&input);
            assert_eq!(sol, 3121910778619);

            let input = fs::read_to_string("./src/ed2025/day3/input.txt").unwrap();
            let input = parse_input(&input);

            let sol = solution(&input);
            assert_eq!(sol, 171990312704598);
        }
    }
}
