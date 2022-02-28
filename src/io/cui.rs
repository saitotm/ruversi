use crate::core::board::{Board, Position};
use crate::core::ruversi::{TurnPlayer, GameResult};
use super::super::core::ruversi::IO;

struct CUI;

impl CUI {
    fn new() -> Self {
        Self
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
        println!("A disk cannot be placed on {}", pos);
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
