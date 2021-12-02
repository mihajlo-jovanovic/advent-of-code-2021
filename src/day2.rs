use itertools::Itertools;

type Command = (i32, i32);

#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<Command> {
    input.lines().map(|l| {
        let (i, x) = l.split_whitespace().next_tuple().unwrap();
        match i {
            "forward" => (x.parse().unwrap(), 0),
            "down" => (0, x.parse().unwrap()),
            "up" => (0, -x.parse::<i32>().unwrap()),
            _ => panic!("invalid command")
        }
    }).collect()
}

#[aoc(day2, part1)]
fn part1(commands: &[Command]) -> i32 {
    let pos: (i32, i32) = commands.iter().fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("final position: {:?}", pos);
    pos.0 * pos.1
}

#[aoc(day2, part2)]
fn part2(commands: &[Command]) -> i32 {
    // second position repurposed to store aim, depth is now last position
    let pos = commands.iter().fold((0, 0, 0), |acc, x| if x.0 > 0 { (acc.0 + x.0, acc.1, acc.2 + acc.1 * x.0) } else { (acc.0, acc.1 + x.1, acc.2) });
    println!("final position: {:?}", pos);
    pos.0 * pos.2
}

#[test]
fn test_part1() {
    assert_eq!(150, part1(&generator_input("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2")));
}

#[test]
fn test_part2() {
    assert_eq!(900, part2(&generator_input("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2")));
}