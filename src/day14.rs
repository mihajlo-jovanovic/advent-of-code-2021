#[aoc_generator(day14)]
fn generator_input(input: &str) -> Vec<(String, String)> {
    let mut parts = input.split("\n\n");
    parts.nth(1).unwrap().lines().map(|l| {
        let mut p = l.split(" -> ");
        (p.next().unwrap().to_owned(), p.next().unwrap().to_owned())
    }).collect()
}

#[aoc(day14, part1)]
fn part1(input: &[(String, String)]) -> usize {
    // just parsing input here, solution for part 1 is in Clojure again...Todo: finish part2
    for (k, v) in input {
        print!("\"{}\" \"{}\",", k, v);
    }
    println!();
    1
}

#[test]
fn test_parsing() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let parsed = generator_input(input);
    // println!("{:?}", parsed.iter().next().unwrap());
    // for (k, v) in &parsed {
    //     print!("\"{}\" \"{}\",", k, v);
    // }
    // println!();
    assert_eq!(16, parsed.len());
}