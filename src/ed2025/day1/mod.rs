use std::str::FromStr;

pub mod solution_part1;
pub mod solution_part2;

pub const MODULE: usize = 100;

#[derive(PartialEq, Debug)]
pub enum Movement {
    Left(usize),
    Right(usize),
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some(movement) => match movement {
                'L' | 'R' => {
                    let num = match s.strip_prefix(movement).map(|n| n.parse::<usize>()) {
                        Some(num) => {
                            num.map_err(|e| format!("Error found when parsing distance: {e}"))?
                        }
                        None => Err("Invalid str after stripping L or R prefix".to_string())?,
                    };
                    match movement {
                        'L' => Ok(Self::Left(num)),
                        'R' => Ok(Self::Right(num)),
                        _ => panic!(),
                    }
                }
                _ => Err("Non leaded by R or L str cannot be a Movement".to_string()),
            },
            None => Err("Empty str cannot be a Movement".to_string()),
        }
    }
}

pub struct Count<const MODULE: usize>(usize);

impl<const M: usize> From<usize> for Count<M> {
    fn from(value: usize) -> Self {
        Self(value % MODULE)
    }
}

impl<const M: usize> From<i32> for Count<M> {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self(MODULE - (value % MODULE as i32).unsigned_abs() as usize)
        } else {
            Self(value as usize % MODULE)
        }
    }
}

impl<const M: usize> Count<M> {
    pub fn get_inner(&self) -> usize {
        self.0
    }

    pub fn move_left(&mut self, movement: usize) -> usize {
        let Count(inner) = self;
        let extra_spins = movement / M;
        let starts_at_zero = *inner == 0;

        let movement = movement % M;

        if *inner >= movement {
            *inner -= movement;
            extra_spins
        } else {
            *inner = M - (movement - *inner);
            extra_spins + if *inner != 0 && !starts_at_zero { 1 } else { 0 }
        }
    }

    pub fn move_right(&mut self, movement: usize) -> usize {
        let Count(inner) = self;
        let extra_spins = movement / M;

        let movement = movement % M;

        if *inner + movement >= M {
            *inner = *inner + movement - M;
            extra_spins + if *inner != 0 { 1 } else { 0 }
        } else {
            *inner += movement;
            extra_spins
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_movement_from_str() {
        let movement = "R49";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Right(49));
        let movement = "R27";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Right(27));
        let movement = "R22";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Right(22));
        let movement = "R5";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Right(5));
        let movement = "R6";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Right(6));
        let movement = "R10";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Right(10));
        let movement = "L13";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Left(13));
        let movement = "L37";
        assert_eq!(Movement::from_str(movement).unwrap(), Movement::Left(37));
    }
}
