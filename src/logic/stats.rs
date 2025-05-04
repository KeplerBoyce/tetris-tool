#[derive(Clone, Copy)]
pub struct Stats {
    pub inputs: u32,
    pub pieces: u32,
    pub lines: u32,
    pub faults: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            inputs: 0,
            pieces: 0,
            lines: 0,
            faults: 0,
        }
    }
}
