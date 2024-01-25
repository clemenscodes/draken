pub trait Model {}

#[derive(Debug, Clone)]
pub struct ChessModel {}

impl ChessModel {
    pub fn new() -> Self {
        ChessModel {}
    }
}

impl Model for ChessModel {}
