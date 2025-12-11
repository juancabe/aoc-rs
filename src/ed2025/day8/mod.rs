#[allow(dead_code)]
const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

pub mod part1 {
    use std::collections::HashMap;

    #[allow(unused_imports)]
    use super::*;

    type Vec3 = [usize; 3];

    fn euc_distance(v0: &Vec3, v1: &Vec3) -> f64 {
        // 1. Calcular la diferencia en cada dimensión
        let dx = v1[0] as f64 - v0[0] as f64;
        let dy = v1[1] as f64 - v0[1] as f64;
        let dz = v1[2] as f64 - v0[2] as f64;

        // 2. Calcular el cuadrado de la diferencia y sumarlos (Distancia al cuadrado)
        let dist_sq = dx * dx + dy * dy + dz * dz;

        // 3. Aplicar la raíz cuadrada
        (dist_sq).sqrt()
    }

    fn get_parent(idx: usize, map: &HashMap<usize, usize>) -> usize {
        let mut index = idx;

        loop {
            let new = map.get(&index).unwrap();
            if *new == index {
                return index;
            }
            index = *new;
        }
    }

    pub fn solve(input: &str, max_iters: usize) -> usize {
        let junctions: Vec<[usize; 3]> = input
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap()
            })
            .collect();

        // Vec of (v0_idx, v1_idx, distance)
        let mut distances: Vec<(usize, usize, f64)> = junctions
            .iter()
            .enumerate()
            .flat_map(|(j0_idx, j0)| {
                junctions
                    .iter()
                    .enumerate()
                    .filter(move |(i, _)| *i < j0_idx)
                    .map(move |(j1_idx, j1)| (j0_idx, j1_idx, euc_distance(j0, j1)))
            })
            .map(|(a, b, d)| (a.max(b), a.min(b), d))
            .collect();

        distances.sort_by_key(|(a, b, _)| (*a, *b));
        distances.dedup();

        distances.sort_by(|(_, _, dist_a), (_, _, dist_b)| dist_a.partial_cmp(dist_b).unwrap());

        // key: idx of a junction
        // value: idx of the junction that represents the circuit (defaults to itself)
        let mut map: HashMap<usize, usize> =
            HashMap::from_iter(junctions.iter().enumerate().map(|(idx, _)| (idx, idx)));

        let mut distances = distances.iter();

        for _ in 0..max_iters {
            if let Some((v0_idx, v1_idx, _)) = distances.next() {
                let v0_p = get_parent(*v0_idx, &map);
                let v1_p = get_parent(*v1_idx, &map);

                // They are from different circuit
                if v0_p != v1_p {
                    map.insert(v0_p, v1_p).unwrap();
                }
            }
        }

        let mut circuits = Vec::from_iter(std::iter::repeat_n(0usize, junctions.len()));

        for k in map.keys() {
            circuits[get_parent(*k, &map)] += 1;
        }

        circuits.sort();

        circuits
            .into_iter()
            .rev()
            .take(3)
            .map(|e| dbg!(e))
            .reduce(|acc, b| acc * b)
            .unwrap()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE, 10), 40);
            assert_eq!(
                solve(
                    &std::fs::read_to_string("./src/ed2025/day8/input.txt").unwrap(),
                    1000
                ),
                29406
            );
        }
    }
}

pub mod part2 {
    use std::collections::HashMap;

    #[allow(unused_imports)]
    use super::*;

    type Vec3 = [usize; 3];

    fn euc_distance(v0: &Vec3, v1: &Vec3) -> f64 {
        // 1. Calcular la diferencia en cada dimensión
        let dx = v1[0] as f64 - v0[0] as f64;
        let dy = v1[1] as f64 - v0[1] as f64;
        let dz = v1[2] as f64 - v0[2] as f64;

        // 2. Calcular el cuadrado de la diferencia y sumarlos (Distancia al cuadrado)
        let dist_sq = dx * dx + dy * dy + dz * dz;

        // 3. Aplicar la raíz cuadrada
        (dist_sq).sqrt()
    }

    fn get_parent(idx: usize, map: &HashMap<usize, usize>) -> usize {
        let mut index = idx;

        loop {
            let new = map.get(&index).unwrap();
            if *new == index {
                return index;
            }
            index = *new;
        }
    }

    pub fn solve(input: &str) -> usize {
        let junctions: Vec<[usize; 3]> = input
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap()
            })
            .collect();

        // Vec of (v0_idx, v1_idx, distance)
        let mut distances: Vec<(usize, usize, f64)> = junctions
            .iter()
            .enumerate()
            .flat_map(|(j0_idx, j0)| {
                junctions
                    .iter()
                    .enumerate()
                    .filter(move |(i, _)| *i < j0_idx)
                    .map(move |(j1_idx, j1)| (j0_idx, j1_idx, euc_distance(j0, j1)))
            })
            .map(|(a, b, d)| (a.max(b), a.min(b), d))
            .collect();

        distances.sort_by_key(|(a, b, _)| (*a, *b));
        distances.dedup();

        distances.sort_by(|(_, _, dist_a), (_, _, dist_b)| dist_a.partial_cmp(dist_b).unwrap());

        // key: idx of a junction
        // value: idx of the junction that represents the circuit (defaults to itself)
        let mut map: HashMap<usize, usize> =
            HashMap::from_iter(junctions.iter().enumerate().map(|(idx, _)| (idx, idx)));

        let mut distances = distances.iter();

        let mut count = junctions.len();

        loop {
            if let Some((v0_idx, v1_idx, _)) = distances.next() {
                let v0_p = get_parent(*v0_idx, &map);
                let v1_p = get_parent(*v1_idx, &map);

                // They are from different circuit
                if v0_p != v1_p {
                    map.insert(v0_p, v1_p).unwrap();
                    count -= 1;
                    if count == 1 {
                        return junctions[*v0_idx][0] * junctions[*v1_idx][0];
                    }
                }
            } else {
                panic!("Run out of distances")
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 25272);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day8/input.txt").unwrap()),
                7499461416
            );
        }
    }
}
