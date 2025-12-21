#[allow(dead_code)]
const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

pub mod part1 {

    pub fn solve(input: &str) -> usize {
        let mut buttons = Vec::with_capacity(2 >> 4);
        let mut to_visit = Vec::with_capacity(2 >> 8);
        let mut to_visit_temp = Vec::with_capacity(2 >> 8);

        #[derive(Default, Debug, Clone, Copy)]
        struct Lights(usize);

        impl Lights {
            pub fn is_empty(&self) -> bool {
                self.0 == 0
            }

            pub fn set(&mut self, light_index: usize) {
                assert!(light_index < usize::BITS as usize);
                self.0 |= 1usize << light_index
            }

            pub fn new(prev_lights: Lights, button: Lights) -> Self {
                Self(prev_lights.0 ^ button.0)
            }
        }

        input
            .lines()
            .map(|l| {
                let mut space_split = l.split(" ");
                let mut desired = space_split.next().unwrap().chars();
                assert_eq!(desired.next().unwrap(), '[');
                assert_eq!(desired.next_back().unwrap(), ']');
                let desired = desired
                    .map(|s| match s {
                        '.' => false,
                        '#' => true,
                        _ => panic!("unexpected light state"),
                    })
                    .enumerate()
                    .filter(|(_, b)| *b)
                    .map(|(i, _)| i);

                let mut desired_lights = Lights::default();
                for l in desired {
                    desired_lights.set(l);
                }

                let btns_i = space_split.filter_map(|parse| {
                    let mut i = parse.chars();
                    match (i.next(), i.next_back()) {
                        (Some('('), Some(')')) => Some(
                            i.as_str()
                                .split(",")
                                .map(|usz| usz.parse::<usize>().unwrap())
                                .fold(Lights::default(), |mut acc, e| {
                                    acc.set(e);
                                    acc
                                }),
                        ),
                        _ => None,
                    }
                });

                buttons.clear();
                for button in btns_i {
                    buttons.push(button)
                }

                let mut button_presses = 1usize;
                let first_layer = buttons.iter().map(|b| Lights::new(desired_lights, *b));

                to_visit.clear();
                to_visit.extend(first_layer);

                loop {
                    if to_visit.iter().any(|n| n.is_empty()) {
                        return button_presses;
                    }

                    button_presses += 1;
                    let temp = to_visit
                        .iter()
                        .flat_map(|n| buttons.iter().map(|b| Lights::new(*n, *b)));

                    to_visit_temp.clear();
                    to_visit_temp.extend(temp);

                    to_visit.clear();
                    to_visit.extend(to_visit_temp.iter());
                }
            })
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::super::*;
        use super::*;
        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 7);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day10/input.txt").unwrap()),
                404
            )
        }
    }
}

#[cfg(feature = "day10_lp")]
pub mod part2 {
    use std::iter::Sum;

    use good_lp::{Expression, Solution, SolverModel, variable, variables};

    #[derive(Default, Debug, Clone, Copy)]
    struct Button(usize);

    impl Button {
        pub fn set(&mut self, light_index: usize) {
            assert!(light_index < usize::BITS as usize);
            self.0 |= 1usize << light_index
        }

        pub fn is_set(&self, light_index: usize) -> bool {
            self.0 >> light_index & 1 == 1
        }
    }

    pub fn solve(input: &str) -> usize {
        let mut buttons = Vec::with_capacity(2 << 8);
        let mut button_vars = Vec::with_capacity(2 << 8);
        let mut joltages = Vec::with_capacity(2 << 8);

        input
            .lines()
            .map(|l| {
                let space_split = l.split(" ");
                let mut desired = space_split.clone().last().unwrap().chars();
                assert_eq!(desired.next().unwrap(), '{');
                assert_eq!(desired.next_back().unwrap(), '}');

                let desired = desired
                    .as_str()
                    .split(",")
                    .map(|d| d.parse::<u16>().unwrap());

                joltages.clear();
                joltages.extend(desired);

                let btns_i = space_split
                    // Skip the illustration of lights on or off
                    .skip(1)
                    .filter_map(|parse| {
                        let mut i = parse.chars();
                        match (i.next(), i.next_back()) {
                            (Some('('), Some(')')) => i
                                .as_str()
                                .split(",")
                                .map(|usz| usz.parse::<usize>().unwrap())
                                .try_fold(Button::default(), |mut acc, e| {
                                    if joltages[e] == 0 {
                                        None
                                    } else {
                                        acc.set(e);
                                        Some(acc)
                                    }
                                }),
                            _ => None,
                        }
                    });

                buttons.clear();
                for button in btns_i {
                    buttons.push(button)
                }

                // for every button B [{usize}, {usize}, ...] of len joltages.len()
                //   b_i element of B maps to light_i of joltages whose joltage will grow by b_i when B
                //   is pressed
                // We have an array of buttons (B) -> BTNS [B0, B1, ...]
                // We want a linear combination of BTNS that sums up exactly to joltages
                // such as: x * B0 + y * B1 + ... == joltages
                // Where x + y + ... = S / S is the minimum possible

                let mut vars = variables! {};

                button_vars.clear();
                for _ in &buttons {
                    button_vars.push(vars.add(variable().integer().min(0)));
                }

                let mut problem = vars
                    .minimise(Expression::sum(button_vars.iter()))
                    .using(good_lp::microlp);

                for (light_idx, &target_val) in joltages.iter().enumerate() {
                    let mut row_expression = Expression::from(0);

                    for (btn_idx, button) in buttons.iter().enumerate() {
                        let contribution = if button.is_set(light_idx) { 1 } else { 0 };
                        if contribution > 0 {
                            // Add (coefficient * variable) to the expression
                            row_expression += button_vars[btn_idx] * contribution;
                        }
                    }
                    problem = problem.with(row_expression.eq(target_val as i32));
                }

                let solution = problem.solve().unwrap();

                solution.eval(Expression::sum(button_vars.iter())).round() as usize
            })
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::super::*;
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 33);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day10/input.txt").unwrap()),
                16474
            )
        }
    }
}
