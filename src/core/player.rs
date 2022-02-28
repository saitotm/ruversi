use super::board::*;

pub trait Player {
    fn init(&mut self, board: Board);
    fn update(&mut self, pos: Position, disk: Disk);
    fn mov(&self) -> Position;
}
