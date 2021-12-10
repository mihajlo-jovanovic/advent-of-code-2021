use std::collections::{BTreeMap, HashMap, HashSet};
use std::iter::FromIterator;
use itertools::Itertools;

#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    //input.lines().flat_map(| l | l.split(" | ").skip(1).next().unwrap().split_whitespace().map(| s | s.len())).collect()
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" | ");
            let signals = parts.next().unwrap();
            let digits = parts.next().unwrap();
            (
                signals.split_whitespace().map(|s| s.to_owned()).collect(),
                digits.split_whitespace().map(|s| s.to_owned()).collect(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
fn p1(display: &[(HashSet<String>, HashSet<String>)]) -> usize {
    println!("display: {:?}", display);
    display.len()
}

fn find_mapping(entry: &str, displays: &[HashSet<char>]) -> HashMap<char, char> {
    let mut result = HashMap::new();
    for i in 'a'..'h' {
        for j in 'a'..'h' {
            if i == j {
                continue;
            }
            result.insert(i, j);
        }
    }
    result
}

fn is_valid(entry: &str, map: &HashMap<char, char>, displays_config: &Vec<HashSet<char>>) -> bool {
    for i in entry.split_whitespace() {
        println!("Checking string {}...", i);
        let r: Vec<char> = i.chars()
            .flat_map(|c| map.get(&c)).cloned().collect();
        println!("after mapping: {:?}", r);
        let r2 = displays_config.iter().filter(|a | a.len() == i.len());
        //println!("need to be in ane of these {:?}", r2);
        //let r2 = r.all(|c| displays_config.iter().filter(|a| a.len() == i.len()).any(|b| b.contains(c)));
        if !r2.into_iter().any(|s | s.is_superset(&r.iter().cloned().collect())) {
            println!("found one that does not work!");
            return false;
        }
    }
    true
}

const DISPLAYS: [&'static str; 10] = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];

#[test]
fn test_parsing() {
    let display = generator_input("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
    println!("{:?}", display);
    assert_eq!(1, display.len());
}

#[test]
fn test_find_mapping() {
    let displays: [HashSet<char>; 1] = [HashSet::from(['a', 'b', 'c', 'e', 'f', 'g'])];
    let res = find_mapping("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", &displays);
    println!("{:?}", res);
    assert_eq!(7, res.len());
}

#[test]
fn test_is_valid() {
    let mut m = HashMap::new();
    let mut displays: Vec<HashSet<char>> = vec![];
    for digit in DISPLAYS {
        let x: Vec<char> = digit.chars().collect();
        let d: HashSet<char> = HashSet::from_iter(x.iter().cloned());
        //d.insert(digit.chars());
        displays.push(d);
    }
    // println!("displays configured: {:?}", displays);
    // for c in 'b'..'h' {
    //     println!("checking for char {}", c);
    //     m.insert('a', c);
    //     if c == 'c' || c == 'f'  {
    //         assert!(is_valid("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", &m, &displays));
    //     } else {
    //         assert!(!is_valid("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", &m, &displays));
    //     }
    // }
    let mut displays: Vec<HashSet<char>> = vec![];
    for c in 'a'..'h' {
        if c == 'b' {
            continue
        }
        println!("checking for char {}", c);
        m.insert('b', c);
        if c == 'f'  {
            assert!(is_valid("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", &m, &displays));
        } else {
            assert!(!is_valid("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab", &m, &displays));
        }
    }
}