use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::FromIterator;

use itertools::Itertools;

#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<(String, String)> {
    input.lines().map(|l| {
        let mut parts = l.split(" | ");
        (
            parts.next().unwrap().to_owned(),
            parts.next().unwrap().to_owned(),
        )
    }).collect()
}

//#[aoc(day8, part1)] - did the rest in Clojure REPL, using frequencies
#[allow(dead_code)]
fn part1(display: &[(String, String)]) -> usize {
    println!("display: {:?}", display);
    display.len()
}

#[aoc(day8, part2)]
fn part2(display: &[(String, String)]) -> u32 {
    let displays = displays();
    let digits: Vec<u32> = display.iter().map(|s| {
        let mut m = HashMap::new();
        let found = find_mapping(&s.0, &mut m, &displays);
        if !found {
            panic!("could not find mapping")
        }
        decode(&s.1, &m, &displays)
    }).collect();
    digits.iter().sum()
}

fn decode(output: &str, m: &HashMap<char, char>, display_config: &[HashSet<char>]) -> u32 {
    let digits: Vec<u8> = output.split_whitespace().map(|s| {
        let decoded: Vec<char> = s.chars().flat_map(|c| m.get(&c)).cloned().collect();
        let y = display_config.iter().enumerate().find_map(|(i, x)| {
            if *x == HashSet::from_iter(decoded.iter().cloned()) {
                return Some(i as u8);
            }
            None
        }).expect("");
        y
    }).collect();
    let mut result: u32 = 0;
    let base: u32 = 10;
    for i in 0..4 {
        result += (digits[3 - i] as u32) * base.pow(i.try_into().unwrap());
    }
    result
}

fn displays() -> Vec<HashSet<char>> {
    let mut displays: Vec<HashSet<char>> = vec![];
    for digit in DISPLAYS {
        let x: Vec<char> = digit.chars().collect();
        let d: HashSet<char> = HashSet::from_iter(x.iter().cloned());
        displays.push(d);
    }
    displays
}

// using `backtracking` here; could be improved with Dynamic programming/memoization
fn find_mapping(entry: &str, m: &mut HashMap<char, char>, displays: &[HashSet<char>]) -> bool {
    if m.len() == 7 {
        return true;
    }
    for i in 'a'..'h' {
        if m.contains_key(&i) {
            continue;
        }
        for j in 'a'..'h' {
            if m.values().contains(&j) {
                continue;
            }
            m.insert(i, j);
            if is_valid(entry, m, displays) && find_mapping(entry, m, displays) {
                return true;
            }
            m.remove(&i);
        }
    }
    false
}

fn is_valid(entry: &str, map: &HashMap<char, char>, displays_config: &[HashSet<char>]) -> bool {
    for i in entry.split_whitespace() {
        let decoded: Vec<char> = i.chars().flat_map(|c| map.get(&c)).cloned().collect();
        if !displays_config
            .iter()
            .filter(|a| a.len() == i.len())
            .cloned()
            .any(|s| s.is_superset(&decoded.iter().cloned().collect())) {
            return false;
        }
    }
    true
}

const DISPLAYS: [&str; 10] = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_SMALL: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const TEST_INPUT_LARGE: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_parsing() {
        let display = generator_input(TEST_INPUT_SMALL);
        assert_eq!(1, display.len());
        let display = generator_input(TEST_INPUT_LARGE);
        assert_eq!(10, display.len());
    }

    #[test]
    fn test_part2() {
        assert_eq!(61229, part2(&generator_input(TEST_INPUT_LARGE)));
    }

    #[test]
    fn test_find_mapping() {
        let mut m = HashMap::new();
        let displays = displays();
        assert!(find_mapping("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef", &mut m, &displays));
        assert_eq!(7, m.len());
    }

    #[test]
    fn test_is_valid() {
        let mut m = HashMap::new();
        let displays = displays();
        for c in 'b'..'h' {
            assert(&mut m, &displays, c)
        }
        m.clear();
        for c in 'a'..'h' {
            if c == 'b' {
                continue;
            }
            assert(&mut m, &displays, c)
        }
    }

    fn assert(m: &mut HashMap<char, char>, displays: &Vec<HashSet<char>>, c: char) {
        m.insert('a', c);
        if c == 'c' || c == 'f' {
            assert!(is_valid("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", &m, &displays));
        } else {
            assert!(!is_valid("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", &m, &displays));
        }
    }

    #[test]
    fn test_decode() {
        let mut mapping: HashMap<char, char> = HashMap::new();
        mapping.insert('d', 'a');
        mapping.insert('e', 'b');
        mapping.insert('a', 'c');
        mapping.insert('f', 'd');
        mapping.insert('g', 'e');
        mapping.insert('b', 'f');
        mapping.insert('c', 'g');
        let display_config = displays();
        assert_eq!(5353, decode("cdfeb fcadb cdfeb cdbaf", &mapping, &display_config));
    }
}