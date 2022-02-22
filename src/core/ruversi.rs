use super::board::*;
use super::player::Player;

#[derive(Clone, Copy)]
pub enum TurnPlayer {
    Dark,
    Light,
}

impl TurnPlayer {
    fn another(turn_player: &Self) -> Self {
        match turn_player {
            TurnPlayer::Dark => TurnPlayer::Light,
            TurnPlayer::Light => TurnPlayer::Dark,
        }
    }

    fn make_turn(&mut self) {
        *self = Self::another(self);
    }

    fn into_disk(self) -> Disk {
        match self {
            TurnPlayer::Dark => Disk::Dark,
            TurnPlayer::Light => Disk::Light,
        }
    }
}

pub struct Ruversi {
    board: Board,
    player_dark: Box<dyn Player>,
    player_light: Box<dyn Player>,
}

impl Ruversi {
    pub fn new(board: Board, player_dark: Box<dyn Player>, player_light: Box<dyn Player>) -> Self {
        Self { board, player_dark, player_light }
    }

    fn game_start(&self) {
        unimplemented!();
    }

    fn ends_game(&self, skip_count: i32) -> bool {
        unimplemented!();
    }

    fn turn_player_mov(&self) -> Position {
        unimplemented!();
    }

    fn exists_legal_mov(&self, player: TurnPlayer) -> bool {
        self.board.exists_legal_mov(player.into_disk())
    }

    fn update(&mut self, pos: Position) {
        unimplemented!();
    }

    fn game_end(&self) {
        unimplemented!();
    }

    pub fn run(&mut self) {
        let mut turn_player = TurnPlayer::Dark;
        let mut skip_count = 0;


        self.game_start();

        loop {
            if self.ends_game(skip_count) {
                break;
            }

            if self.exists_legal_mov(turn_player) {
                let pos = self.turn_player_mov();
                self.update(pos);
                skip_count = 0;
            } else {
                skip_count += 1;
            }

            turn_player.make_turn();
        }

        self.game_end();
    }

}
