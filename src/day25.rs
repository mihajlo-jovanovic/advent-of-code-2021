use array2d::Array2D;

#[aoc_generator(day25)]
fn generator_input(input: &str) -> Array2D<char> {
    let tmp: Vec<char> = input.lines().flat_map(|l| l.chars()).collect();
    Array2D::from_row_major(&*tmp,
                            input.lines().count(),
                            input.lines().next().unwrap().chars().count())
}

#[aoc(day25, part1)]
fn part1(input: &Array2D<char>) -> usize {
    let mut prev = input.clone();
    let mut new = step(prev.clone());
    let mut i = 1;
    while new.as_row_major() != prev.as_row_major() {
        prev = new;
        new = step(prev.clone());
        i += 1;
        //println!("loop count:{}", i);
    }
    i
}

fn step(map: Array2D<char>) -> Array2D<char> {
    let mut new = map.clone();
    for (r, row_iter) in map.rows_iter().enumerate() {
        for (c, el) in row_iter.enumerate() {
            match el {
                '>' => {
                    if let Some(x) = map.get(r, c + 1) {
                        if *x == '.' {
                            new[(r, c)] = '.';
                            new[(r, c + 1)] = '>';
                        }
                    } else if *map.get(r, 0).unwrap() == '.' {
                        new[(r, c)] = '.';
                        new[(r, 0)] = '>';
                    }
                }
                &_ => {}
            }
        }
    }
    let mut new2 = new.clone();
    for (r, row_iter) in new.rows_iter().enumerate() {
        for (c, el) in row_iter.enumerate() {
            //println!("row: {} col: {}  val: {}", r, c, el);
            match el {
                'v' => {
                    if let Some(x) = new.get(r + 1, c) {
                        if *x == '.' {
                            new2[(r, c)] = '.';
                            new2[(r + 1, c)] = 'v';
                        }
                    } else if *new.get(0, c).unwrap() == '.' {
                        new2[(r, c)] = '.';
                        new2[(0, c)] = 'v';
                    }
                }
                &_ => {}
            }
        }
    }
    new2
}

#[cfg(test)]
mod tests {
    use super::{generator_input, step, part1};

    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_parsing() {
        let parsed = generator_input(INPUT);
        assert_eq!(10, parsed.num_columns());
        assert_eq!(9, parsed.num_rows());
        assert_eq!(parsed[(0, 0)], 'v');
        assert_eq!(parsed[(0, 9)], '>');
        assert_eq!(parsed[(8, 0)], '.');
    }

    #[test]
    fn test_step() {
        let parsed = generator_input(INPUT);
        let new_state = step(parsed);
        assert_eq!(new_state[(1, 0)], 'v');
        assert_eq!(new_state[(1, 1)], '.');
        // for row_iter in map.rows_iter() {
        //     for el in row_iter {
        //         print!("{}", el);
        //     }
        //     println!()
        // }
        assert_eq!(58, part1(&generator_input(INPUT)));
    }
}