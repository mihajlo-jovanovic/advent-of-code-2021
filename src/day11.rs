use std::collections::HashSet;

#[aoc_generator(day11)]
fn generator_input(input: &str) -> [[u8; 10]; 10] {
    let a: Vec<Vec<u8>> = input.lines().map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as u8).collect()).collect();
    let mut b = [[0; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            b[i][j] = a[i][j];
        }
    }
    b
}

fn step(energy_levels: &mut [[u8; 10]; 10]) -> usize {
    for row in energy_levels.iter_mut() {
        for lvl in row.iter_mut() {
            *lvl += 1;
        }
    }
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..10 {
        for j in 0..10 {
            if energy_levels[i][j] > 9 {
                flashed.insert((i, j));
                increment_neighbors(energy_levels, i as isize, j as isize, &mut flashed);
                energy_levels[i][j] = 0;
            }
        }
    }
    flashed.len()
}

fn increment_neighbors(energy_levels: &mut [[u8; 10]; 10], i: isize, j: isize, flashed: &mut HashSet<(usize, usize)>) {
    for k in i - 1..i + 2 {
        for l in j - 1..j + 2 {
            if valid_indices(k, l) && !flashed.contains(&(k as usize, l as usize)) {
                energy_levels[k as usize][l as usize] += 1;
                if energy_levels[k as usize][l as usize] > 9 {
                    flashed.insert((k as usize, l as usize));
                    increment_neighbors(energy_levels, k, l, flashed);
                    energy_levels[k as usize][l as usize] = 0;
                }
            }
        }
    }
}

fn valid_indices(i: isize, j: isize) -> bool {
    (0..10).contains(&i) && (0..10).contains(&j)
}

#[aoc(day11, part1)]
fn part1(energy_levels: &[[u8; 10]; 10]) -> usize {
    let mut m = energy_levels.to_owned();
    let mut flash_cnt: usize = 0;
    for _ in 0..100 {
        flash_cnt += step(&mut m);
    }
    flash_cnt
}

#[aoc(day11, part2)]
fn part2(energy_levels: &[[u8; 10]; 10]) -> u16 {
    let mut m = energy_levels.to_owned();
    let mut flash_cnt: usize = 0;
    let mut step_cnt: u16 = 0;
    while flash_cnt != 100 {
        step_cnt += 1;
        flash_cnt = step(&mut m);
        if step_cnt == 1000 {
            println!("Reached step 1000: giving up...");
            break;
        }
    }
    step_cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_parsing() {
        let parsed = generator_input(INPUT);
        print(&parsed);
        assert_eq!(10, parsed.len());
    }

    #[test]
    fn test_step() {
        let mut parsed = generator_input(INPUT);
        assert_eq!(0, step(&mut parsed));
        assert_eq!(35, step(&mut parsed));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1656, part1(&generator_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(195, part2(&generator_input(INPUT)));
    }

    fn print(energy_levels: &[[u8; 10]; 10]) {
        for i in energy_levels.iter().take(10) {
            for j in i.iter().take(10) {
                print!("{}", j);
            }
            println!();
        }
        println!();
    }
}