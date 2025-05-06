use crate::state::Board;
use super::PcState;

#[derive(Clone, Debug)]
pub struct Pc {
    board: Board,
}

impl Pc {
    pub fn from(state: &PcState) -> Self {
        Self {
            board: state.board,
        }
    }
}
