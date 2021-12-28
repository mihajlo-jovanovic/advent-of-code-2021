use std::collections::HashMap;

#[aoc_generator(day21)]
fn generator_input(_: &str) -> GameState {
    GameState {
        p1_pos: 1,
        p1_score: 0,
        p2_pos: 2,
        p2_score: 0,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct GameState {
    p1_pos: u8,
    p1_score: u16,
    p2_pos: u8,
    p2_score: u16,
}

impl GameState {
    fn move_player1(&mut self, rolled: u16) {
        self.p1_pos = ((self.p1_pos as u16 + rolled - 1) % 10 + 1) as u8;
        self.p1_score += self.p1_pos as u16;
    }
    fn move_player2(&mut self, rolled: u16) {
        self.p2_pos = ((self.p2_pos as u16 + rolled - 1) % 10 + 1) as u8;
        self.p2_score += self.p2_pos as u16;
    }
}

#[aoc(day21, part1)]
fn part1(start: &GameState) -> u32 {
    let mut game = *start;
    let mut die = 0;
    let mut player1_turn = true;
    let mut total_rolled = 0;
    while game.p1_score < 1000 && game.p2_score < 1000 {
        let mut roll = 0_u16;
        for _ in 0..3 {
            die = (die + 1) % 100;
            roll += die;
            total_rolled += 1;
        }
        //println!("rolled; {}", roll);
        if player1_turn {
            game.move_player1(roll);
            player1_turn = false;
        } else {
            game.move_player2(roll);
            player1_turn = true;
        }
    }
    if game.p1_score >= 1000 {
        println!(
            "Player 1 won! (state: {:?}) total rolled: {}",
            game, total_rolled
        );
        game.p2_score as u32 * total_rolled as u32
    } else {
        println!(
            "Player 2 won! (state: {:?}) total rolled: {}",
            game, total_rolled
        );
        game.p1_score as u32 * total_rolled
    }
}

#[aoc(day21, part2)]
fn part2(start: &GameState) -> u64 {
    let mut quantum_die: HashMap<u8, u64> = HashMap::new();
    quantum_die.insert(3, 1);
    quantum_die.insert(4, 3);
    quantum_die.insert(5, 6);
    quantum_die.insert(6, 7);
    quantum_die.insert(7, 6);
    quantum_die.insert(8, 3);
    quantum_die.insert(9, 1);

    let mut universes: HashMap<GameState, u64> = HashMap::new();

    universes.insert(*start, 1);
    let mut p1_wins = 0_u64;
    let mut p2_wins = 0_u64;
    let mut p1_turn = true;
    while !universes.is_empty() {
        universes = roll_quantum(universes, &quantum_die, p1_turn);
        for (k, v) in universes.clone() {
            if p1_turn && k.p1_score >= 21 {
                p1_wins += v;
                universes.remove(&k);
            } else if !p1_turn && k.p2_score >= 21 {
                p2_wins += v;
                universes.remove(&k);
            }
        }
        p1_turn = !p1_turn;
    }
    if p1_wins > p2_wins {
        println!("Player 1 won more games - {} to be exact", p1_wins);
        p1_wins
    } else {
        println!("Player 2 won more games - {} to be exact", p2_wins);
        p2_wins
    }
}

fn roll_quantum(before: HashMap<GameState, u64>, quantum_die: &HashMap<u8, u64>, p1: bool) -> HashMap<GameState, u64> {
    let mut result = HashMap::new();
    for (k, v) in before {
        for (rolled, freq) in quantum_die {
            let mut k2 = k;
            let v2 = v * freq;
            if p1 {
                k2.move_player1(*rolled as u16);
            } else {
                k2.move_player2(*rolled as u16);
            }
            if let Some(universes) = result.clone().get(&k2) {
                result.insert(k2, v2 + universes);
            } else {
                result.insert(k2, v2);
            }
        }
    }
    result
}

#[test]
fn test_roll_quantum() {
    let mut quantum_die: HashMap<u8, u64> = HashMap::new();
    quantum_die.insert(3, 1);
    quantum_die.insert(4, 3);
    quantum_die.insert(5, 6);
    quantum_die.insert(6, 7);
    quantum_die.insert(7, 6);
    quantum_die.insert(8, 3);
    quantum_die.insert(9, 1);
    let mut universes: HashMap<GameState, u64> = HashMap::new();
    let start = GameState {
        p1_pos: 4,
        p1_score: 0,
        p2_pos: 8,
        p2_score: 0,
    };
    universes.insert(start.clone(), 1);
    let result = roll_quantum(universes, &quantum_die, true);
    println!("{:?}", result);
    assert_eq!(None, result.get(&start));
    assert_eq!(7, result.len());
    assert_eq!(27, result.values().sum::<u64>());
    let after = GameState {
        p1_pos: 7,
        p1_score: 7,
        p2_pos: 8,
        p2_score: 0,
    };
    assert_eq!(1, *result.get(&after).unwrap());
    let after = GameState {
        p1_pos: 3,
        p1_score: 3,
        p2_pos: 8,
        p2_score: 0,
    };
    assert_eq!(1, *result.get(&after).unwrap());
}

#[test]
fn test_rolling() {
    let mut game = GameState {
        p1_pos: 4,
        p1_score: 0,
        p2_pos: 8,
        p2_score: 0,
    };
    game.move_player1(6);
    assert_eq!(10, game.p1_pos);
    assert_eq!(10, game.p1_score);
    game.move_player1(7 + 8 + 9);
    assert_eq!(4, game.p1_pos);
    assert_eq!(14, game.p1_score);
    game.move_player1(13 + 14 + 15);
    assert_eq!(6, game.p1_pos);
    assert_eq!(20, game.p1_score);
}

#[test]
fn test_part1() {
    assert_eq!(739785, part1(&GameState {
        p1_pos: 4,
        p1_score: 0,
        p2_pos: 8,
        p2_score: 0,
    }));
    assert_eq!(598416, part1(&GameState {
        p1_pos: 1,
        p1_score: 0,
        p2_pos: 2,
        p2_score: 0,
    }));
}

#[test]
fn test_part2() {
    assert_eq!(444356092776315, part2(&GameState {
        p1_pos: 4,
        p1_score: 0,
        p2_pos: 8,
        p2_score: 0,
    }))
}
