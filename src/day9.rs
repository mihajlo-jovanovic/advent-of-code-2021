use std::collections::{BinaryHeap, HashSet};

use array2d::Array2D;

#[aoc_generator(day9)]
fn generator_input(input: &str) -> Array2D<u8> {
    let num_rows = input.lines().count();
    let x: Vec<u8> = input
        .lines()
        .fold("".to_owned(), |mut acc, l| {
            acc.push_str(l);
            acc
        }).chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    Array2D::from_row_major(x.as_slice(), num_rows, x.len() / num_rows)
}

#[aoc(day9, part1)]
fn part1(input: &Array2D<u8>) -> u16 {
    let x: Vec<u8> = input
        .rows_iter()
        .enumerate()
        .flat_map(|(r, rows)| {
            rows
                .enumerate()
                .flat_map(move |(c, el)| {
                    // is it a low point?
                    //println!("row: {} col: {}", r, c);
                    let cnt = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .iter()
                        .flat_map(|(x, y)| {
                            let (x1, x2) = (r as i32 + x, c as i32 + y);
                            if x1 >= 0 && x2 >= 0 {
                                input.get(x1 as usize, x2 as usize)
                            } else {
                                None
                            }
                        }).filter(|&n| n <= el).count();
                    if cnt == 0 {
                        Some(el)
                    } else {
                        None
                    }
                })
        }).cloned().collect();
    println!("total low points is {}", x.len());
    x.iter().fold(0, |acc, el| acc + *el as u16 + 1)
}

fn basin_size(input: &Array2D<u8>, low_point: (u8, u8), neighbors: &mut HashSet<(u8, u8)>) {
    let adjacent: Vec<(u8, u8)> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .flat_map(|(x, y)| {
            let (x1, x2) = (low_point.0 as i32 + x, low_point.1 as i32 + y);
            // println!("{}-{}", x1, x2);
            if x1 >= 0 && x2 >= 0 && x1 < input.num_rows() as i32 && x2 < input.num_columns() as i32 && *input.get(x1 as usize, x2 as usize).unwrap() != 9 && !neighbors.contains(&(x1 as u8, x2 as u8)) {
                Some((x1 as u8, x2 as u8))
            } else {
                None
            }
        }).collect();
    // println!("neighbors: {:?}", n);
    if !adjacent.is_empty() {
        for x in adjacent {
            neighbors.insert(x);
            basin_size(input, x, neighbors);
        }
    }
}

#[aoc(day9, part2)]
fn part2(input: &Array2D<u8>) -> usize {
    let x: Vec<(u8, u8)> = input
        .rows_iter()
        .enumerate()
        .flat_map(|(r, rows)| {
            rows
                .enumerate()
                .flat_map(move |(c, el)| {
                    // is it a low point?
                    //println!("row: {} col: {}", r, c);
                    let cnt = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .iter()
                        .flat_map(|(x, y)| {
                            let (x1, x2) = (r as i32 + x, c as i32 + y);
                            if x1 >= 0 && x2 >= 0 {
                                input.get(x1 as usize, x2 as usize)
                            } else {
                                None
                            }
                        }).filter(|&n| n <= el).count();
                    if cnt == 0 {
                        Some((r as u8, c as u8))
                    } else {
                        None
                    }
                })
        }).collect();
    // println!("low points: {:?}", x);
    //println!("total low points: {}", x.len());
    let mut sizes: BinaryHeap<usize> = BinaryHeap::new();
    for i in x {
        let mut n: HashSet<(u8, u8)> = HashSet::new();
        basin_size(input, i, &mut n);
        sizes.push(n.len());
    }
    //println!("sizes: {:?}", sizes);
    sizes.iter().take(3).fold(1, |acc: usize, p| acc * *p)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn parse() {
        let a = generator_input(TEST_INPUT);
        println!("{:?}", a);
        assert_eq!(10, a.row_len());
    }

    #[test]
    fn test_part1() {
        assert_eq!(15, part1(&generator_input(TEST_INPUT)));
    }

    #[test]
    fn test_basin_size() {
        let a = generator_input(TEST_INPUT);
        let mut neighbors: HashSet<(u8, u8)> = HashSet::new();
        basin_size(&a, (0, 1), &mut neighbors);
        assert_eq!(3, neighbors.len());

        let mut n: HashSet<(u8, u8)> = HashSet::new();
        basin_size(&a, (0, 9), &mut n);
        assert_eq!(9, n.len());

        let mut n: HashSet<(u8, u8)> = HashSet::new();
        basin_size(&a, (2, 2), &mut n);
        assert_eq!(14, n.len());

        let mut n: HashSet<(u8, u8)> = HashSet::new();
        basin_size(&a, (4, 6), &mut n);
        assert_eq!(9, n.len());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1134, part2(&generator_input(TEST_INPUT)));
    }
}