use api::Square;

#[derive(Debug)]
pub struct Coordinates {
    source: Square,
    destination: Square,
}

impl Coordinates {
    pub fn new(source: Square, destination: Square) -> Self {
        Self { source, destination }
    }

    pub fn source(&self) -> Square {
        self.source
    }

    pub fn destination(&self) -> Square {
        self.destination
    }
}
