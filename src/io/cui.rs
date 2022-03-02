use std::io::Write;

use crate::core::board::{Board, Position};
use crate::core::ruversi::{TurnPlayer, GameResult, IO, Input};

#[derive(Clone)]
pub struct CUI;

impl CUI {
    pub fn new() -> Self {
        Self
    }

    fn input_num(prompt: &str) -> i32 {
        loop {
            print!("{}", prompt);
            std::io::stdout().flush();
            match Self::read_num() {
                Ok(num) if (1..=8).contains(&num) => return num, 
                Ok(num) => println!("{} is not valid.", num),
                Err(msg) => println!("{}", msg),
            }
        }
    }


    fn read_num() -> Result<i32, String> {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).map_err(|_| "read_line error")?;
        s.retain(|c| c != '\n');
        s.parse::<i32>().map_err(|e| e.to_string())
    }
}

impl IO for CUI {
    fn game_start(&self, board: &Board) {
        println!("Game Start");
        println!("{}", board);
    }

    fn skip_turn(&self, turn: &TurnPlayer) {
        println!("There is no place to a {} disk.", turn);
    }

    fn start_turn(&self, turn: &TurnPlayer) {
        println!("{} turn:", turn);
    }

    fn before_mov(&self, board: &Board, turn: &TurnPlayer) {
        println!("{} move", turn);
    }

    fn after_illegal_mov(&self, pos: &Position, turn: &TurnPlayer) {
        println!("A disk cannot be placed on ({}, {})", pos.x + 1, pos.y + 1);
    }

    fn after_mov(&self, pos: &Position, turn: &TurnPlayer) {
        println!("A disk be place on {}", turn);
    }

    fn after_update(&self, board: &Board) {
        println!("{}", board);
    }

    fn game_end(&self, board: &Board, result: &GameResult) {
        println!("Result");
        println!("{}", board);
        println!("Dark vs Light");
        println!("{} : {}", result.dark_disks, result.light_disks);

        match result.winner {
            None => println!("Draw"),
            Some(player) => println!("{} WIN", player),
        }
    }
}

impl Input for CUI {
    fn input_pos(&self) -> Position {
        let x = Self::input_num("input x >> ") - 1;
        let y = Self::input_num("input y >> ") - 1;
        Position::new(x, y).expect("the range of x and y must be valid.")
    }
}
