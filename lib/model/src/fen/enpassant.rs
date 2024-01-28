use api::Square;

#[derive(Debug)]
pub struct EnPassant {
    square: Option<Square>,
}

impl EnPassant {
    pub fn new(square: Option<Square>) -> Self {
        Self { square }
    }

    pub fn square(&self) -> Option<Square> {
        self.square
    }
}
