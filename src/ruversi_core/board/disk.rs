#[derive(PartialEq, Eq, Clone)]
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
