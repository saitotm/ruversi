use crate::core::{
    board::{Board, Disk, Position},
    player::Player,
    ruversi::Input,
};

pub struct User {
    input: Box<dyn Input>,
}

impl Player for User {
    fn init(&mut self, _board: Board) {
        // nothing to do
    }

    fn update(&mut self, _pos: Position, _disk: Disk) {
        // nothing to do
    }

    fn mov(&self) -> Position {
        self.input.input_pos()
    }
}

impl User {
    pub fn new(input: Box<dyn Input>) -> Self {
        Self { input }
    }
}
