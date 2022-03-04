use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::disk::Disk;

#[derive(EnumIter)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::UpRight => (1, 1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, -1),
            Direction::Down => (0, -1),
            Direction::DownLeft => (-1, -1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, 1),
        }
    }
}

#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    // Todo: fix to return Postion instead of Result<Positoin, &'static str>
    pub fn new(x: i32, y: i32) -> Result<Position, &'static str> {
        if !(0..8).contains(&x) {
            return Err("x must be in 0 to 7");
        }

        if !(0..8).contains(&y) {
            return Err("y must be in 0 to 7");
        }

        Ok(Position { x, y })
    }

    fn next(&self, dir: &Direction) -> Result<Self, &'static str> {
        let (dx, dy) = dir.tuple();
        Self::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    disks: [Option<Disk>; 64],
}

struct BoardLineIter<'a> {
    board: &'a Board,
    pos: Position,
    dir: Direction,
}

impl<'a> BoardLineIter<'a> {
    fn new(board: &'a Board, pos: Position, dir: Direction) -> Self {
        Self { board, pos, dir }
    }
}

impl<'a> Iterator for BoardLineIter<'a> {
    type Item = &'a Disk;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos = self.pos.next(&self.dir).ok()?;
        self.board.get(&self.pos)
    }
}

struct BoardLineIterMut<'a> {
    board: &'a mut Board,
    pos: Position,
    dir: Direction,
}

impl<'a> BoardLineIterMut<'a> {
    fn new(board: &'a mut Board, pos: Position, dir: Direction) -> Self {
        Self { board, pos, dir }
    }
}

impl<'a> Iterator for BoardLineIterMut<'a> {
    type Item = &'a mut Disk;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos = self.pos.next(&self.dir).ok()?;

        self.board.get_mut(&self.pos).map(|disk| {
            let disk_ptr: *mut Disk = disk;
            unsafe { &mut *disk_ptr }
        })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  x 1 2 3 4 5 6 7 8")?;
        writeln!(f, "y\n")?;
        for y in 0..8 {
            write!(f, "{}  ", y + 1)?;
            for x in 0..8 {
                let index = 8 * y + x;
                match self.disks[index] {
                    Some(disk) => write!(f, " {}", disk)?,
                    None => write!(f, " _")?,
                }
            }

            if y < 7 {
                writeln!(f)?
            }
        }

        write!(f, "")
    }
}

impl Board {
    pub fn new() -> Self {
        Self { disks: [None; 64] }
    }

    pub fn try_from_str(source: &str) -> Result<Self, &'static str> {
        let mut board = Self::new();

        for y in 0..8 {
            for x in 0..8 {
                let pos = Position::new(x, y).unwrap();
                let idx = Self::get_index(&pos);

                match source.chars().nth(idx) {
                    Some(c) if c == 'o' => board.set(&pos, Disk::Light),
                    Some(c) if c == 'x' => board.set(&pos, Disk::Dark),
                    Some(c) if c == '_' => (),
                    Some(_) => return Err(r#"character must be 'x', 'o', or '_' "#),
                    None => return Err("the length of source is not enough"),
                };
            }
        }

        Ok(board)
    }

