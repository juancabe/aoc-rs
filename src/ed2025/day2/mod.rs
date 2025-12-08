pub mod part1 {

    trait IsInvalidId {
        fn is_invalid_id(&self) -> bool;
    }

    impl IsInvalidId for usize {
        fn is_invalid_id(&self) -> bool {
            let s = self.to_string();
            s[..s.len() / 2] == s[s.len() / 2..]
        }
    }

    pub struct Range {
        pub start: usize,
        pub end: usize,
    }

    impl Range {
        pub fn invalid_ids_added(&self) -> usize {
            (self.start..=self.end)
                .map(|num| if num.is_invalid_id() { num } else { 0 })
                .sum()
        }
    }

    pub fn parse_input(input: &str) -> Vec<Range> {
        input
            .split(",")
            .map(|range| range.split("-"))
            .map(|mut r| [r.next().unwrap(), r.next().unwrap()])
            .map(|[low, up]| Range {
                start: low.parse().unwrap(),
                end: up.parse().unwrap(),
            })
            .collect()
    }

    pub fn calculate_occurances(p_input: Vec<Range>) -> usize {
        p_input.iter().map(|r| r.invalid_ids_added()).sum()
    }

    #[cfg(test)]
    mod test {
        use std::{fs::File, io::Read};

        use super::*;

        pub const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        #[test]
        fn test_parse_input() {
            let v = parse_input(EXAMPLE);
            assert_eq!(v.len(), 11);
        }

        #[test]
        fn test_is_invalid_id() {
            assert!(!0usize.is_invalid_id());
            assert!(!1usize.is_invalid_id());
            assert!(!2usize.is_invalid_id());
            assert!(!2usize.is_invalid_id());
            assert!(22usize.is_invalid_id());
            assert!(102102usize.is_invalid_id());
        }

        #[test]
        fn test_solution1_example() {
            let sum = calculate_occurances(parse_input(EXAMPLE));
            assert_eq!(sum, 1227775554)
        }

        #[test]
        fn test_solution1_input() {
            let mut input = String::new();
            let mut file = File::open("./src/ed2025/day2/input.txt").unwrap();
            file.read_to_string(&mut input).unwrap();
            let sum = calculate_occurances(parse_input(input.trim()));
            assert_eq!(sum, 37314786486)
        }
    }
}

pub mod part2 {

    trait IsInvalidId {
        fn is_invalid_id(&self) -> bool;
    }

    impl IsInvalidId for usize {
        fn is_invalid_id(&self) -> bool {
            self.to_string().is_invalid_id()
        }
    }

    impl IsInvalidId for str {
        // My approach was this (inefficient)... 😔
        // fn is_invalid_id(&self) -> bool {
        //     (2..=self.len())
        //         .filter(|n_parts| self.len().is_multiple_of(*n_parts))
        //         .any(|n_parts| {
        //             let p_size = self.len() / n_parts;
        //             let mut iter = (0..n_parts).map(|i| &self[i * p_size..(i + 1) * p_size]);
        //             if let Some(true) = iter.next().map(|first| iter.all(|e| e == first)) {
        //                 return true;
        //             }
        //             false
        //         })
        // }

        // So cool
        fn is_invalid_id(&self) -> bool {
            if self.len() < 2 {
                return false;
            }
            let doubled = self.repeat(2);
            doubled[1..doubled.len() - 1].contains(self)
        }
    }

    pub struct Range {
        pub start: usize,
        pub end: usize,
    }

    impl Range {
        pub fn invalid_ids_added(&self) -> usize {
            (self.start..=self.end)
                .map(|num| if num.is_invalid_id() { num } else { 0 })
                .sum()
        }
    }

    pub fn parse_input(input: &str) -> Vec<Range> {
        input
            .split(",")
            .map(|range| range.split("-"))
            .map(|mut r| [r.next().unwrap(), r.next().unwrap()])
            .map(|[low, up]| Range {
                start: low.parse().unwrap(),
                end: up.parse().unwrap(),
            })
            .collect()
    }

    pub fn calculate_occurances(p_input: Vec<Range>) -> usize {
        p_input.iter().map(|r| r.invalid_ids_added()).sum()
    }

    #[cfg(test)]
    mod test {
        use std::{fs::File, io::Read};

        use super::*;

        pub const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        #[test]
        fn test_parse_input() {
            let v = parse_input(EXAMPLE);
            assert_eq!(v.len(), 11);
        }

        #[test]
        fn test_is_invalid_id() {
            assert!(!0usize.is_invalid_id());
            assert!(!1usize.is_invalid_id());
            assert!(!2usize.is_invalid_id());
            assert!(!2usize.is_invalid_id());
            assert!(22usize.is_invalid_id());
            assert!(222usize.is_invalid_id());
            assert!(102102usize.is_invalid_id());
            assert!(102102102usize.is_invalid_id());
        }

        #[test]
        fn test_solution1_example() {
            let sum = calculate_occurances(parse_input(EXAMPLE));
            assert_eq!(sum, 4174379265)
        }

        #[test]
        fn test_solution1_input() {
            let mut input = String::new();
            let mut file = File::open("./src/ed2025/day2/input.txt").unwrap();
            file.read_to_string(&mut input).unwrap();
            let sum = calculate_occurances(parse_input(input.trim()));
            assert_eq!(sum, 47477053982)
        }
    }
}
