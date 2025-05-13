use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref SECOND_PCS: Vec<PcSetup> = {
        let mut setups = vec![

            /*** Tub ***/
            PcSetup::new(
                "Tub",
                vec![
                    Placement::place(I, 22, 5, Normal),
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(J, 21, 5, Normal),
                    Placement::place(T, 20, 9, Ccw),
                ]),
            PcSetup::new(
                "Tub",
                vec![
                    Placement::place(I, 20, 9, Cw),
                    Placement::place(J, 21, 8, Ccw),
                    Placement::place(S, 22, 6, Normal),
                    Placement::place(T, 21, 4, Cw),
                ]),
            PcSetup::new(
                "Tub",
                vec![
                    Placement::place(I, 20, 9, Cw),
                    Placement::place(J, 21, 8, Ccw),
                    Placement::place(L, 21, 4, Cw),
                    Placement::place(T, 21, 6, Flip),
                ]),
            PcSetup::new(
                "Tub",
                vec![
                    Placement::place(I, 20, 9, Cw),
                    Placement::place(T, 21, 8, Ccw),
                    Placement::place(Z, 22, 6, Normal),
                    Placement::place(L, 21, 4, Cw),
                ]),
            PcSetup::new(
                "Tub",
                vec![
                    Placement::place(L, 22, 8, Normal),
                    Placement::place(Z, 20, 9, Ccw),
                    Placement::place(S, 22, 6, Normal),
                    Placement::place(T, 21, 4, Cw),
                ]),

            /*** Factory ***/
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(I, 22, 7, Normal),
                    Placement::place(J, 20, 9, Ccw),
                    Placement::place(L, 21, 4, Cw),
                    Placement::place(T, 21, 6, Normal),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(I, 22, 7, Normal),
                    Placement::place(J, 21, 8, Normal),
                    Placement::place(S, 22, 5, Normal),
                    Placement::place(T, 20, 4, Cw),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(I, 22, 6, Normal),
                    Placement::place(J, 21, 8, Flip),
                    Placement::place(Z, 20, 7, Ccw),
                    Placement::place(T, 21, 4, Cw),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(L, 22, 8, Normal),
                    Placement::place(J, 22, 5, Normal),
                    Placement::place(S, 20, 4, Cw),
                    Placement::place(T, 21, 7, Normal),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(J, 21, 7, Ccw),
                    Placement::place(S, 22, 5, Normal),
                    Placement::place(T, 20, 4, Cw),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(T, 21, 7, Ccw),
                    Placement::place(J, 22, 5, Normal),
                    Placement::place(S, 20, 4, Cw),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(Z, 22, 6, Normal),
                    Placement::place(L, 21, 4, Cw),
                    Placement::place(T, 20, 7, Ccw),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(L, 22, 6, Normal),
                    Placement::place(T, 21, 4, Cw),
                    Placement::place(Z, 20, 7, Ccw),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(T, 21, 7, Ccw),
                    Placement::place(L, 21, 5, Cw),
                    Placement::place(I, 21, 4, Ccw),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(J, 21, 7, Ccw),
                    Placement::place(T, 21, 5, Cw),
                    Placement::place(I, 21, 4, Ccw),
                ]),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
