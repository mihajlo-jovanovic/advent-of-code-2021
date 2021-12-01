#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse::<u16>().expect("not a number?")).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u16]) -> usize {
    input.windows(2).fold(0, |acc, pair| if pair[0] < pair[1] { acc + 1 } else { acc })
}

#[aoc(day1, part2)]
fn part2(input: &[u16]) -> usize {
    part1(&input.windows(3).map(|x| x.iter().sum()).collect::<Vec<u16>>())
}

#[test]
fn test_part1() {
    assert_eq!(7, part1(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
}

#[test]
fn test_part2() {
    assert_eq!(5, part2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]))
}