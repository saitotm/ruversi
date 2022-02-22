#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Disk {
    Light,    
    Dark,
}

impl Disk {
    pub fn reverse(&mut self) {
        match self {
            Self::Light => *self = Self::Dark,
            Self::Dark => *self = Self::Light,
        }
    }
}
