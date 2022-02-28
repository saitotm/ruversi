use super::disk::Disk;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Result<Position, &'static str> {
        if !(0..8).contains(&x) {
            return Err("x must be in 0 to 7");
        }

        if !(0..8).contains(&y) {
            return Err("y must be in 0 to 7");
        }

        Ok(Position{x, y})
    }

    fn next(&self, dir: &Direction) -> Result<Self, &'static str> {
        let (dx, dy) = dir.tuple();
        Self::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    disks : [Option<Disk>; 64]
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

        self.board.get_mut(&self.pos)
        .map(|disk| {
            let disk_ptr: *mut Disk = disk;
            unsafe {
                &mut *disk_ptr
            }
        })
    }
}

impl Board {
    pub fn new() -> Self {
        Self { disks: [None; 64] }
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
            return Err("Exists disks already in the position.")
        }

        Ok(Direction::iter()
        .map(|dir| self.count_line_disks_sandwitched_by_another_colors(pos.clone(), dir, &disk).unwrap_or(0))
        .sum())
        .and_then(|c|
            match c {
                0 => Err("There is no disk to turn."),
                _ => Ok(c),
            }
        )
    }

    fn count_line_disks_sandwitched_by_another_colors(&self, pos: Position, dir: Direction, end_disk: &Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disk already.");
        }

        let mut iter = self.line_iter(pos, dir);
        match iter.next() {
            Some(disk) if disk != end_disk => {
                Self::count_line_disks_end_another_color(iter, end_disk).map(|c| c + 1)
            },
            _ => Err("There is no disk to turn"),
        }
    }

    fn count_line_disks_end_another_color(mut iter: BoardLineIter, end_disk: &Disk) -> Result<i32, &'static str> {
        match iter.next() {
            Some(disk) => 
                if disk == end_disk { Ok(0) } 
                else { Self::count_line_disks_end_another_color(iter, end_disk).map(|c| c + 1) },
            None => Err("The end of this line has not another color."),
        }
    }

    pub fn turn_disks(&mut self, pos: Position, disk: Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disks already in the position.")
        }

        Ok(Direction::iter()
        .map(|dir| self.turn_line_disks_sandwitched_by_another_colors(pos.clone(), dir, &disk).unwrap_or(0))
        .sum())
        .and_then( |c|
            match c {
                0 => Err("There is no disk to turn"),
                _ => Ok(c),
            }
        )
    }

    fn turn_line_disks_sandwitched_by_another_colors(&mut self, pos: Position, dir: Direction, end_disk: &Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disk already.");
        }

        let mut iter = self.line_iter_mut(pos, dir);
        match iter.next() {
            Some(disk) if disk != end_disk => {
                Self::turn_line_disks_end_another_color(iter, end_disk)
                .map(|c| {
                    *disk = *end_disk;
                    c + 1
                })
            },
            _ => Err("There is no disk to turn"),
        }
    }

    fn turn_line_disks_end_another_color(mut iter: BoardLineIterMut, end_disk: &Disk) -> Result<i32, &'static str> {
        match iter.next() {
            Some(disk) => 
                if disk == end_disk { Ok(0) } 
                else { 
                    Self::turn_line_disks_end_another_color(iter, end_disk)
                    .map(|c| {
                        *disk = *end_disk;
                        c + 1
                    }) 
                },
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
        self.turn_disks(pos.clone(), disk.clone())
        .map(|c| {
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
        self.disks.iter()
        .filter(|&&disk_opt| 
            disk_opt == Some(*disk)
        )
        .count()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::disk::Disk::{Light, Dark};


    #[test]
    fn test_count_turn_disks1() {
        let board = board!( 
            [(3, 3), Light], [(3, 4), Dark] 
        );
        
        assert_eq!(board.count_turn_disks(Position::new(3, 2).unwrap(), Dark), Ok(1));
    }

    #[test]
    fn test_count_turn_disks2() {
        let board = board!( 
            [(3, 3), Light], [(3, 4), Light], [(3, 5), Light], [(3, 6), Dark] 
        );
        
        assert_eq!(board.count_turn_disks(Position::new(3, 2).unwrap(), Dark), Ok(3));
    }

    #[test]
    fn test_count_turn_disks3() {
        let board = board!( 
            [(3, 2), Light], [(3, 3), Light], [(3, 4), Light], [(3, 5), Dark], [(3, 6), Light], [(3, 7), Light]
        );
        
        assert_eq!(board.count_turn_disks(Position::new(3, 1).unwrap(), Dark), Ok(3));
    }

    #[test]
    fn test_count_turn_disks4() {
        let board = board!( 
            [(7, 3), Light], [(7, 4), Dark], [(7, 5), Dark], [(7, 6), Dark],
            [(3, 7), Light], [(4, 7), Dark], [(5, 7), Dark], [(6, 7), Dark], 
            [(3, 3), Light], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark] 
        );
        
        assert_eq!(board.count_turn_disks(Position::new(7, 7).unwrap(), Light), Ok(9));
    }

    #[test]
    fn test_count_turn_disks5() {
        let board = board!(
            [(0, 0), Light], [(1, 1), Dark], [(2, 2), Dark], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark], [(7, 7), Light],
            [(6, 0), Light], [(5, 1), Dark], [(4, 2), Dark], [(2, 4), Dark], [(1, 5), Dark], [(0, 6), Light],
            [(0, 3), Light], [(1, 3), Dark], [(2, 3), Dark], [(4, 3), Dark], [(5, 3), Dark], [(6, 3), Dark], [(7, 3), Light],
            [(3, 0), Light], [(3, 1), Dark], [(3, 2), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Dark], [(3, 7), Light]
        );

        assert_eq!(board.count_turn_disks(Position::new(3, 3).unwrap(), Light), Ok(19));
    }

    #[test]
    fn test_count_turn_disks_err1() {
        let board = board!(
            [(3, 3), Light], [(3, 4), Light], [(3, 5), Light]
        );

        assert!(board.count_turn_disks(Position::new(3, 2).unwrap(), Dark).is_err());
        assert!(board.count_turn_disks(Position::new(3, 3).unwrap(), Dark).is_err());
        assert!(board.count_turn_disks(Position::new(3, 4).unwrap(), Dark).is_err());
        assert!(board.count_turn_disks(Position::new(3, 5).unwrap(), Dark).is_err());
        assert!(board.count_turn_disks(Position::new(3, 6).unwrap(), Dark).is_err());

        assert!(board.count_turn_disks(Position::new(3, 2).unwrap(), Light).is_err());
        assert!(board.count_turn_disks(Position::new(3, 3).unwrap(), Light).is_err());
        assert!(board.count_turn_disks(Position::new(3, 4).unwrap(), Light).is_err());
        assert!(board.count_turn_disks(Position::new(3, 5).unwrap(), Light).is_err());
        assert!(board.count_turn_disks(Position::new(3, 6).unwrap(), Light).is_err());
    }

    #[test]
    fn test_count_turn_disks_err2() {
        let board = board!( 
            [(0, 0), Dark], [(1, 1), Dark], [(2, 2), Dark], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark], [(7, 7), Dark],
            [(6, 0), Dark], [(5, 1), Dark], [(4, 2), Dark], [(2, 4), Dark], [(1, 5), Dark], [(0, 6), Dark],
            [(0, 3), Dark], [(1, 3), Dark], [(2, 3), Dark], [(4, 3), Dark], [(5, 3), Dark], [(6, 3), Dark], [(7, 3), Dark],
            [(3, 0), Dark], [(3, 1), Dark], [(3, 2), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Dark], [(3, 7), Dark]
        );

        assert!(board.count_turn_disks(Position::new(3, 3).unwrap(), Light).is_err());
    }

    #[test]
    fn test_count_turn_disks_err3() {
        let board = board!(
            [(5, 7), Dark]
        );

        assert!(board.count_turn_disks(Position::new(5, 6).unwrap(), Light).is_err());
    }


    #[test]
    fn test_place1() {
        let mut board = board!( 
            [(3, 3), Light], [(3, 4), Dark] 
        );

        assert_eq!(board.place(Position::new(3, 2).unwrap(), Dark), Ok(1));
        assert_eq!(
            board, 
            board!(
                [(3, 2), Dark], [(3, 3), Dark], [(3, 4), Dark]
            )
        );
    }

    #[test]
    fn test_place2() {
        let mut board = board!( 
            [(3, 3), Light], [(3, 4), Light], [(3, 5), Light], [(3, 6), Dark] 
        );

        assert_eq!(board.place(Position::new(3, 2).unwrap(), Dark), Ok(3));
        assert_eq!(
            board, 
            board!(
                [(3, 2), Dark], [(3, 3), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Dark] 
            )
        );
    }

    #[test]
    fn test_place3() {
        let mut board = board!( 
            [(3, 2), Light], [(3, 3), Light], [(3, 4), Light], [(3, 5), Dark], [(3, 6), Light], [(3, 7), Light]
        );
        
        assert_eq!(board.place(Position::new(3, 1).unwrap(), Dark), Ok(3));
        assert_eq!(
            board, 
            board!(
                [(3, 1), Dark], [(3, 2), Dark], [(3, 3), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Light], [(3, 7), Light]
            )
        );
    }

    #[test]
    fn test_place4() {
        let mut board = board!( 
            [(7, 3), Light], [(7, 4), Dark], [(7, 5), Dark], [(7, 6), Dark],
            [(3, 7), Light], [(4, 7), Dark], [(5, 7), Dark], [(6, 7), Dark], 
            [(3, 3), Light], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark] 
        );

        assert_eq!(board.place(Position::new(7, 7).unwrap(), Light), Ok(9));
        assert_eq!(
            board, 
            board!(
                [(7, 7), Light],
                [(7, 3), Light], [(7, 4), Light], [(7, 5), Light], [(7, 6), Light],
                [(3, 7), Light], [(4, 7), Light], [(5, 7), Light], [(6, 7), Light], 
                [(3, 3), Light], [(4, 4), Light], [(5, 5), Light], [(6, 6), Light] 
            )
        );
    }

    #[test]
    fn test_place5() {
        let mut board = board!( 
            [(0, 0), Light], [(1, 1), Dark], [(2, 2), Dark], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark], [(7, 7), Light],
            [(6, 0), Light], [(5, 1), Dark], [(4, 2), Dark], [(2, 4), Dark], [(1, 5), Dark], [(0, 6), Light],
            [(0, 3), Light], [(1, 3), Dark], [(2, 3), Dark], [(4, 3), Dark], [(5, 3), Dark], [(6, 3), Dark], [(7, 3), Light],
            [(3, 0), Light], [(3, 1), Dark], [(3, 2), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Dark], [(3, 7), Light]
        );

        assert_eq!(board.place(Position::new(3, 3).unwrap(), Light), Ok(19));
        assert_eq!(
            board, 
            board!(
                [(3, 3), Light],
                [(0, 0), Light], [(1, 1), Light], [(2, 2), Light], [(4, 4), Light], [(5, 5), Light], [(6, 6), Light], [(7, 7), Light],
                [(6, 0), Light], [(5, 1), Light], [(4, 2), Light], [(2, 4), Light], [(1, 5), Light], [(0, 6), Light],
                [(0, 3), Light], [(1, 3), Light], [(2, 3), Light], [(4, 3), Light], [(5, 3), Light], [(6, 3), Light], [(7, 3), Light],
                [(3, 0), Light], [(3, 1), Light], [(3, 2), Light], [(3, 4), Light], [(3, 5), Light], [(3, 6), Light], [(3, 7), Light]
            )
        );
    }

    #[test]
    fn test_place_err1() {
        let mut board = board!(
            [(3, 3), Light], [(3, 4), Light], [(3, 5), Light]
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

        assert_eq!(
            board, 
            board!(
                [(3, 3), Light], [(3, 4), Light], [(3, 5), Light]
            )
        );
    }

    #[test]
    fn test_place_err2() {
        let mut board = board!( 
            [(0, 0), Dark], [(1, 1), Dark], [(2, 2), Dark], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark], [(7, 7), Dark],
            [(6, 0), Dark], [(5, 1), Dark], [(4, 2), Dark], [(2, 4), Dark], [(1, 5), Dark], [(0, 6), Dark],
            [(0, 3), Dark], [(1, 3), Dark], [(2, 3), Dark], [(4, 3), Dark], [(5, 3), Dark], [(6, 3), Dark], [(7, 3), Dark],
            [(3, 0), Dark], [(3, 1), Dark], [(3, 2), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Dark], [(3, 7), Dark]
        );

        assert!(board.place(Position::new(3, 3).unwrap(), Light).is_err());

        assert_eq!(
            board, 
            board!( 
                [(0, 0), Dark], [(1, 1), Dark], [(2, 2), Dark], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark], [(7, 7), Dark],
                [(6, 0), Dark], [(5, 1), Dark], [(4, 2), Dark], [(2, 4), Dark], [(1, 5), Dark], [(0, 6), Dark],
                [(0, 3), Dark], [(1, 3), Dark], [(2, 3), Dark], [(4, 3), Dark], [(5, 3), Dark], [(6, 3), Dark], [(7, 3), Dark],
                [(3, 0), Dark], [(3, 1), Dark], [(3, 2), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Dark], [(3, 7), Dark]
            )
        );
    }

    #[test]
    fn test_place_err3() {
        let mut board = board!(
            [(5, 7), Dark]
        );

        assert!(board.place(Position::new(5, 6).unwrap(), Light).is_err());

        assert_eq!(
            board,
            board!(
                [(5, 7), Dark]
            )
        )
    }

    #[test]
    fn test_count_disks1() {
        let board = board!();

        assert_eq!(board.count_disks(&Dark), 0);
        assert_eq!(board.count_disks(&Light), 0);
    }

    #[test]
    fn test_count_disks2() {
        let board = board!(
            [(0, 0), Light], [(1, 1), Dark], [(2, 2), Dark], [(4, 4), Dark], [(5, 5), Dark], [(6, 6), Dark], [(7, 7), Light],
            [(6, 0), Light], [(5, 1), Dark], [(4, 2), Dark], [(2, 4), Dark], [(1, 5), Dark], [(0, 6), Light],
            [(0, 3), Light], [(1, 3), Dark], [(2, 3), Dark], [(4, 3), Dark], [(5, 3), Dark], [(6, 3), Dark], [(7, 3), Light],
            [(3, 0), Light], [(3, 1), Dark], [(3, 2), Dark], [(3, 4), Dark], [(3, 5), Dark], [(3, 6), Dark], [(3, 7), Light]
        );

        assert_eq!(board.count_disks(&Dark), 19);
        assert_eq!(board.count_disks(&Light), 8);
    }

    #[test]
    fn test_count_legal_movs1() {
        let board = board!();        

        assert_eq!(board.count_legal_movs(Dark), 0);
        assert_eq!(board.count_legal_movs(Light), 0);
    }

    #[test]
    fn test_count_legal_movs2() {
        let board = board!( 
            [(3, 3), Light], [(3, 4), Light], [(3, 5), Light], [(3, 6), Dark] 
        );

        assert_eq!(board.count_legal_movs(Dark), 1);
        assert_eq!(board.count_legal_movs(Light), 1);
    }

    #[test]
    fn test_count_legal_movs3() {
        let board = board!( 
            [(2, 3), Dark],
            [(3, 3), Dark], [(3, 4), Dark], 
            [(4, 3), Dark], [(4, 4), Light] 
        );

        assert_eq!(board.count_legal_movs(Dark), 3);
        assert_eq!(board.count_legal_movs(Light), 3);
    }
}