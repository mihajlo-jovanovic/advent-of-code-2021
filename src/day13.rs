#[aoc_generator(day13)]
fn generator_input(input: &str) -> Vec<(u16, u16)> {
    let mut parts = input.split("\n\n");
    parts.next().unwrap().lines().map(|l| {
        let mut parts = l.split(',');
        (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
    }).collect()
}

#[aoc(day13, part1)]
fn part1(input: &[(u16, u16)]) -> usize {
    // just printing coords here, solution is in Clojure for today. I wanted to play in REPL ;-)
    for (x, y) in input {
        print!("'({} {}), ", x, y)
    }
    println!();
    input.len()
}

#[test]
fn test_parsing() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let points = generator_input(input);
    println!("total points: {}", points.len());
    for (x, y) in &points {
        print!("'({} {}), ", x, y)
    }
    println!();
    assert_eq!(18, points.len());
}