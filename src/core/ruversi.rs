use super::board::*;
use super::player::Player;

use std::cmp::Ordering::{Less, Greater, Equal};

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

    fn init_players(&mut self) {
        self.player_dark.init(self.board.clone());
        self.player_light.init(self.board.clone());
    }

    fn game_start(&self) {
        "Game Start";
    }

    fn ends_game(&self, skip_count: i32) -> bool {
        // Todo: add a condition of no room to put a disk.
        skip_count >= 2
    }

    fn turn_player_mov(&mut self, turn: TurnPlayer) -> Position {
        loop {
            let pos = self.get_turn_player(turn).mov();
            if self.board.can_place(pos.clone(), turn.into_disk()) {
                return pos;
            }
        }
    }

    fn exists_legal_mov(&self, player: TurnPlayer) -> bool {
        self.board.exists_legal_mov(player.into_disk())
    }

    fn update(&mut self, pos: Position, disk: Disk) {
        self.player_dark.update(pos.clone(), disk);
        self.player_light.update(pos, disk);
    }

    fn skip_turn(&self) {
        "turn skip";
    }

    fn game_end(&self) {
        let dark_disks = self.board.count_disks(&Disk::Dark);
        let light_disks =  self.board.count_disks(&Disk::Light);

        match dark_disks.cmp(&light_disks) {
            Less => "Dark win",
            Greater => "Light win",
            Equal =>  "Draw",
        };
    }

    fn get_turn_player(&mut self, turn: TurnPlayer) -> &mut Box<dyn Player> {
        match turn {
            TurnPlayer::Dark => &mut self.player_dark,
            TurnPlayer::Light => &mut self.player_light,
        }
    }

    pub fn run(&mut self) {
        let mut turn_player = TurnPlayer::Dark;
        let mut skip_count = 0;

        self.init_players();
        self.game_start();

        loop {
            if self.ends_game(skip_count) {
                break;
            }

            if self.exists_legal_mov(turn_player) {
                let pos = self.turn_player_mov(turn_player);
                self.update(pos, turn_player.into_disk());
                skip_count = 0;
            } else {
                self.skip_turn();
                skip_count += 1;
            }

            turn_player.make_turn();
        }

        self.game_end();
    }

}
