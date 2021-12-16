use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use array2d::Array2D;

#[aoc_generator(day15)]
fn generator_input(input: &str) -> Array2D<usize> {
    let num_rows = input.lines().count();
    let x: Vec<usize> = input
        .lines()
        .fold("".to_owned(), |mut acc, l| {
            acc.push_str(l);
            acc
        }).chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    Array2D::from_row_major(x.as_slice(), num_rows, x.len() / num_rows)
}

#[aoc(day15, part1)]
fn part1(input: &Array2D<usize>) -> usize {
    lowest_risk(input).unwrap()
}

#[aoc(day15, part2)]
fn part2(input: &Array2D<usize>) -> usize {
    lowest_risk(&blowup(input)).unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// `Borrowed` from https://doc.rust-lang.org/std/collections/binary_heap/index.html
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//doing classic dijkstra shortest-path alg here...formal CS education paying off
fn lowest_risk(matrix: &Array2D<usize>) -> Option<usize> {
    let start = (0, 0);
    let end = (matrix.num_rows() - 1, matrix.num_columns() - 1);
    let mut dist = HashMap::new();
    for r in 0..matrix.num_rows() {
        for c in 0..matrix.num_columns() {
            dist.insert((r, c), usize::MAX);
        }
    }
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == end { return Some(cost); }

        // Important as we may have already found a better way
        if cost > *dist.get(&position).unwrap() { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in neighbors(matrix, &position) {
            let next = State { cost: cost + matrix[edge], position: edge };

            // If so, add it to the frontier and continue
            if next.cost < *dist.get(&next.position).unwrap() {
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
                heap.push(next);
            }
        }
    }

    // Goal not reachable
    None
}

fn neighbors(matrix: &Array2D<usize>, current: &(usize, usize)) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .flat_map(|(x, y)| {
            let (x, y) = (current.0 as i32 + x, current.1 as i32 + y);
            if x < 0 || y < 0 || x > matrix.num_columns() as i32 - 1 || y > matrix.num_rows() as i32 - 1 {
                None
            } else {
                Some((x as usize, y as usize))
            }
        }).collect()
}

fn blowup(cave: &Array2D<usize>) -> Array2D<usize> {
    let mut new: Vec<usize> = Vec::with_capacity(cave.num_rows() * cave.num_columns() * 5 * 5);
    for i in 0..new.capacity() {
        let row = i / (cave.num_rows() * 5);//2  10(510)
        let col = i % (cave.num_columns() * 5);//10   10(510)
        let orig_row = row % cave.num_rows();//2   //0
        let orig_col = col % cave.num_columns();//0      //0
        let offset_x = row / cave.num_rows();
        let offset_y = col / cave.num_columns();
        let mut new_val = cave[(orig_row, orig_col)] + offset_x + offset_y;
        if new_val > 9 {
            new_val -= 9;
        }
        new.push(new_val);
    }
    Array2D::from_row_major(new.as_slice(), cave.num_rows() * 5, cave.num_columns() * 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_parsing() {
        let parsed = generator_input(INPUT);
        //println!("{:?}", parsed);
        assert_eq!(10, parsed.num_rows());
        assert_eq!(10, parsed.num_columns());
        assert_eq!(100, parsed.num_elements());
    }

    #[test]
    fn test_neighbors() {
        let parsed = generator_input(INPUT);
        assert_eq!(2, neighbors(&parsed, &(0, 0)).len());
        assert!(neighbors(&parsed, &(0, 0)).contains(&(0, 1)));
        assert_eq!(4, neighbors(&parsed, &(1, 2)).len());
        assert_eq!(2, neighbors(&parsed, &(9, 9)).len());
        assert!(neighbors(&parsed, &(9, 9)).contains(&(8, 9)));
    }

    #[test]
    fn test_shortest_path() {
        let parsed = generator_input(INPUT);
        assert_eq!(Some(40), lowest_risk(&parsed));
        assert_eq!(40, part1(&parsed));
        // make sure we count the `end` cost and not the `start`
        let parsed2 = generator_input("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944582");
        assert_eq!(Some(41), lowest_risk(&parsed2));
    }

    #[test]
    fn test_blowup() {
        let bigger = blowup(&generator_input(INPUT));
        assert_eq!(50, bigger.num_rows());
        assert_eq!(50, bigger.num_columns());
        //println!("{:?}", bigger.rows_iter().next().unwrap().cloned().collect::<Vec<usize>>());
        assert_eq!(2, bigger[(0, 10)]);
        assert_eq!(6, bigger[(0, 49)]);
        //println!("{:?}", bigger.rows_iter().nth(10).unwrap().cloned().collect::<Vec<usize>>());
        assert_eq!(3, bigger[(10, 47)]);
        assert_eq!(7, bigger[(10, 49)]);
        assert_eq!(9, bigger[(49, 49)]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(315, part2(&generator_input(INPUT)));
    }
}

