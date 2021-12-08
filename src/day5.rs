use std::collections::{HashMap, HashSet};

#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<(u16, u16)> {
    input.lines().flat_map(|l| {
        let a = l.split_whitespace().collect::<Vec<&str>>();
        let b = a.get(0).unwrap().split(',').filter_map(|a| a.parse().ok()).collect::<Vec<u16>>();
        let (x1, y1) = (*b.get(0).unwrap(), *b.get(1).unwrap());
        let c = a.get(2).unwrap().split(',').filter_map(|a| a.parse().ok()).collect::<Vec<u16>>();
        let (x2, y2) = (*c.get(0).unwrap(), *c.get(1).unwrap());
        if h_v((x1, y1), (x2, y2)) {
            // println!("found horizontal lineL {:?}->{:?}", (x1, y1), (x2, y2));
            explode((x1, y1), (x2, y2))
        } else {
            vec![]
        }
    }).collect()
}

fn h_v((x1, y1): (u16, u16), (x2, y2): (u16, u16)) -> bool {
    x1 == x2 || y1 == y2 || ((x2 as i16 - x1 as i16).abs() == (y2 as i16 - y1 as i16).abs())
}

fn explode((x1, y1): (u16, u16), (x2, y2): (u16, u16)) -> Vec<(u16, u16)> {
    let mut res: Vec<(u16, u16)> = vec![];

    //check for diagonals (not proud of this, but it is late I just wanted to get it done)
    if (x2 as i16 - x1 as i16).abs() == (y2 as i16 - y1 as i16).abs() {
        if x1 > x2 {
            if y1 > y2 {
                for x in 0..=x1 - x2 {
                    res.push((x1 - x, y1 - x));
                }
            } else {
                for x in 0..=x1 - x2 {
                    res.push((x1 - x, y1 + x));
                }
            }
        } else if y1 > y2 {
            for x in 0..=x2 - x1 {
                res.push((x1 + x, y1 - x));
            }
        } else {
            for x in 0..=x2 - x1 {
                res.push((x1 + x, y1 + x));
            }
        }
        //println!("example horizontal line: {:?}", res);
    } else {
        if x1 > x2 {
            for i in x2..=x1 {
                res.push((i, y1));
            }
        } else {
            for i in x1..=x2 {
                res.push((i, y1));
            }
        }
        if y1 > y2 {
            for i in y2..=y1 {
                res.push((x1, i));
            }
        } else {
            for i in y1..=y2 {
                res.push((x1, i));
            }
        }
    }
    let b: HashSet<(u16, u16)> = res.iter().cloned().collect();
    b.iter().cloned().collect()
}

#[aoc(day5, part1)]
fn part1(input: &[(u16, u16)]) -> usize {
    //println!("{:?}", input);
    let mut m: HashMap<(u16, u16), u16> = HashMap::new();
    for i in input {
        *m.entry(*i).or_default() += 1;
    }
    m.values().filter(|&v| *v > 1).count()
}

#[test]
fn test_parse_input() {
    let a = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    assert_eq!(12, part1(&generator_input(a)));
}

#[test]
fn test_explode() {
    assert_eq!(3, explode((9, 7), (7, 7)).len());
    assert!(explode((9, 7), (7, 7)).contains(&(8, 7)));
    assert_eq!(3, explode((1, 1), (1, 3)).len());
    assert!(explode((1, 1), (1, 3)).contains(&(1, 2)));
    assert_eq!(3, explode((1, 1), (3, 3)).len());
    assert!(explode((1, 1), (3, 3)).contains(&(2, 2)));
    assert_eq!(9, explode((8, 0), (0, 8)).len());
    assert!(explode((8, 0), (0, 8)).contains(&(1, 7)));
}