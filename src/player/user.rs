use crate::core::{board::{Board, Position, Disk}, player::Player, ruversi::Input};

struct User {
    input: Box<dyn Input>,
}

impl Player for User {
    fn init(&mut self, board: Board) {
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
    fn new(input: Box<dyn Input>) -> Self {
        Self { input }
    }
}

