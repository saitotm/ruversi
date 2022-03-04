use crate::core::{board::Board, ruversi::Ruversi};

use io::cui::CUI;
use player::user::User;

mod core;
mod io;
mod player;

#[rustfmt::skip]
fn init_board() -> Board {
    board_fig!(
        "________",
        "________",
        "________",
        "___ox___",
        "___xo___",
        "________",
        "________",
        "________"
    )
}

fn main() {
    let board = init_board();
    let io = CUI::new();
    let player_dark = User::new(Box::new(io.clone()));
    let player_light = User::new(Box::new(io.clone()));
    let mut ruversi = Ruversi::new(
        board,
        Box::new(player_dark),
        Box::new(player_light),
        Box::new(io),
    );

    ruversi.run();
}