    fn line_iter<'a>(&'a self, pos: Position, dir: Direction) -> BoardLineIter<'a> {
        BoardLineIter::new(self, pos, dir)
    }

    fn line_iter_mut<'a>(&'a mut self, pos: Position, dir: Direction) -> BoardLineIterMut<'a> {
        BoardLineIterMut::new(self, pos, dir)
    }

    pub fn count_legal_movs(&self, disk: Disk) -> i32 {
        let mut count = 0;
        for x in 0..8 {
            for y in 0..8 {
                let pos = Position::new(x, y).unwrap();
                if self.can_place(pos, disk) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn exists_legal_mov(&self, disk: Disk) -> bool {
        self.count_legal_movs(disk) > 0
    }

    pub fn count_turn_disks(&self, pos: Position, disk: Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disks already in the position.");
        }

        Ok(Direction::iter()
            .map(|dir| {
                self.count_line_disks_sandwitched_by_another_colors(pos.clone(), dir, &disk)
                    .unwrap_or(0)
            })
            .sum())
        .and_then(|c| match c {
            0 => Err("There is no disk to turn."),
            _ => Ok(c),
        })
    }

    fn count_line_disks_sandwitched_by_another_colors(
        &self,
        pos: Position,
        dir: Direction,
        end_disk: &Disk,
    ) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disk already.");
        }

        let mut iter = self.line_iter(pos, dir);
        match iter.next() {
            Some(disk) if disk != end_disk => {
                Self::count_line_disks_end_another_color(iter, end_disk).map(|c| c + 1)
            }
            _ => Err("There is no disk to turn"),
        }
    }

    fn count_line_disks_end_another_color(
        mut iter: BoardLineIter,
        end_disk: &Disk,
    ) -> Result<i32, &'static str> {
        match iter.next() {
            Some(disk) => {
                if disk == end_disk {
                    Ok(0)
                } else {
                    Self::count_line_disks_end_another_color(iter, end_disk).map(|c| c + 1)
                }
            }
            None => Err("The end of this line has not another color."),
        }
    }

    pub fn turn_disks(&mut self, pos: Position, disk: Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disks already in the position.");
        }

        Ok(Direction::iter()
            .map(|dir| {
                self.turn_line_disks_sandwitched_by_another_colors(pos.clone(), dir, &disk)
                    .unwrap_or(0)
            })
            .sum())
        .and_then(|c| match c {
            0 => Err("There is no disk to turn"),
            _ => Ok(c),
        })
    }

    fn turn_line_disks_sandwitched_by_another_colors(
        &mut self,
        pos: Position,
        dir: Direction,
        end_disk: &Disk,
    ) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disk already.");
        }

        let mut iter = self.line_iter_mut(pos, dir);
        match iter.next() {
            Some(disk) if disk != end_disk => {
                Self::turn_line_disks_end_another_color(iter, end_disk).map(|c| {
                    *disk = *end_disk;
                    c + 1
                })
            }
            _ => Err("There is no disk to turn"),
        }
    }

    fn turn_line_disks_end_another_color(
        mut iter: BoardLineIterMut,
        end_disk: &Disk,
    ) -> Result<i32, &'static str> {
        match iter.next() {
            Some(disk) => {
                if disk == end_disk {
                    Ok(0)
                } else {
                    Self::turn_line_disks_end_another_color(iter, end_disk).map(|c| {
                        *disk = *end_disk;
                        c + 1
                    })
                }
            }
            None => Err("The end of this line has not another color."),
        }
    }

    fn get_index(pos: &Position) -> usize {
        (8 * pos.y + pos.x) as usize
    }

    pub fn is_empty(&self, pos: &Position) -> bool {
        self.get(pos).is_none()
    }

    pub fn get(&self, pos: &Position) -> Option<&Disk> {
        let idx = Self::get_index(pos);
        self.disks.get(idx).unwrap().as_ref()
    }

    pub fn get_mut(&mut self, pos: &Position) -> Option<&mut Disk> {
        let idx = Self::get_index(pos);
        self.disks.get_mut(idx).unwrap().as_mut()
    }

    pub fn can_place(&self, pos: Position, disk: Disk) -> bool {
        self.count_turn_disks(pos, disk).is_ok()
    }

    pub fn place(&mut self, pos: Position, disk: Disk) -> Result<i32, &'static str> {
        self.turn_disks(pos.clone(), disk.clone()).map(|c| {
            let idx = Self::get_index(&pos);
            self.disks[idx] = Some(disk);
            c
        })
    }

    pub fn set(&mut self, pos: &Position, disk: Disk) {
        let index = Self::get_index(&pos);
        self.disks[index] = Some(disk);
    }

    pub fn count_disks(&self, disk: &Disk) -> usize {
        self.disks
            .iter()
            .filter(|&&disk_opt| disk_opt == Some(*disk))
            .count()
    }
}

