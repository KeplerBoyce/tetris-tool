use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref THIRD_PCS: Vec<PcSetup> = {
        let mut setups = vec![

            /*** OO Jaws ***/
            PcSetup::new(
                "OO Jaws Base",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(I, 20, 2, Cw),
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(J, 19, 8, Flip),
                ]),
            PcSetup::new(
                "OO Jaws + Z",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(I, 20, 2, Cw),
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(J, 19, 8, Flip),
                    Placement::place(Z, 22, 7, Normal),
                ]),
            PcSetup::new(
                "OO Jaws + ZL",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(I, 20, 2, Cw),
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(J, 19, 8, Flip),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(L, 21, 4, Flip),
                ]),

            /*** OO Half Shoe ***/
            PcSetup::new(
                "OO Half Shoe",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(L, 22, 8, Normal),
                    Placement::place(Z, 20, 9, Ccw),
                ]),

            /*** II/OO/JJ IOSJ ***/
            PcSetup::new(
                "II/OO/JJ IOSJ",
                vec![
                    Placement::place(I, 22, 5, Normal),
                    Placement::place(O, 21, 8, Normal),
                    Placement::place(S, 21, 7, Normal),
                    Placement::place(J, 19, 8, Flip),
                ]),

            /*** II Jaws ***/
            PcSetup::new(
                "II Jaws",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(I, 21, 3, Normal),
                    Placement::place(L, 20, 0, Cw),
                    Placement::place(O, 19, 1, Normal),
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(J, 19, 8, Flip),
                    Placement::place(Z, 22, 7, Normal),
                ]),

            /*** ZZ Mound ***/
            PcSetup::new(
                "ZZ Mound Base",
                vec![
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(Z, 20, 7, Normal),
                    Placement::place(I, 22, 5, Normal),
                    Placement::place(O, 20, 5, Normal),
                    Placement::place(L, 20, 9, Ccw),
                ]),
            PcSetup::new(
                "ZZ Mound + S",
                vec![
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(Z, 20, 7, Normal),
                    Placement::place(I, 22, 5, Normal),
                    Placement::place(O, 20, 5, Normal),
                    Placement::place(L, 20, 9, Ccw),
                    Placement::place(S, 20, 4, Normal),
                ]),
            PcSetup::new(
                "ZZ Mound + ST",
                vec![
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(Z, 20, 7, Normal),
                    Placement::place(I, 22, 5, Normal),
                    Placement::place(O, 20, 5, Normal),
                    Placement::place(L, 20, 9, Ccw),
                    Placement::place(S, 20, 4, Normal),
                    Placement::place(T, 21, 3, Flip),
                ]),

            /*** ZZ PCO ***/
            PcSetup::new(
                "ZZ PCO",
                vec![
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(Z, 20, 7, Normal),
                    Placement::place(L, 20, 9, Ccw),
                    Placement::place(T, 22, 6, Normal),
                    Placement::place(S, 21, 5, Normal),
                ]),

            /*** ZZ Overhang ***/
            PcSetup::new(
                "ZZ Overhang Base",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(Z, 22, 2, Normal),
                    Placement::place(Z, 20, 1, Normal),
                    Placement::place(O, 20, 3, Normal),
                    Placement::place(I, 19, 3, Normal),
                ]),
            PcSetup::new(
                "ZZ Overhang + S",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(Z, 22, 2, Normal),
                    Placement::place(Z, 20, 1, Normal),
                    Placement::place(O, 20, 3, Normal),
                    Placement::place(I, 19, 3, Normal),
                    Placement::place(S, 22, 5, Normal),
                ]),
            PcSetup::new(
                "ZZ Overhang + SJ",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(Z, 22, 2, Normal),
                    Placement::place(Z, 20, 1, Normal),
                    Placement::place(O, 20, 3, Normal),
                    Placement::place(I, 19, 3, Normal),
                    Placement::place(S, 22, 5, Normal),
                    Placement::place(J, 21, 8, Flip),
                ]),

            /*** ZZ Shoe ***/
            PcSetup::new(
                "ZZ Shoe",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(Z, 20, 2, Normal),
                ]),

            /*** Grace System ***/
            PcSetup::new(
                "LL Grace System",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(L, 22, 4, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(Z, 20, 5, Ccw),
                    Placement::place(O, 20, 2, Normal),
                    Placement::place(I, 19, 2, Normal),
                ]),
            PcSetup::new(
                "LL Grace System",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(L, 22, 4, Normal),
                    Placement::place(S, 21, 1, Cw),
                    Placement::place(Z, 21, 3, Normal),
                    Placement::place(O, 19, 4, Normal),
                    Placement::place(I, 19, 1, Normal),
                ]),
            PcSetup::new(
                "LL Grace System",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(I, 22, 3, Normal),
                    Placement::place(Z, 21, 2, Normal),
                    Placement::place(S, 20, 3, Cw),
                    Placement::place(L, 19, 1, Flip),
                    Placement::place(L, 20, 5, Ccw),
                ]),

            /*** JJ Box + J ***/
            PcSetup::new(
                "JJ Box + J",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(J, 21, 9, Ccw),
                    Placement::place(Z, 21, 8, Ccw),
                    Placement::place(L, 19, 8, Flip),
                ]),

            /*** TT Inverse PCO ***/
            PcSetup::new(
                "TT Inverse PCO Base",
                vec![
                    Placement::place(T, 22, 1, Normal),
                    Placement::place(J, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(S, 20, 2, Normal),
                ]),
            PcSetup::new(
                "TT Inverse PCO + I",
                vec![
                    Placement::place(T, 22, 1, Normal),
                    Placement::place(J, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(S, 20, 2, Normal),
                    Placement::place(I, 20, 9, Cw),
                ]),
            PcSetup::new(
                "TT Inverse PCO + IO",
                vec![
                    Placement::place(T, 22, 1, Normal),
                    Placement::place(J, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(S, 20, 2, Normal),
                    Placement::place(I, 20, 9, Cw),
                    Placement::place(O, 21, 7, Normal),
                ]),
            PcSetup::new(
                "TT Inverse PCO + IOL",
                vec![
                    Placement::place(T, 22, 1, Normal),
                    Placement::place(J, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(S, 20, 2, Normal),
                    Placement::place(I, 20, 9, Cw),
                    Placement::place(O, 21, 7, Normal),
                    Placement::place(L, 20, 7, Normal),
                ]),

            /*** TT PCO ***/
            PcSetup::new(
                "TT PCO",
                vec![
                    Placement::place(T, 21, 0, Cw),
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(Z, 20, 1, Normal),
                    Placement::place(L, 22, 8, Normal),
                    Placement::place(O, 20, 7, Normal),
                    Placement::place(J, 19, 8, Flip),
                ]),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
