use std::fmt::{Debug, Display};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    #[default]
    Start,
    Playing,
    Checkmate,
    Stalemate,
    Resignation,
    Draw,
    DrawOffer,
    Promotion,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}
