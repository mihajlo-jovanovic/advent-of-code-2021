use array2d::Array2D;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    data: Array2D<u8>,
}

impl Board {
    fn new(element: &[u8]) -> Board {
        Board { data: Array2D::from_row_major(element, BOARD_SIZE, BOARD_SIZE) }
    }

    // Returns a tuple of the index when this board would be a winner in the first position and its `score` in the second position
    fn play(&self, nums: &[u8]) -> Option<(usize, u16)> {
        (5..nums.len()).find_map(|i| {
            self.data.as_rows().iter().chain(self.data.as_columns().iter()).find_map(|r| {
                if r.iter().filter(|&e| !nums[..i].contains(e)).count() == 0 {
                    let score: u16 = self.data.as_row_major().iter().filter(|&x| !nums[..i].contains(x)).cloned().map(u16::from).sum();
                    return Some((i - 1, score));
                }
                None
            })
        })
    }
}

struct Game {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
fn generator_input(input: &str) -> Game {
    let (n, b) = input.split_once("\n\n").unwrap();
    Game {
        numbers: n.split(',').map(|n| n.parse().unwrap()).collect(),
        boards: b.split("\n\n")
            .map(|l| {
                let a = l.split_whitespace();
                let b: Vec<u8> = a.map(|s| s.parse().unwrap()).collect();
                Board::new(&b)
            })
            .collect(),
    }
}

#[aoc(day4, part1)]
fn part1(input: &Game) -> u16 {
    let winner = input.boards.iter().map(|b| b.play(&input.numbers).unwrap()).min().unwrap();
    *input.numbers.get(winner.0).unwrap() as u16 * winner.1
}

#[aoc(day4, part2)]
fn part2(input: &Game) -> u16 {
    let winner = input.boards.iter().map(|b| b.play(&input.numbers).unwrap()).max().unwrap();
    *input.numbers.get(winner.0).unwrap() as u16 * winner.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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

    #[test]
    fn test_parse_input() {
        let input = generator_input(INPUT);
        assert_eq!(27, input.numbers.len());
        assert_eq!(7, input.numbers[0]);
        assert_eq!(Some(&1), input.numbers.iter().last());
        let b = input.boards.iter().last().unwrap();
        assert_eq!(3, input.boards.len());
        assert_eq!(vec![10, 16, 15, 9, 19], b.data.row_iter(1).cloned().collect::<Vec<u8>>());
        assert_eq!(vec![17, 15, 23, 13, 12], b.data.column_iter(2).cloned().collect::<Vec<u8>>());
    }

    #[test]
    fn test_play() {
        let input = generator_input(INPUT);
        let b = input.boards.iter().last().unwrap();
        assert_eq!(Some((11, 188)), b.play(&input.numbers));
        for b in input.boards {
            println!("{:?}", b.play(&input.numbers))
        }
    }

    #[test]
    fn test_part1() {
        let input = generator_input(INPUT);
        assert_eq!(4512, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = generator_input(INPUT);
        assert_eq!(1924, part2(&input));
    }
}

