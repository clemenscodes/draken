mod quiet;

use quiet::QuietMove;

#[derive(Debug)]
pub enum ReversibleMove {
    Quiet(QuietMove),
}
