#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShotStatus {
    Hit,
    Miss,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FieldStatus {
    Empty,
    Occupied(usize),
}

#[derive(Debug, Copy, Clone)]
pub struct Field {
    pub status: FieldStatus,
    pub shot: Option<ShotStatus>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            status: FieldStatus::Empty,
            shot: None,
        }
    }
}
