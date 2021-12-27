use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use array2d::Array2D;

const MAX_COLS: usize = 11;
static FINAL_STATE: &[char] = &['#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#'];

#[aoc_generator(day23)]
fn generator_input(input: &str) -> Array2D<char> {
    let mut rows: Vec<Vec<char>> = input.lines().map(|l| l.chars().map(|c| if c == ' ' { '#' } else { c }).collect()).collect();
    //strip edges, don't need them
    rows.remove(0);
    rows.remove(rows.len() - 1);
    for r in &mut rows {
        r.remove(0);
        if r.len() == MAX_COLS + 1 {
            r.remove(r.len() - 1);
        } else {
            r.push('#');
        }
    }
    Array2D::from_rows(&rows)
}

#[aoc(day23, part1)]
fn part1(starting_config: &Array2D<char>) -> usize {
    let final_pos = vec![vec!['.'; 11], Vec::from(FINAL_STATE), Vec::from(FINAL_STATE)];
    lowest_risk(starting_config.clone(), Array2D::from_rows(&final_pos)).unwrap()
}

#[aoc(day23, part2)]
fn part2(starting_config: &Array2D<char>) -> usize {
    let mut rows = starting_config.as_rows();
    rows.insert(2, vec!['#', '#', 'D', '#', 'B', '#', 'A', '#', 'C', '#', '#']);
    rows.insert(2, vec!['#', '#', 'D', '#', 'C', '#', 'B', '#', 'A', '#', '#']);
    let unfolded_start = Array2D::from_rows(&*rows);
    let final_pos = vec![vec!['.'; 11], Vec::from(FINAL_STATE), Vec::from(FINAL_STATE), Vec::from(FINAL_STATE), Vec::from(FINAL_STATE)];
    lowest_risk(unfolded_start, Array2D::from_rows(&final_pos)).unwrap()
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize, // hash of node_state
}

struct Edge {
    node_id: u64,
    // hash of node_state
    node_state: Array2D<char>,
    cost: usize,
}

fn all_valid_states(current: &Array2D<char>) -> Vec<Edge> {
    let mut edges = vec![];
    // Check hallway to see if we can move any of the creatures to their final room
    for (r, row_iter) in current.rows_iter().enumerate() {
        if r == 0 {
            for (i, el) in row_iter.enumerate() {
                if *el == '.' {
                    continue;
                }
                if can_go_to_final_room(current, el, (r as u8, i as u8)) {
                    move_to_room(current, &mut edges, el, (r as u8, i as u8));
                }
            }
            continue;
        }
        // check rooms (r is either 1 (for top) or 2 (for bottom)
        'checking_rooms: for (i, el) in row_iter.enumerate() {
            if *el == '#' || *el == '.' {
                continue;
            }
            for j in 1..r {
                if current[(j, i)] != '.' {
                    //cannot go as there is amphipod on top
                    continue 'checking_rooms;
                }
            }
            // 1. Check if we can go straight to the final room (from a room)
            if can_go_to_final_room(current, el, (r as u8, i as u8)) {
                move_to_room(current, &mut edges, el, (r as u8, i as u8));
            } else { // 2. Second, check whether we can move to a hallway
                let valid_hallway_positions = vec![0, 1, 3, 5, 7, 9, 10];
                for hallway_pos in valid_hallway_positions {
                    if current[(0, hallway_pos)] == '.' && nothing_in_the_way(&current.as_rows()[0], i, hallway_pos) {
                        move_amphipod(current, &mut edges, el, (r as u8, i as u8), (0, hallway_pos as u8));
                    }
                }
            }
        }
    }
    edges
}

fn move_to_room(current: &Array2D<char>, mut edges: &mut Vec<Edge>, kind: &char, pos: (u8, u8)) {
    let final_room = final_room(kind);
    let mut dest = (1, final_room);
    for r in (2..current.num_rows()).rev() {
        if current[(r, final_room as usize)] == '.' {
            dest = (r as u8, final_room);
        }
    }
    move_amphipod(current, &mut edges, kind, pos, dest);
}

fn move_amphipod(state: &Array2D<char>, edges: &mut Vec<Edge>, kind: &char, start: (u8, u8), end: (u8, u8)) {
    let cost = euclidean_dist(*kind, start, end);
    let mut new_state = state.as_row_major();
    let start_idx: usize = start.0 as usize * state.num_columns() + start.1 as usize;
    let end_idx: usize = end.0 as usize * state.num_columns() + end.1 as usize;
    new_state.swap(start_idx, end_idx);
    let n: Array2D<char> = Array2D::from_row_major(new_state.as_slice(), state.num_rows(), state.num_columns());
    //print(&n);
    let mut hash = DefaultHasher::new();
    n.as_rows().hash(&mut hash);
    edges.push(Edge { node_id: hash.finish(), node_state: n, cost });
}

