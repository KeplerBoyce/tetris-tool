use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref SECOND_PCS: Vec<PcSetup> = {
        let mut setups = vec![

            /*** Tub + Vert I ***/
            PcSetup::new(
                "Tub + Vert I",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(T, 21, 1, Cw),
                    Placement::place(S, 22, 3, Normal),
                    Placement::place(J, 21, 5, Ccw),
                ]),
            PcSetup::new(
                "Tub + Vert I",
                vec![
                    Placement::place(S, 22, 1, Normal),
                    Placement::place(I, 22, 3, Normal),
                    Placement::place(T, 20, 0, Cw),
                    Placement::place(L, 21, 4, Normal),
                ]),
            PcSetup::new(
                "Tub + Vert I",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(T, 21, 5, Ccw),
                ]),
            PcSetup::new(
                "Tub + Vert I",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(L, 21, 1, Cw),
                    Placement::place(T, 21, 3, Flip),
                    Placement::place(J, 21, 5, Ccw),
                ]),

            /*** Tub + Horiz I ***/
            PcSetup::new(
                "Tub + Horiz I",
                vec![
                    Placement::place(T, 21, 0, Cw),
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(J, 21, 4, Ccw),
                    Placement::place(I, 22, 7, Normal),
                ]),
            PcSetup::new(
                "Tub + Horiz I",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(T, 21, 2, Flip),
                    Placement::place(J, 21, 4, Ccw),
                    Placement::place(I, 22, 7, Normal),
                ]),

            /*** Broken Tub ***/
            PcSetup::new(
                "Broken Tub",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(I, 22, 5, Normal),
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(T, 20, 9, Ccw),
                ]),

            /*** Tub + O ***/
            PcSetup::new(
                "Tub + O",
                vec![
                    Placement::place(T, 21, 0, Cw),
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(J, 21, 4, Ccw),
                    Placement::place(O, 21, 8, Normal),
                ]),
            PcSetup::new(
                "Tub + O",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(T, 21, 2, Flip),
                    Placement::place(J, 21, 4, Ccw),
                    Placement::place(O, 21, 8, Normal),
                ]),

            /*** Heart + PCO ***/
            PcSetup::new(
                "Heart + PCO",
                vec![
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(T, 21, 8, Ccw),
                    Placement::place(I, 20, 9, Cw),
                ]),
            PcSetup::new(
                "Heart + PCO",
                vec![
                    Placement::place(T, 21, 0, Cw),
                    Placement::place(J, 20, 9, Ccw),
                    Placement::place(S, 21, 7, Normal),
                    Placement::place(I, 22, 7, Normal),
                ]),
            PcSetup::new(
                "Heart + PCO",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(Z, 21, 1, Normal),
                    Placement::place(T, 21, 8, Ccw),
                    Placement::place(I, 20, 9, Cw),
                ]),
            PcSetup::new(
                "Heart + PCO",
                vec![
                    Placement::place(S, 22, 1, Normal),
                    Placement::place(T, 20, 0, Cw),
                    Placement::place(J, 22, 8, Normal),
                    Placement::place(O, 20, 8, Normal),
                ]),
            PcSetup::new(
                "Heart + PCO",
                vec![
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(T, 20, 9, Ccw),
                ]),
            PcSetup::new(
                "Heart + PCO",
                vec![
                    Placement::place(S, 22, 1, Normal),
                    Placement::place(T, 20, 0, Cw),
                    Placement::place(J, 21, 9, Ccw),
                    Placement::place(Z, 21, 8, Ccw),
                ]),

            /*** Factory ***/
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(T, 21, 1, Cw),
                    Placement::place(J, 21, 3, Ccw),
                    Placement::place(O, 21, 8, Normal),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(T, 21, 3, Ccw),
                    Placement::place(O, 21, 8, Normal),
                ]),
            PcSetup::new(
                "Factory",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(L, 22, 4, Normal),
                    Placement::place(T, 21, 3, Normal),
                ]),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
