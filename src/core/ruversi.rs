use super::board::*;
use super::player::Player;

use std::cmp::Ordering::{Less, Greater, Equal};

pub trait IO {
    fn game_start(&self, board: &Board);
    fn skip_turn(&self, turn: &TurnPlayer);
    fn start_turn(&self, turn: &TurnPlayer);
    fn before_mov(&self, board: &Board, turn: &TurnPlayer);
    fn after_illegal_mov(&self, pos: &Position, turn: &TurnPlayer);
    fn after_mov(&self, pos: &Position, turn: &TurnPlayer);
    fn after_update(&self, board: &Board);
    fn game_end(&self, board: &Board, result: &GameResult);
}

#[derive(Clone, Copy)]
pub enum TurnPlayer {
    Dark,
    Light,
}

impl TurnPlayer {
    fn other(turn_player: &Self) -> Self {
        match turn_player {
            TurnPlayer::Dark => TurnPlayer::Light,
            TurnPlayer::Light => TurnPlayer::Dark,
        }
    }

    fn reverse(&mut self) {
        *self = Self::other(self);
    }

    fn into_disk(self) -> Disk {
        match self {
            TurnPlayer::Dark => Disk::Dark,
            TurnPlayer::Light => Disk::Light,
        }
    }
}

pub struct GameResult {
    pub light_disks: usize,
    pub dark_disks: usize,
    pub winner: Option<TurnPlayer>,
}

impl GameResult {
    fn new(light_disks: usize, dark_disks: usize) -> Self {
        let winner = match dark_disks.cmp(&light_disks) {
            Less => Some(TurnPlayer::Light),
            Greater => Some(TurnPlayer::Dark),
            Equal =>  None,
        };

        Self { light_disks, dark_disks, winner }
    }
}

pub struct Ruversi {
    board: Board,
    player_dark: Box<dyn Player>,
    player_light: Box<dyn Player>,
    io: Box<dyn IO>,
}

impl Ruversi {
    pub fn new(board: Board, player_dark: Box<dyn Player>, player_light: Box<dyn Player>, io: Box<dyn IO>) -> Self {
        Self { board, player_dark, player_light, io }
    }

    fn init_players(&mut self) {
        self.player_dark.init(self.board.clone());
        self.player_light.init(self.board.clone());
    }

    fn game_start(&self) {
        self.io.game_start(&self.board);
    }

    fn ends_game(&self, skip_count: i32) -> bool {
        // Todo: add a condition of no room to put a disk.
        skip_count >= 2
    }

    fn turn_player_mov(&mut self, turn: TurnPlayer) -> Position {
        loop {
            self.io.before_mov(&self.board, &turn);
            let pos = self.get_turn_player(turn).mov();
            if self.board.can_place(pos.clone(), turn.into_disk()) {
                self.io.after_mov(&pos, &turn);
                return pos;
            } else {
                self.io.after_illegal_mov(&pos, &turn);
            }
        }
    }

    fn start_turn(&self, player: TurnPlayer) {
        self.io.start_turn(&player);
    }

    fn exists_legal_mov(&self, player: TurnPlayer) -> bool {
        self.board.exists_legal_mov(player.into_disk())
    }

    fn update(&mut self, pos: Position, disk: Disk) {
        self.board.place(pos.clone(), disk).expect("A disk must be able to place on the pos.");
        self.player_dark.update(pos.clone(), disk);
        self.player_light.update(pos, disk);

        self.io.after_update(&self.board);
    }

    fn skip_turn(&self, turn: TurnPlayer) {
        self.io.skip_turn(&turn);
    }

    fn game_end(&self) {
        let dark_disks = self.board.count_disks(&Disk::Dark);
        let light_disks =  self.board.count_disks(&Disk::Light);

        let result = GameResult::new(light_disks, dark_disks);
        self.io.game_end(&self.board, &result);
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

            self.start_turn(turn_player);
            if self.exists_legal_mov(turn_player) {
                let pos = self.turn_player_mov(turn_player);
                self.update(pos, turn_player.into_disk());
                skip_count = 0;
            } else {
                self.skip_turn(turn_player);
                skip_count += 1;
            }

            turn_player.reverse();
        }

        self.game_end();
    }

}
