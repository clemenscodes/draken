mod double;
mod single;

use double::DoublePushMove;
use single::SinglePushMove;

#[derive(Debug)]
pub enum PushMove {
    Single(SinglePushMove),
    Double(DoublePushMove),
}
