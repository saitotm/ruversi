use super::board::*;

pub trait Player {
    fn update(&mut self, board: Board);
    fn mov(&self) -> Position;
}