fn can_go_to_final_room(state: &Array2D<char>, kind: &char, pos: (u8, u8)) -> bool {
    let final_room = final_room(kind);
    if final_room == pos.1 {
        //already there
        return false;
    }
    //check final room
    nothing_in_the_way(&state.as_rows()[0], pos.1 as usize, final_room as usize) &&
        (empty(final_room, state) || (state[(1, final_room as usize)] == '.' && state.column_iter(final_room as usize).all(|c| *c == *kind || *c == '.')))
}

fn final_room(kind: &char) -> u8 {
    match kind {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => { panic!("invalid type {}", kind) }
    }
}

fn empty(room: u8, state: &Array2D<char>) -> bool {
    state.column_iter(room as usize).all(|c| *c == '.')
}

fn euclidean_dist(kind: char, x: (u8, u8), y: (u8, u8)) -> usize {
    if x.0 > 0 && y.0 > 0 {
        return euclidean_dist(kind, x, (0, x.1)) + euclidean_dist(kind, (0, x.1), y);
    }
    let dist_x: i8 = x.0 as i8 - y.0 as i8;
    let dist_y = x.1 as i8 - y.1 as i8;
    let base: usize = 10;
    (dist_x.abs() + dist_y.abs()) as usize * base.pow((final_room(&kind) / 2 - 1) as u32)
}

fn nothing_in_the_way(hallway: &[char], start: usize, end: usize) -> bool {
    if start < end {
        hallway[start + 1..=end].iter().filter(|&c| *c != '.').count() == 0
    } else {
        hallway[end..start].iter().filter(|&c| *c != '.').count() == 0
    }
}

#[allow(dead_code)]
fn print(state: &Array2D<char>) {
    println!("#############");
    print!("#");
    for c in &state.as_rows()[0] {
        print!("{}", c);
    }
    println!("#");
    //rooms
    print!("#");
    for c in &state.as_rows()[1] {
        print!("{}", c);
    }
    println!("#");
    print!("#");
    for c in &state.as_rows()[2] {
        print!("{}", c);
    }
    println!("#");
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn lowest_risk(start_state: Array2D<char>, end_state: Array2D<char>) -> Option<usize> {
    let mut dist: HashMap<usize, (usize, Array2D<char>)> = HashMap::new();

    let mut hash = DefaultHasher::new();
    start_state.as_rows().hash(&mut hash);
    let start_state_id: usize = hash.finish() as usize;
    dist.insert(start_state_id, (0, start_state));

    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, position: start_state_id });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        let current_state = &dist.get(&position).unwrap().1;
        let current_cost = dist.get(&position).unwrap().0;

        if final_position(current_state, &end_state) { return Some(cost); }

        // Important as we may have already found a better way
        if cost > current_cost { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in all_valid_states(current_state) {
            let next = State { cost: cost + edge.cost, position: edge.node_id as usize };

            if let Some(x) = dist.get(&(edge.node_id as usize)) {
                if next.cost < x.0 {
                    // Relaxation, we have now found a better way
                    dist.insert(next.position, (next.cost, edge.node_state));
                    heap.push(next);
                }
            } else {
                dist.insert(next.position, (next.cost, edge.node_state));
                heap.push(next);
            }
        }
    }

    // Goal not reachable
    None
}

fn final_position(state: &Array2D<char>, end: &Array2D<char>) -> bool {
    state.as_rows() == end.as_rows()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_parsing() {
        let input = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########";
        assert_eq!(5, generator_input(input).num_rows());
    }

    #[test]
    fn test_adj_list() {
        let edges = all_valid_states(&generator_input(INPUT));
        assert_eq!(28, edges.len());
        println!("---------------------");
        let next = edges.iter().filter(|&s| s.node_state[(0, 3)] == 'B' && s.cost == 40).next().unwrap().node_state.clone();
        let edges = all_valid_states(&next);
        assert_eq!(1, edges.iter().filter(|&s| s.node_state[(1, 6)] == 'C' && s.cost == 400).count());

        let stop_state = vec![vec!['.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.'], vec!['#', '#', 'B', '#', '.', '#', 'C', '#', 'D', '#', '#'], vec!['#', '#', 'A', '#', 'D', '#', 'C', '#', 'A', '#', '#']];
        let start = Array2D::from_rows(&stop_state);
        let edges = all_valid_states(&start);
        println!("---------------------");
        let next = edges.iter().filter(|&s| s.node_state[(0, 5)] == 'D' && s.cost == 3000).next().unwrap().node_state.clone();
        let edges = all_valid_states(&next);
        assert_eq!(1, edges.iter().filter(|&s| s.node_state[(0, 5)] == 'D' && s.node_state[(2, 4)] == 'B').count());
    }

    #[test]
    fn test_euclidean_dist() {
        assert_eq!(40, euclidean_dist('B', (1, 6), (0, 3)));
        assert_eq!(400, euclidean_dist('C', (1, 4), (1, 6)));
        assert_eq!(3000, euclidean_dist('D', (2, 4), (0, 5)));
        assert_eq!(30, euclidean_dist('B', (0, 3), (2, 4)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(12521, part1(&generator_input(INPUT)));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(44169, part2(&generator_input(INPUT)));
    }
}