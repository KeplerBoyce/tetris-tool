#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rotation {
    Normal,
    Cw,
    Ccw,
    Flip,
}
