pub mod part1 {
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        let mut map = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split(":");
            map.entry(parts.next().unwrap())
                .or_insert(Vec::with_capacity(5))
                .extend(parts.next().unwrap().split_whitespace());
        }

        let mut to_visit = vec!["you"];
        let mut paths = 0;

        while let Some(visiting) = to_visit.pop() {
            if let Some(new_nodes) = map.get(visiting) {
                for node in new_nodes {
                    if *node == "out" {
                        paths += 1;
                    } else {
                        // We got no infinite loops here! Nice
                        to_visit.push(node);
                    }
                }
            }
        }

        paths
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 5);
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day11/input.txt").unwrap()),
                699
            );
        }
    }
}

pub mod part2 {
    use std::collections::HashMap;

    // THANKS GEMINI FOR THIS IDEA!!!!!!!!
    type Memo<'a> = HashMap<(&'a str, &'a str), usize>;

    pub fn solve(input: &str) -> usize {
        let mut map = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split(":");
            map.entry(parts.next().unwrap())
                .or_insert(Vec::with_capacity(5))
                .extend(parts.next().unwrap().split_whitespace());
        }

        // THIS IDEA WAS MINE THO!! 😉😌
        // Everything needs to go through fft and dac, so we can precalculate the number of paths bewteen fft
        // and dac (fft-dac) and vice versa (dac-fft), also (fft-out) and (dac-out) number of paths
        // From the start we have the precalculated number of paths for fft-dac-out dac-fft-out
        //
        // fft-dac-out = fft-dac * dac-out
        // dac-fft-out = dac-fft * fft-out
        //
        //
        // Then the normal functioning goes as follows:
        //  From svr:
        //      - dfs svr-dac * dac-fft-out
        //          plus
        //      - dfs svr-fft * fft-dac-out

        let mut memo = HashMap::new();

        let svr_dac = count_paths("svr", &map, 0, "dac", &mut memo);
        let svr_fft = count_paths("svr", &map, 0, "fft", &mut memo);
        let fft_dac = count_paths("fft", &map, 0, "dac", &mut memo);
        let dac_fft = count_paths("dac", &map, 0, "fft", &mut memo);
        let dac_out = count_paths("dac", &map, 0, "out", &mut memo);
        let fft_out = count_paths("fft", &map, 0, "out", &mut memo);

        svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
    }

    fn count_paths<'a>(
        node: &'a str,
        map: &HashMap<&'a str, Vec<&'a str>>,
        depth: usize,
        dest: &'a str,
        memo: &mut Memo<'a>,
    ) -> usize {
        if let Some(&count) = memo.get(&(node, dest)) {
            return count;
        }

        if node == dest {
            println!("found out at: {depth}");
            return 1;
        }

        let mut paths_found = 0;

        if let Some(neighbors) = map.get(node) {
            for &next in neighbors {
                paths_found += count_paths(next, map, depth + 1, dest, memo);
            }
        }

        memo.insert((node, dest), paths_found);

        paths_found
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const EXAMPLE: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        #[test]
        fn test_solve() {
            assert_eq!(solve(EXAMPLE), 2);
            println!("EXAMPLE passed");
            assert_eq!(
                solve(&std::fs::read_to_string("./src/ed2025/day11/input.txt").unwrap()),
                388893655378800
            );
        }
    }
}
