use std::collections::HashMap;

#[derive(Debug)]
struct GameState {
    p1_pos: u8,
    p1_score: u16,
    p2_pos: u8,
    p2_score: u16,
}

impl GameState {
    #[allow(dead_code)]
    fn move_player1(&mut self, rolled: u16) {
        self.p1_pos = ((self.p1_pos as u16 + rolled - 1) % 10 + 1) as u8;
        self.p1_score += self.p1_pos as u16;
    }
    #[allow(dead_code)]
    fn move_player2(&mut self, rolled: u16) {
        self.p2_pos = ((self.p2_pos as u16 + rolled - 1) % 10 + 1) as u8;
        self.p2_score += self.p2_pos as u16;
    }
}

#[allow(dead_code)]
fn play_game() {
    let mut game = GameState {
        p1_pos: 4,
        p1_score: 0,
        p2_pos: 8,
        p2_score: 0,
    };
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
        println!("rolled; {}", roll);
        if player1_turn {
            game.move_player1(roll);
            player1_turn = false;
        } else {
            game.move_player2(roll);
            player1_turn = true;
        }
    }
    if game.p1_score > 1000 {
        println!("Player 1 won! (state: {:?}) total rolled: {}", game, total_rolled);
    } else {
        println!("Player 2 won! (state: {:?}) total rolled: {}", game, total_rolled);
    }
}

#[allow(dead_code)]
fn part2() {
    let mut quantum_die: HashMap<u8,u64> = HashMap::new();
    quantum_die.insert(3, 1);
    quantum_die.insert(4, 3);
    quantum_die.insert(5, 6);
    quantum_die.insert(6, 7);
    quantum_die.insert(7, 6);
    quantum_die.insert(8, 3);
    quantum_die.insert(9, 1);

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
fn test_playing() {
    play_game();
}