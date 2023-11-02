#[derive(Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub len: usize,
}

impl Location {
    pub fn new() -> Location {
        Location {
            line: 0,
            column: 0,
            len: 0,
        }
    }
}
