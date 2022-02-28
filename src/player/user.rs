use crate::core::{board::Board, player::Player};

struct User {
    board: Board,
}

impl Player for User {
    fn init(&mut self, board: Board) {
        todo!()
    }

    fn update(&mut self, pos: crate::core::board::Position, disk: crate::core::board::Disk) {
        todo!()
    }

    fn mov(&self) -> crate::core::board::Position {
        todo!()
    }
}

impl User {
    fn new() -> Self {
        unimplemented!();
    }
}
