#[allow(dead_code)]
const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

const N_SHAPES: usize = 6;
const SHAPE_DIM: usize = 3;

pub mod part1 {

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Shape([[bool; SHAPE_DIM]; SHAPE_DIM]);

    #[derive(Clone, Copy, Debug)]
    pub struct BitShape {
        mem: [u8; SHAPE_DIM],
        area: usize,
    }

    impl From<Shape> for BitShape {
        fn from(value: Shape) -> Self {
            let mut ret = [0; SHAPE_DIM];
            let mut area = 0;
            #[allow(clippy::needless_range_loop)]
            for x in 0..SHAPE_DIM {
                let mut rx = 0;
                for y in 0..SHAPE_DIM {
                    if value.0[x][y] {
                        area += 1;
                        rx |= 1 << y;
                    }
                }
                ret[x] = rx
            }

            Self { mem: ret, area }
        }
    }

    impl Shape {
        pub fn rotate_90(&self) -> Self {
            let mut new_mem = [[false; SHAPE_DIM]; SHAPE_DIM];

            for r in 0..SHAPE_DIM {
                #[allow(clippy::needless_range_loop)]
                for c in 0..SHAPE_DIM {
                    new_mem[c][(SHAPE_DIM - 1) - r] = self.0[r][c]
                }
            }

            Shape(new_mem)
        }

        pub fn get_rotations(&self) -> [Shape; 4] {
            let r0 = Self(self.0);
            let r1 = r0.rotate_90();
            let r2 = r1.rotate_90();
            let r3 = r2.rotate_90();
            [r0, r1, r2, r3]
        }

        pub fn get_rotations_bits(&self) -> [BitShape; 4] {
            let mut i = self.get_rotations().into_iter().map(BitShape::from);
            std::array::from_fn(|_| i.next().unwrap())
        }
    }

    type Region = ((usize, usize), [usize; N_SHAPES]);

    pub fn parse_input(input: &str) -> ([Shape; N_SHAPES], Vec<Region>) {
        let mut shapes: Vec<Shape> = Vec::with_capacity(N_SHAPES);
        let mut regions: Vec<Region> = Vec::new();

        let mut actual_shape = None;

        for line in input.lines() {
            // Shape index
            if !line.contains("x") && line.contains(":") {
                assert_eq!(actual_shape, None);
                actual_shape = Some(([[false; SHAPE_DIM]; SHAPE_DIM], 0usize));
                continue;
            }

            // We are parsing a shape
            if let Some((shape, i)) = &mut actual_shape {
                if *i == SHAPE_DIM {
                    // End of shape
                    assert!(line.is_empty());
                    shapes.push(Shape(*shape));
                    actual_shape = None;
                    continue;
                }

                for (j, char) in line.chars().enumerate() {
                    shape[*i][j] = char == '#'
                }
                *i += 1;

                continue;
            }

            // Parse regions part
            assert!(line.contains("x"));
            let mut parts = line.split(":");
            let mut dimensions = parts.next().unwrap().split("x");
            let dimensions: (usize, usize) = (
                dimensions.next().unwrap().parse().unwrap(),
                dimensions.next().unwrap().parse().unwrap(),
            );

            let mut to_assign_desired = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|p| p.parse::<usize>().unwrap());
            let desired_parts = std::array::from_fn(|_| to_assign_desired.next().unwrap());

            regions.push((dimensions, desired_parts));
        }

