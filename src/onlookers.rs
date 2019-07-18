pub struct Public {
    pub prime: u64,
    pub root: u64,
    pub amy: Option<u64>,
    pub ben: Option<u64>,
}

impl Public {
    pub fn amy(&mut self, value: u64) {
        self.amy = Some(value);
    }

    pub fn ben(&mut self, value: u64) {
        self.ben = Some(value);
    }
}
