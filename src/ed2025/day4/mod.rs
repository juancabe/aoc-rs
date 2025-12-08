use std::str::FromStr;
pub const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

pub enum GridItem {
    Roll,
    Empty,
}

impl FromStr for GridItem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "@" => Ok(Self::Roll),
            _ => Err(format!("Invalid str: {s}")),
        }
    }
}

pub struct Grid {
    pub mat: Vec<Vec<GridItem>>,
    pub columns: usize,
}

impl Grid {
    pub fn new(mat: Vec<Vec<GridItem>>) -> Self {
        let columns = if mat.iter().all(|r| r.len() == mat[0].len()) {
            mat[0].len()
        } else {
            panic!("Some row has diff num of elements")
        };
        Self { mat, columns }
    }

    pub fn remove_rolls(&mut self, to_remove: &[(usize, usize)]) {
        for (i, j) in to_remove.iter() {
            self.mat[*i][*j] = GridItem::Empty
        }
    }

    pub fn n_adjacent_rolls(&self, row: usize, col: usize) -> usize {
        let mut ret = 0;

        for i in [-1, 0, 1] {
            for j in [-1, 1] {
                let i: i32 = row as i32 + i;
                let j: i32 = col as i32 + j;
                if i >= 0
                    && i < self.mat.len() as i32
                    && j >= 0
                    && j < self.columns as i32
                    && let GridItem::Roll = self.mat[i as usize][j as usize]
                {
                    ret += 1;
                }
            }
        }

        for i in [-1, 1] {
            let i: i32 = row as i32 + i;
            if i >= 0
                && i < self.mat.len() as i32
                && let GridItem::Roll = self.mat[i as usize][col]
            {
                ret += 1
            }
        }

        ret
    }
}

pub fn parse_input(input: &str) -> Grid {
    let mat = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| GridItem::from_str(&char.to_string()).unwrap())
                .collect()
        })
        .collect();
    Grid::new(mat)
}

pub mod part1 {

    use super::*;

    pub fn solve(input: &Grid) -> usize {
        let mut ret = 0;

        for i in 0..input.mat.len() {
            for j in 0..input.columns {
                let adjacent = input.n_adjacent_rolls(i, j);
                if let GridItem::Roll = input.mat[i][j]
                    && adjacent < 4
                {
                    ret += 1;
                }
            }
        }
        ret
    }

    #[cfg(test)]
    mod test {

        use std::fs;

        use super::*;

        #[test]
        fn test_parse() {
            let res = parse_input(EXAMPLE);
            assert_eq!(res.columns, 10);
            assert_eq!(res.mat.len(), 10);
        }

        #[test]
        fn test_solve() {
            let input = parse_input(EXAMPLE);
            let res = solve(&input);
            assert_eq!(res, 13);

            let input = fs::read_to_string("./src/ed2025/day4/input.txt").unwrap();
            let input = parse_input(&input);
            let res = solve(&input);
            assert_eq!(res, 1451);
        }
    }
}

pub mod part2 {

    use super::*;

    pub fn solve(input: &mut Grid) -> usize {
        let mut removed = 0;
        let mut to_remove = Vec::new();

        let rows = input.mat.len();
        let cols = input.columns;

        loop {
            for i in 0..rows {
                for j in 0..cols {
                    if let GridItem::Roll = input.mat[i][j]
                        && input.n_adjacent_rolls(i, j) < 4
                    {
                        to_remove.push((i, j));
                    }
                }
            }

            if to_remove.is_empty() {
                break;
            }

            removed += to_remove.len();
            input.remove_rolls(&to_remove);
            to_remove.clear();
        }

        removed
    }

    #[cfg(test)]
    mod test {

        use std::fs;

        use super::*;

        #[test]
        fn test_solve() {
            let mut input = parse_input(EXAMPLE);
            let res = solve(&mut input);
            assert_eq!(res, 43);

            let input = fs::read_to_string("./src/ed2025/day4/input.txt").unwrap();
            let mut input = parse_input(&input);
            let res = solve(&mut input);
            assert_eq!(res, 8701);
        }
    }
}
