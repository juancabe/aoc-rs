#[allow(dead_code)]
const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2(isize, isize);

impl Vec2 {
    pub fn add(&mut self, b: &Vec2) {
        self.0 += b.0;
        self.1 += b.1;
    }

    pub fn some_neg(&self) -> bool {
        self.0 < 0 || self.1 < 0
    }
}

pub fn area(v0: &Vec2, v1: &Vec2) -> usize {
    (v0.0.abs_diff(v1.0) + 1) * (v0.1.abs_diff(v1.1) + 1)
}
pub mod part1 {

    use super::*;

    pub fn solve(input: &str) -> usize {
        let parsed = input.lines().map(|l| {
            let mut coords = l.split(",").map(|c| c.parse::<isize>().unwrap());
            Vec2(coords.next().unwrap(), coords.next().unwrap())
        });

        let (_, area) = parsed.clone().enumerate().skip(1).fold(
            ((Vec2::default(), Vec2::default()), 0usize),
            |acc, (v0i, v0)| {
                let max_d_v1 = parsed
                    .clone()
                    .enumerate()
                    .filter(|(v1i, _)| *v1i < v0i)
                    .map(|(_, v1)| (v1, area(&v0, &v1)))
                    .max_by_key(|(_, a)| *a)
                    .unwrap();
                println!();

                if acc.1 < max_d_v1.1 {
                    ((v0, max_d_v1.0), max_d_v1.1)
                } else {
                    acc
                }
            },
        );

        area
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 50);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day9/input.txt").unwrap()),
                4765757080
            );
        }
    }
}

pub mod part2 {
    use super::*;

    fn parse_input(input: &str) -> Vec<Vec2> {
        input
            .lines()
            .map(|l| {
                let mut coords = l.split(",").map(|c| c.parse::<isize>().unwrap());
                Vec2(coords.next().unwrap(), coords.next().unwrap())
            })
            .collect()
    }

    // Check if point p is on the segment defined by a and b.
    fn is_on_segment(p: &Vec2, a: &Vec2, b: &Vec2) -> bool {
        // Must be collinear and within the bounding box of the segment.
        // This is like checking if the slops of the lines p-a b-a are the same
        (b.1 - a.1) * (p.0 - a.0) == (p.1 - a.1) * (b.0 - a.0) // Collinear check (cross-product is 0)
            && p.0 >= isize::min(a.0, b.0) && p.0 <= isize::max(a.0, b.0)
            && p.1 >= isize::min(a.1, b.1) && p.1 <= isize::max(a.1, b.1)
    }

    // Ray Casting Algorithm to check if a point is inside a polygon.
    // Also checks if the point is on the boundary.
    fn is_inside_or_on_boundary(p: &Vec2, red_tiles: &[Vec2]) -> bool {
        let n = red_tiles.len();
        let mut winding_number = 0;

        // Iterate through all segments of the polygon
        for i in 0..n {
            let p1 = &red_tiles[i];
            let p2 = &red_tiles[(i + 1) % n];

            // 1. Check if the point is on the boundary segment
            if is_on_segment(p, p1, p2) {
                return true;
            }

            // 2. Ray Casting (Winding Number variant for simplicity)
            // We use the horizontal ray (y = p.1) for simplicity in the ray casting/crossing count,
            // but the problem geometry (axis-aligned segments) allows for a simple crossing count.

            // Check if the segment crosses the horizontal ray extending from p to the right
            if p1.1 <= p.1 {
                if p2.1 > p.1 {
                    // Upward crossing
                    let cross = (p2.0 - p1.0) * (p.1 - p1.1) - (p2.1 - p1.1) * (p.0 - p1.0);
                    if cross > 0 {
                        winding_number += 1;
                    }
                }
            } else {
                // p1.1 > p.1
                if p2.1 <= p.1 {
                    // Downward crossing
                    let cross = (p2.0 - p1.0) * (p.1 - p1.1) - (p2.1 - p1.1) * (p.0 - p1.0);
                    if cross < 0 {
                        winding_number -= 1;
                    }
                }
            }
            // Note: For grid-based problems with axis-aligned segments, a simpler check for
            // even/odd crossings often suffices, but the winding number is generally more robust.
        }

        // Inside if winding number is non-zero (non-zero rule)
        // For the simple ray casting rule (odd/even), inside if count is odd.
        // Given the problem context, the winding number check (non-zero == inside) is used here.
        // For simple, non-self-intersecting polygons, this simplifies to odd/even.
        winding_number != 0
    }

    fn boundary_intersects_rectangle(r1: &Vec2, r2: &Vec2, red_tiles: &[Vec2]) -> bool {
        let rx_min = isize::min(r1.0, r2.0);
        let rx_max = isize::max(r1.0, r2.0);
        let ry_min = isize::min(r1.1, r2.1);
        let ry_max = isize::max(r1.1, r2.1);

        let n = red_tiles.len();
        for i in 0..n {
            let p1 = &red_tiles[i];
            let p2 = &red_tiles[(i + 1) % n];

            // Determine segment bounds
            let seg_x_min = isize::min(p1.0, p2.0);
            let seg_x_max = isize::max(p1.0, p2.0);
            let seg_y_min = isize::min(p1.1, p2.1);
            let seg_y_max = isize::max(p1.1, p2.1);

            // Check for Vertical Segment (x is constant) intersecting the rectangle
            if p1.0 == p2.0 {
                let seg_x = p1.0;
                // A vertical wall intersects if its X is strictly between the rect's Left/Right
                // AND its Y range overlaps with the rect's Y range.
                if seg_x > rx_min && seg_x < rx_max {
                    // Check for Y overlap. We use strict inequality for intersection of *interior*
                    if isize::max(seg_y_min, ry_min) < isize::min(seg_y_max, ry_max) {
                        return true; // Intersection found!
                    }
                }
            }
            // Check for Horizontal Segment (y is constant) intersecting the rectangle
            else if p1.1 == p2.1 {
                let seg_y = p1.1;
                // A horizontal wall intersects if its Y is strictly between the rect's Top/Bottom
                // AND its X range overlaps with the rect's X range.
                if seg_y > ry_min
                    && seg_y < ry_max
                    && isize::max(seg_x_min, rx_min) < isize::min(seg_x_max, rx_max)
                {
                    return true; // Intersection found!
                }
            }
        }
        false
    }

    pub fn solve(input: &str) -> isize {
        let red_tiles = parse_input(input);
        let mut max_area = 0;

        // Iterate through all pairs of red tiles (R1, R2) as opposite corners
        for i in 0..red_tiles.len() {
            for j in 0..red_tiles.len() {
                if i == j {
                    continue;
                }

                let r1 = red_tiles[i];
                let r2 = red_tiles[j];

                let current_area = area(&r1, &r2);

                // Determine the two implicit (non-red) corners of the rectangle:
                // C1 = (R1.x, R2.y)
                // C2 = (R2.x, R1.y)
                let c1 = Vec2(r1.0, r2.1);
                let c2 = Vec2(r2.0, r1.1);

                // Check if C1 and C2 are inside or on the boundary of the red/green polygon.
                // and no other boundary intersects the rectangle
                if is_inside_or_on_boundary(&c1, &red_tiles)
                    && is_inside_or_on_boundary(&c2, &red_tiles)
                    && !boundary_intersects_rectangle(&r1, &r2, &red_tiles)
                {
                    max_area = isize::max(max_area, current_area as isize);
                }
            }
        }

        max_area
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 24);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day9/input.txt").unwrap()),
                1498673376
            );
        }
    }
}
