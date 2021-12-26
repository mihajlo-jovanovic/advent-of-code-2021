use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Amphipod {
    kind: char,
    position: (u8, u8),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    positions: Vec<(char, u8, u8)>,
}

// impl Ord for State {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//             .then_with(|| self.positions.cmp(&other.position))
//     }
// }
//
// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

#[allow(dead_code)]
fn lowest_risk(starting_config: Vec<(char, u8, u8)>) -> Option<usize> {
    //let end = (matrix.num_rows() - 1, matrix.num_columns() - 1);
    let mut dist = HashMap::new();

    //let mut heap = BinaryHeap::new();
    dist.insert(starting_config, 0);
    //heap.push(State { cost: 0, positions: starting_config.clone() });

    // Examine the frontier with lower cost nodes first (min-heap)
    //while let Some(State { cost, positions }) = heap.pop() {
    // Alternatively we could have continued to find all shortest paths
    //if final_position(positions) { return Some(cost); }

    // Important as we may have already found a better way
    //if cost > *dist.get(&positions).unwrap() { continue; }

    // For each node we can reach, see if we can find a way with
    // a lower cost going through this node
    // for edge in neighbors(matrix, &position) {
    //     let next = State { cost: cost + matrix[edge], position: edge };
    //
    //
    //     // If so, add it to the frontier and continue
    //     if next.cost < *dist.get(&next.position).unwrap() {
    //         // Relaxation, we have now found a better way
    //         dist.insert(next.position, next.cost);
    //         heap.push(next);
    //     }
    // }
    //}

    // Goal not reachable
    None
}

// fn final_position(positions: HashSet<Amphipod>) -> bool {
//     let a = positions.get('A').unwrap();
//     let b = positions.get('A').unwrap();
//     let c = positions.get('A').unwrap();
//     let d = positions.get('A').unwrap();
// }