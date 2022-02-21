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
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Result<Position, &'static str> {
        if x < 0 || x >= 8 {
            return Err("x must be in 0 to 7");
        }

        if y < 0 || y >= 8 {
            return Err("y must be in 0 to 7");
        }

        Ok(Position{x, y})
    }

    fn next(&self, dir: &Direction) -> Result<Self, &'static str> {
        let (dx, dy) = dir.tuple();
        Self::new(self.x + dx, self.y + dy)
    }
}


struct Board {
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
    fn line_iter<'a>(&'a self, pos: Position, dir: Direction) -> BoardLineIter<'a> {
        BoardLineIter::new(self, pos, dir)
    }

    fn line_iter_mut<'a>(&'a mut self, pos: Position, dir: Direction) -> BoardLineIterMut<'a> {
        BoardLineIterMut::new(self, pos, dir)
    }

    pub fn count_turn_disks(&self, pos: Position, disk: Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disks already in the position.")
        }

        Ok(Direction::iter()
        .map(|dir| self.count_line_disks_sandwitched_by_another_colors(pos.clone(), dir, &disk).unwrap_or(0))
        .fold(0, |count, line_count| {
            count + line_count
        }))
    }

    fn count_line_disks_sandwitched_by_another_colors(&self, pos: Position, dir: Direction, end_disk: &Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disk already.");
        }

        let mut iter = self.line_iter(pos, dir);
        match iter.next() {
            Some(disk) if disk != end_disk => {
                Self::count_line_disks_end_another_color(iter, end_disk)
            },
            _ => {
                Err("")
            },
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
        .fold(0, |count, line_count| {
            count + line_count
        }))
    }

    fn turn_line_disks_sandwitched_by_another_colors(&mut self, pos: Position, dir: Direction, end_disk: &Disk) -> Result<i32, &'static str> {
        if !self.is_empty(&pos) {
            return Err("Exists disk already.");
        }

        let mut iter = self.line_iter_mut(pos, dir);
        match iter.next() {
            Some(disk) if disk != end_disk => {
                Self::turn_line_disks_end_another_color(iter, end_disk)
            },
            _ => {
                Err("")
            },
        }
    }

    fn turn_line_disks_end_another_color(mut iter: BoardLineIterMut, end_disk: &Disk) -> Result<i32, &'static str> {
        match iter.next() {
            Some(disk) => 
                if disk == end_disk { Ok(0) } 
                else { Self::turn_line_disks_end_another_color(iter, end_disk).map(|c| c + 1) },
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
        .and_then(|c| {
            let idx = Self::get_index(&pos);
            self.disks[idx] = Some(disk);
            Ok(c)
        })
    }
}
