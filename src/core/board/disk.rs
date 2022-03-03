use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Disk {
    Light,
    Dark,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Light => write!(f, "o"),
            Self::Dark => write!(f, "x"),
        }
    }
}

impl Disk {
    pub fn reverse(&mut self) {
        match self {
            Self::Light => *self = Self::Dark,
            Self::Dark => *self = Self::Light,
        }
    }
}
