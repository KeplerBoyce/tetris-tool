use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, EnumIter)]
pub enum KeyAction {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    RotateCw,
    RotateCcw,
    Rotate180,
    Hold,
    Reset,
    Undo,
}
