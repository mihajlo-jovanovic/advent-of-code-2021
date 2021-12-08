use std::collections::HashSet;

#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<(HashSet<String>,HashSet<String>)> {
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



#[test]
fn test_parsing() {
    let display = generator_input("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
    println!("{:?}", display);
    assert_eq!(1, display.len());
}