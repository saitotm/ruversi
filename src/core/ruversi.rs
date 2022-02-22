use super::board::Board;
use super::player::Player;

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

    fn turn_player_mov(&self) {
        unimplemented!();
    }

    fn update(&mut self) {
        unimplemented!();
    }

    fn game_end(&self) {
        unimplemented!();
    }

    pub fn run(&mut self) {
        unimplemented!();
        /*
        self.game_start();

        let mut turn_player = TurnPlayer::Dark;
        loop {
            self.turn_player_mov();
            self.update();
            turn_player.make_turn();
        }

        self.game_end();
        */
    }

}
