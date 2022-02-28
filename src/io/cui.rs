use crate::core::board::{Board, Position};
use crate::core::ruversi::{TurnPlayer, GameResult};
use super::super::core::ruversi::IO;

struct CUI {

}

impl IO for CUI {
    fn game_start(&self, board: &Board) {
        unimplemented!()
    }

    fn skip_turn(&self, turn: &TurnPlayer) {
        unimplemented!()
    }

    fn start_turn(&self, turn: &TurnPlayer) {
        unimplemented!()
    }

    fn before_mov(&self, board: &Board, turn: &TurnPlayer) {
        unimplemented!()
    }

    fn after_illegal_mov(&self, pos: &Position, turn: &TurnPlayer) {
        unimplemented!()
    }

    fn after_mov(&self, pos: &Position, turn: &TurnPlayer) {
        unimplemented!()
    }

    fn after_update(&self, board: &Board) {
        unimplemented!()
    }

    fn game_end(&self, board: &Board, result: &GameResult) {
        unimplemented!()
    }
}
