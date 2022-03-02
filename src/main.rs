use crate::core::{ruversi::Ruversi, board::Board, board::Position};
use crate::core::board::Disk::*;

use io::cui::CUI;
use player::user::User;

mod core;
mod io;
mod player;

fn init_board() -> Board {
    board!( 
        [(3, 3), Light], [(3, 4), Dark],
        [(4, 3), Dark], [(4, 4), Light] 
    )
}

fn main() {
    let board = init_board();
    let io = CUI::new();
    let player_dark = User::new(Box::new(io.clone()));
    let player_light = User::new(Box::new(io.clone()));
    let mut ruversi = Ruversi::new(board, Box::new(player_dark), Box::new(player_light), Box::new(io));

    ruversi.run();
}
