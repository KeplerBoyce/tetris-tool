use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref FIFTH_PCS: Vec<PcSetup> = {
        let mut setups = vec![

            PcSetup::new(
                "IJ",
                vec![
                    Placement::place(I, 22, 7, Normal),
                    Placement::place(J, 21, 7, Normal),
                ]),
            PcSetup::new(
                "JO",
                vec![
                    Placement::place(J, 21, 7, Ccw),
                    Placement::place(O, 21, 8, Normal),
                ]),
            PcSetup::new(
                "IO",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(O, 21, 1, Normal),
                ]),
            PcSetup::new(
                "IS",
                vec![
                    Placement::place(I, 22, 3, Normal),
                    Placement::place(S, 22, 1, Normal),
                ]),
            PcSetup::new(
                "IT",
                vec![
                    Placement::place(I, 22, 6, Normal),
                    Placement::place(T, 22, 1, Normal),
                ]),
            PcSetup::new(
                "JT",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(T, 21, 9, Ccw),
                ]),
            PcSetup::new(
                "JL",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(L, 22, 8, Normal),
                ]),
            PcSetup::new(
                "JZ",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(Z, 22, 3, Normal),
                ]),
            PcSetup::new(
                "JS",
                vec![
                    Placement::place(J, 21, 9, Ccw),
                    Placement::place(S, 22, 7, Normal),
                ]),
            PcSetup::new(
                "OS",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(S, 22, 3, Normal),
                ]),
            PcSetup::new(
                "OT",
                vec![
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(T, 22, 1, Normal),
                ]),
            PcSetup::new(
                "ST",
                vec![
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(T, 22, 7, Normal),
                ]),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
