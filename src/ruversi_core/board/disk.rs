enum Disk {
    Light,    
    Dark,
}

impl Disk {
    fn reverse(&mut self) {
        match self {
            Self::Light => *self = Self::Dark,
            Self::Dark => *self = Self::Light,
        }
    }
}
