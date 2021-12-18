use std::collections::HashMap;

#[aoc_generator(day14)]
fn generator_input(input: &str) -> (String, HashMap<String, char>) {
    let mut parts = input.split("\n\n");
    let input = parts.next().unwrap();
    let rules = parts.next().unwrap().lines().map(|l| {
        let mut p = l.split(" -> ");
        (p.next().unwrap().to_owned(), p.next().unwrap().chars().next().unwrap())
    }).collect();
    (input.to_owned(), rules)
}

#[aoc(day14, part1)]
fn part1(input: &(String, HashMap<String, char>)) -> u64 {
    let mut state = split_into_pairs(&input.0);
    for _ in 0..10 {
        state = step(state, &input.1);
    }
    //got lazy here, since I already new (from REPL) which ones to count
    freq('S', &state) - freq('F', &state)
}

fn freq(c: char, state: &HashMap<String, u64>) -> u64 {
    state.iter().filter_map(|(k, v)| if k.chars().nth(1).unwrap() == c {Some(*v)} else {None}).sum()
}

fn split_into_pairs(input: &str) -> HashMap<String, u64> {
    let mut state = HashMap::new();
    for i in 1..input.len() {
        state.insert(input[i - 1..=i].to_owned(), 1);
    }
    state
}

#[aoc(day14, part2)]
fn part2(input: &(String, HashMap<String, char>)) -> u64 {
    let mut state = split_into_pairs(&input.0);
    for _ in 0..40 {
        state = step(state, &input.1);
    }
    freq('S', &state) - freq('F', &state)
}

fn step(pairs: HashMap<String, u64>, rules: &HashMap<String, char>) -> HashMap<String, u64> {
    let mut after = HashMap::new();
    for (p, cnt) in pairs {
        let between = rules.get(&p).unwrap();
        let mut f = p[..1].to_owned();
        f.push(*between);
        let mut s = between.to_string();
        s.push_str(&p[1..]);
        *after.entry(f).or_insert(0) += cnt;
        *after.entry(s).or_insert(0) += cnt;
    }
    after
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

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

    #[test]
    fn test_parsing() {
        let (input, rules) = generator_input(INPUT);
        assert_eq!(16, rules.len());
        assert_eq!("NNCB", input);
    }

    #[test]
    fn test_step() {
        let (input, rules) = generator_input(INPUT);
        let mut state = split_into_pairs(&input);
        for _ in 0..40 {
            state = step(state, &rules);
        }
        assert_eq!(3849876073, freq('H', &state));
        assert_eq!(2192039569602, freq('B', &state));
    }
}