        (shapes.try_into().unwrap(), regions)
    }

    #[derive(Debug, Clone, Copy)]
    struct Row {
        mem: usize,
        len: usize,
    }

    impl Row {
        pub fn zeros_len(len: usize) -> Self {
            assert!((len as u32) < usize::BITS);
            Self { mem: 0usize, len }
        }
    }

    struct WorkingRegion {
        // each bit: 1 if occupied, 0 if empty
        mem: Vec<Row>,

        desired_shapes: [(usize, [BitShape; 4]); N_SHAPES],
        free_tiles: usize,
    }

    impl WorkingRegion {
        pub fn new(((width, lenght), desired_shapes): Region, shapes: &[Shape; N_SHAPES]) -> Self {
            let mut desired_shapes = desired_shapes
                .iter()
                .enumerate()
                .map(|(si, v)| (*v, shapes[si].get_rotations_bits()));

            let desired_shapes = std::array::from_fn(|_| desired_shapes.next().unwrap());

            Self {
                mem: Vec::from_iter(std::iter::repeat_n(Row::zeros_len(width), lenght)),
                desired_shapes,
                free_tiles: width * lenght,
            }
        }

        pub fn no_more_desired(&self) -> bool {
            self.desired_shapes.iter().all(|(e, _)| *e == 0)
        }

        pub fn fit_rotation(
            &mut self,
            shape_index: usize,
            rotation: BitShape,
            place: (usize, usize),
        ) -> Result<(), &'static str> {
            let (dx, dy) = place;
            // check if the rotation will work
            for x in dx..SHAPE_DIM + dx {
                let placed_shape_row = (rotation.mem[x - dx] as usize) << dy;
                if (self.mem[x].mem & placed_shape_row) != 0 {
                    return Err("Collision detected");
                }
            }

            // set the shape
            for x in dx..SHAPE_DIM + dx {
                self.mem[x].mem |= (rotation.mem[x - dx] as usize) << dy;
            }

            self.desired_shapes[shape_index].0 -= 1;
            self.free_tiles -= rotation.area;
            Ok(())
        }

        pub fn unfit_rotation(
            &mut self,
            shape_index: usize,
            rotation: BitShape,
            place: (usize, usize),
        ) -> Result<(), &'static str> {
            let (dx, dy) = place;

            // remove the shape
            for x in dx..SHAPE_DIM + dx {
                self.mem[x].mem ^= (rotation.mem[x - dx] as usize) << dy
            }

            self.desired_shapes[shape_index].0 += 1;
            self.free_tiles += rotation.area;

            Ok(())
        }

        pub fn can_fit_shapes(mut self) -> bool {
            fn dfs(state: &mut WorkingRegion) -> bool {
                if state.no_more_desired() {
                    return true;
                }

                let desired_shape = state
                    .desired_shapes
                    .into_iter()
                    .enumerate()
                    .filter(|(_, (s, _))| *s > 0)
                    .map(|(i, (_, s))| (i, s))
                    .next();

                if let Some((sh_i, rotations)) = desired_shape {
                    for rot in rotations {
                        for x in 0..state.mem.len() - (SHAPE_DIM - 1) {
                            for y in 0..state.mem[0].len - (SHAPE_DIM - 1) {
                                let place = (x, y);
                                if state.fit_rotation(sh_i, rot, place).is_err() {
                                    continue;
                                }
                                if dfs(state) {
                                    return true;
                                } else {
                                    state.unfit_rotation(sh_i, rot, place).unwrap();
                                }
                            }
                        }
                    }
                }

                false
            }

            // this check is what makes the problem solvable
            // makes no sense tho I have to do this, the problem is too complex
            if self.free_tiles
                < self
                    .desired_shapes
                    .iter()
                    .map(|(m, [s, ..])| s.area * m)
                    .sum()
            {
                return false;
            }

            dfs(&mut self)
        }
    }

    pub fn solve(input: &str) -> usize {
        use rayon::prelude::*;
        let (shapes, regions) = parse_input(input);
        regions
            .into_par_iter()
            .map(|r| WorkingRegion::new(r, &shapes))
            .map(|wr| wr.can_fit_shapes())
            .filter(|wr| *wr)
            .count()
    }

    #[cfg(test)]
    mod test {
        use super::super::*;
        use super::*;

        #[test]
        fn test_parse_input() {
            let (shapes, regions) = parse_input(EXAMPLE);
            assert_eq!(shapes.len(), N_SHAPES);
            assert_eq!(regions.len(), 3);
        }

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 2);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day12/input.txt").unwrap()),
                531
            );
        }
    }
}