#[macro_export]
macro_rules! board {
    ( $( [($x:expr, $y:expr), $disk:expr] ),* ) => {
        {
            let mut board = Board::new();
            $(
                board.set(&Position::new($x, $y).unwrap(), $disk);
            )*
            board
        }
    };
}

#[macro_export]
macro_rules! board_fig {
    ($( $line:expr ),*) => {{
        let mut source = String::new();
        $(
            source += $line;
        )*
        Board::try_from_str(&source).unwrap()
    }}
}

//Todo: rewrite board to use board_fig!
#[cfg(test)]
mod tests {
    use super::super::disk::Disk::{Dark, Light};
    use super::*;

    #[test]
    fn test_display() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________",
            "________",
            "________",
            "___ox___",
            "___xo___",
            "________",
            "________",
            "________"
        );

        #[rustfmt::skip]
        let board_str = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "  x 1 2 3 4 5 6 7 8",
            "y",
            "",
            "1   _ _ _ _ _ _ _ _",
            "2   _ _ _ _ _ _ _ _",
            "3   _ _ _ _ _ _ _ _",
            "4   _ _ _ o x _ _ _",
            "5   _ _ _ x o _ _ _",
            "6   _ _ _ _ _ _ _ _",
            "7   _ _ _ _ _ _ _ _",
            "8   _ _ _ _ _ _ _ _",
        );
        assert_eq!(format!("{}", board), board_str);
    }

    #[test]
    fn test_count_turn_disks1() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________",
            "________",
            "________",
            "___o____",
            "___x____",
            "________",
            "________",
            "________"
        );

        assert_eq!(
            board.count_turn_disks(Position::new(3, 2).unwrap(), Dark),
            Ok(1)
        );
    }

    #[test]
    fn test_count_turn_disks2() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________", 
            "________", 
            "________", 
            "___o____", 
            "___o____", 
            "___o____", 
            "___x____",
            "________"
        );

        assert_eq!(
            board.count_turn_disks(Position::new(3, 2).unwrap(), Dark),
            Ok(3)
        );
    }

    #[test]
    fn test_count_turn_disks3() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________", 
            "________", 
            "___o____", 
            "___o____", 
            "___o____", 
            "___x____", 
            "___o____",
            "___o____"
        );

        assert_eq!(
            board.count_turn_disks(Position::new(3, 1).unwrap(), Dark),
            Ok(3)
        );
    }

    #[test]
    fn test_count_turn_disks4() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________", 
            "________", 
            "________", 
            "___o___o", 
            "____x__x", 
            "_____x_x", 
            "______xx",
            "___oxxx_"
        );

        assert_eq!(
            board.count_turn_disks(Position::new(7, 7).unwrap(), Light),
            Ok(9)
        );
    }

    #[test]
    fn test_count_turn_disks5() {
        #[rustfmt::skip]
        let board = board_fig!(
            "o__o__o_", 
            "_x_x_x__", 
            "__xxx___", 
            "oxx_xxxo", 
            "__xxx___", 
            "_x_x_x__", 
            "o__x__x_",
            "___o___o"
        );

        assert_eq!(
            board.count_turn_disks(Position::new(3, 3).unwrap(), Light),
            Ok(19)
        );
    }

    #[test]
    fn test_count_turn_disks_err1() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________",
            "________",
            "________",
            "___o____",
            "___o____",
            "___o____",
            "________",
            "________"
        );

        assert!(board
            .count_turn_disks(Position::new(3, 2).unwrap(), Dark)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 3).unwrap(), Dark)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 4).unwrap(), Dark)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 5).unwrap(), Dark)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 6).unwrap(), Dark)
            .is_err());

        assert!(board
            .count_turn_disks(Position::new(3, 2).unwrap(), Light)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 3).unwrap(), Light)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 4).unwrap(), Light)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 5).unwrap(), Light)
            .is_err());
        assert!(board
            .count_turn_disks(Position::new(3, 6).unwrap(), Light)
            .is_err());
    }

    #[test]
    fn test_count_turn_disks_err2() {
        #[rustfmt::skip]
        let board = board_fig!(
            "x__x__x_", 
            "_x_x_x__", 
            "__xxx___", 
            "xxx_xxxx", 
            "__xxx___", 
            "_x_x_x__", 
            "x__x__x_",
            "___x___x"
        );

        assert!(board
            .count_turn_disks(Position::new(3, 3).unwrap(), Light)
            .is_err());
    }

    #[test]
    fn test_count_turn_disks_err3() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________",
            "________",
            "________",
            "________",
            "________",
            "________",
            "________",
            "_____x__"
        );

        assert!(board
            .count_turn_disks(Position::new(5, 6).unwrap(), Light)
            .is_err());
    }

    #[test]
    fn test_place1() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "________",
            "________",
            "________",
            "___o____",
            "___x____",
            "________",
            "________",
            "________"
        );

        assert_eq!(board.place(Position::new(3, 2).unwrap(), Dark), Ok(1));
        #[rustfmt::skip]
        assert_eq!(
            board,
            board_fig!(
                "________",
                "________",
                "___x____",
                "___x____",
                "___x____",
                "________",
                "________",
                "________"
            )
        );
    }

    #[test]
    fn test_place2() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "________", 
            "________", 
            "________", 
            "___o____", 
            "___o____", 
            "___o____", 
            "___x____",
            "________"
        );

        assert_eq!(board.place(Position::new(3, 2).unwrap(), Dark), Ok(3));
        #[rustfmt::skip]
        assert_eq!(
            board,
            board_fig!(
                "________", 
                "________", 
                "___x____", 
                "___x____", 
                "___x____", 
                "___x____", 
                "___x____",
                "________"
            )
        );
    }

    #[test]
    fn test_place3() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "________", 
            "________", 
            "___o____", 
            "___o____", 
            "___o____", 
            "___x____", 
            "___o____",
            "___o____"
        );

        assert_eq!(board.place(Position::new(3, 1).unwrap(), Dark), Ok(3));
        #[rustfmt::skip]
        assert_eq!(
            board,
            board_fig!(
                "________", 
                "___x____", 
                "___x____", 
                "___x____", 
                "___x____", 
                "___x____", 
                "___o____",
                "___o____"
            )
        );
    }

    #[test]
    fn test_place4() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "________", 
            "________", 
            "________", 
            "___o___o", 
            "____x__x", 
            "_____x_x", 
            "______xx",
            "___oxxx_"
        );

        assert_eq!(board.place(Position::new(7, 7).unwrap(), Light), Ok(9));
        #[rustfmt::skip]
        assert_eq!(
            board,
            board_fig!(
                "________", 
                "________", 
                "________", 
                "___o___o", 
                "____o__o", 
                "_____o_o", 
                "______oo",
                "___ooooo"
            )
        );
    }

    #[test]
    fn test_place5() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "o__o__o_", 
            "_x_x_x__", 
            "__xxx___", 
            "oxx_xxxo", 
            "__xxx___", 
            "_x_x_x__", 
            "o__x__x_",
            "___o___o"
        );

        assert_eq!(board.place(Position::new(3, 3).unwrap(), Light), Ok(19));
        #[rustfmt::skip]
        assert_eq!(
            board,
            board_fig!(
                "o__o__o_", 
                "_o_o_o__", 
                "__ooo___", 
                "oooooooo", 
                "__ooo___", 
                "_o_o_o__", 
                "o__o__o_",
                "___o___o"
            )
        );
    }

    #[test]
    fn test_place_err1() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "________", 
            "________", 
            "________", 
            "___o____", 
            "___o____", 
            "___o____", 
            "________",
            "________"
        );

        assert!(board.place(Position::new(3, 2).unwrap(), Dark).is_err());
        assert!(board.place(Position::new(3, 3).unwrap(), Dark).is_err());
        assert!(board.place(Position::new(3, 4).unwrap(), Dark).is_err());
        assert!(board.place(Position::new(3, 5).unwrap(), Dark).is_err());
        assert!(board.place(Position::new(3, 6).unwrap(), Dark).is_err());

        assert!(board.place(Position::new(3, 2).unwrap(), Light).is_err());
        assert!(board.place(Position::new(3, 3).unwrap(), Light).is_err());
        assert!(board.place(Position::new(3, 4).unwrap(), Light).is_err());
        assert!(board.place(Position::new(3, 5).unwrap(), Light).is_err());
        assert!(board.place(Position::new(3, 6).unwrap(), Light).is_err());

        #[rustfmt::skip]
        assert_eq!(
            board,
            board_fig!(
                "________", 
                "________", 
                "________", 
                "___o____", 
                "___o____", 
                "___o____", 
                "________",
                "________"
            )
        );
    }

    #[test]
    fn test_place_err2() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "x__x__x_", 
            "_x_x_x__", 
            "__xxx___", 
            "xxx_xxxx", 
            "__xxx___", 
            "_x_x_x__", 
            "x__x__x_",
            "___x___x"
        );

        assert!(board.place(Position::new(3, 3).unwrap(), Light).is_err());

        #[rustfmt::skip]
        assert_eq!(
            board,
            board_fig!(
                "x__x__x_", 
                "_x_x_x__", 
                "__xxx___", 
                "xxx_xxxx", 
                "__xxx___", 
                "_x_x_x__", 
                "x__x__x_",
                "___x___x"
            )
        );
    }

    #[test]
    fn test_place_err3() {
        #[rustfmt::skip]
        let mut board = board_fig!(
            "________"                  ,
            "________",
            "________",
            "________",
            "________",
            "________",
            "________",
            "_____x__"
        );

        assert!(board.place(Position::new(5, 6).unwrap(), Light).is_err());

        #[rustfmt::skip]
        assert_eq!(
            board, 
            board_fig!(
                "________"                  ,
                "________",
                "________",
                "________",
                "________",
                "________",
                "________",
                "_____x__"
            )
        );
    }

    #[test]
    fn test_count_disks1() {
        let board = Board::new();

        assert_eq!(board.count_disks(&Dark), 0);
        assert_eq!(board.count_disks(&Light), 0);
    }

    #[test]
    fn test_count_disks2() {
        #[rustfmt::skip]
        let board = board_fig!(
            "o__o__o_", 
            "_x_x_x__", 
            "__xxx___", 
            "oxx_xxxo", 
            "__xxx___", 
            "_x_x_x__", 
            "o__x__x_",
            "___o___o"
        );

        assert_eq!(board.count_disks(&Dark), 19);
        assert_eq!(board.count_disks(&Light), 8);
    }

    #[test]
    fn test_count_legal_movs1() {
        let board = Board::new();

        assert_eq!(board.count_legal_movs(Dark), 0);
        assert_eq!(board.count_legal_movs(Light), 0);
    }

    #[test]
    fn test_count_legal_movs2() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________", 
            "________", 
            "________", 
            "___o____", 
            "___o____", 
            "___o____", 
            "___x____",
            "________"
        );

        assert_eq!(board.count_legal_movs(Dark), 1);
        assert_eq!(board.count_legal_movs(Light), 1);
    }

    #[test]
    fn test_count_legal_movs3() {
        #[rustfmt::skip]
        let board = board_fig!(
            "________", 
            "________", 
            "________", 
            "__xxx___", 
            "___xo___", 
            "________", 
            "________",
            "________"
        );

        assert_eq!(board.count_legal_movs(Dark), 3);
        assert_eq!(board.count_legal_movs(Light), 3);
    }
}
