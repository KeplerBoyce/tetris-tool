#[derive(Clone, Copy, Debug)]
pub enum Movement {
    Left,
    DasLeft,
    Right,
    DasRight,
    SoftDrop,
    HardDrop,
    RotateCw,
    RotateCcw,
    Rotate180,
}
