use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Board {
    row_cols: Vec<HashSet<u8>>
}

impl Board {
    fn check_bingo(&self, nums: &HashSet<u8>) -> bool {
        for n in &self.row_cols {
            if n.is_subset(nums) {
                return true;
            }
        }
        false
    }
    fn sum_nonmatching_numbers(&self, nums: &HashSet<u8>) -> u32 {
        let mut acc: u32 = 0;
        for i in self.row_cols.iter().take(BOARD_SIZE) {
            acc += i.iter().filter(|&x| !nums.contains(x)).cloned().map(u32::from).sum::<u32>()
        }
        acc
    }
}

const BOARD_SIZE: usize = 5;

#[aoc_generator(day4)]
fn generator_input(input: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let nums = input.lines().next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();
    let a = input.lines().skip(1).filter(|l| !l.is_empty()).map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();
    (nums, a)
}

#[aoc(day4, part1)]
fn part1(input: &(Vec<u8>, Vec<Vec<u8>>)) -> Option<u32> {
    let boards = parse_boards(&input.1);
    let mut bingo: HashSet<u8> = input.0.iter().take(BOARD_SIZE).cloned().collect();
    for i in BOARD_SIZE..input.0.len() {
        for b in &boards {
            if b.check_bingo(&bingo) {
                println!("found one: {:?}", b);
                println!("last number drawn: {:?}", input.0.get(i - 1).unwrap());
                println!("sum non matching: {:?}", b.sum_nonmatching_numbers(&bingo));
                return Some(*input.0.get(i - 1).unwrap() as u32 * b.sum_nonmatching_numbers(&bingo));
            }
        }
        bingo.insert(*input.0.get(i).unwrap());
    }
    None
}

#[aoc(day4, part2)]
fn part2(input: &(Vec<u8>, Vec<Vec<u8>>)) -> Option<u32> {
    let mut boards = parse_boards(&input.1);
    let mut bingo: HashSet<u8> = input.0.iter().take(BOARD_SIZE).cloned().collect();
    for i in BOARD_SIZE..input.0.len() {
        if boards.len() == 1 {
            let b = boards.last().unwrap();
            if b.check_bingo(&bingo) {
                println!("found one: {:?}", b);
                let last = input.0.get(i - 1).unwrap();
                println!("last number drawn: {}", last);
                println!("sum non matching: {:?}", b.sum_nonmatching_numbers(&bingo));
                return Some(*last as u32 * b.sum_nonmatching_numbers(&bingo));
            }
        }
        // filter out those boards that are already matching
        boards = boards.iter().filter(|&b| !b.check_bingo(&bingo)).cloned().collect();
        // move to the next bingo number
        bingo.insert(*input.0.get(i).unwrap());
    }
    None
}

fn parse_boards(input: &[Vec<u8>]) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    for board_idx in 0..input.len() / BOARD_SIZE {
        let mut nums: Vec<HashSet<u8>> = vec![];
        //rows
        for i in 0..BOARD_SIZE {
            let row = input.get(board_idx * BOARD_SIZE + i).unwrap();
            nums.push(row.iter().cloned().collect());
        }
        //cols
        let mut cols: Vec<u8> = vec![];
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let row = input.get(board_idx * 5 + j).unwrap();
                cols.push(*row.get(i).unwrap());
            }
        }
        nums.extend(cols.chunks(5).map(|c| c.iter().cloned().collect()).collect::<Vec<HashSet<u8>>>());
        boards.push(Board { row_cols: nums })
    }
    boards
}

#[test]
fn test_parse_input() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    let parsed = generator_input(input);
    println!("{:?}", parsed);
    assert_eq!(27, parsed.0.len());
    assert_eq!(15, parsed.1.len());
    let boards = parse_boards(&parsed.1);
    assert_eq!(3, boards.len());
    for b in boards {
        assert_eq!(10, b.row_cols.len());
        for nums in b.row_cols {
            assert_eq!(5, nums.len());
        }
    }
    assert_eq!(Some(4512), part1(&parsed));
    assert_eq!(Some(1924), part2(&parsed));
}