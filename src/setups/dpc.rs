use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref DPCS: Vec<PcSetup> = {
        let mut setups = vec![

            /*** O DPC ***/
            PcSetup::new_with_save(
                "O DPC 1",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(O, 20, 6, Normal),
                    Placement::place(Z, 19, 0, Cw),
                    Placement::place(S, 22, 3, Normal),
                    Placement::place(L, 21, 5, Ccw),
                    Placement::place(J, 21, 8, Ccw),
                    Placement::place(I, 20, 9, Cw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "O DPC 2",
                vec![
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(O, 21, 7, Normal),
                    Placement::place(Z, 19, 1, Cw),
                    Placement::place(S, 22, 4, Normal),
                    Placement::place(L, 21, 6, Ccw),
                    Placement::place(J, 20, 8, Flip),
                    Placement::place(I, 20, 0, Cw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "O DPC 3",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(I, 22, 4, Normal),
                    Placement::place(L, 21, 3, Flip),
                    Placement::place(Z, 20, 4, Normal),
                    Placement::place(S, 20, 5, Cw),
                    Placement::place(J, 21, 9, Ccw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "O DPC 3",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(I, 22, 4, Normal),
                    Placement::place(Z, 21, 2, Cw),
                    Placement::place(S, 21, 5, Normal),
                    Placement::place(L, 19, 5, Flip),
                    Placement::place(J, 21, 9, Ccw),
                ],
                T,
            ),

            /*** I DPC ***/
            PcSetup::new_with_save(
                "I DPC 1",
                vec![
                    Placement::place(I, 22, 3, Normal),
                    Placement::place(O, 20, 0, Normal),
                    Placement::place(L, 19, 0, Cw),
                    Placement::place(Z, 20, 2, Cw),
                    Placement::place(S, 21, 4, Normal),
                    Placement::place(J, 21, 8, Ccw),
                    Placement::place(I, 20, 9, Cw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "I DPC 2",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(I, 22, 6, Normal),
                    Placement::place(J, 21, 1, Normal),
                    Placement::place(Z, 20, 1, Normal),
                    Placement::place(O, 20, 6, Normal),
                    Placement::place(L, 19, 6, Flip),
                    Placement::place(S, 21, 9, Ccw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "I DPC 2",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(I, 22, 6, Normal),
                    Placement::place(L, 21, 1, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(S, 20, 6, Ccw),
                    Placement::place(J, 21, 8, Flip),
                    Placement::place(Z, 20, 7, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "I DPC 2",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(I, 22, 6, Normal),
                    Placement::place(L, 21, 1, Normal),
                    Placement::place(O, 19, 0, Normal),
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(Z, 21, 6, Normal),
                    Placement::place(J, 19, 6, Flip),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "I DPC 3",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(O, 20, 0, Normal),
                    Placement::place(Z, 20, 2, Cw),
                    Placement::place(S, 22, 6, Normal),
                    Placement::place(L, 22, 8, Normal),
                    Placement::place(J, 20, 7, Flip),
                    Placement::place(I, 19, 9, Ccw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "I DPC 4",
                vec![
                    Placement::place(I, 22, 3, Normal),
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(L, 21, 3, Normal),
                    Placement::place(I, 20, 1, Normal),
                    Placement::place(S, 21, 5, Cw),
                    Placement::place(Z, 21, 9, Ccw),
                    Placement::place(J, 19, 5, Normal),
                ],
                T,
            ),

            /*** S/Z DPC ***/
            PcSetup::new_with_save(
                "S/Z DPC 1",
                vec![
                    Placement::place(S, 21, 0, Cw),
                    Placement::place(L, 21, 2, Ccw),
                    Placement::place(J, 21, 3, Cw),
                    Placement::place(Z, 22, 5, Normal),
                    Placement::place(O, 21, 7, Normal),
                    Placement::place(S, 19, 8, Ccw),
                    Placement::place(I, 21, 9, Ccw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "S/Z DPC 2 (Lime)",
                vec![
                    Placement::place(S, 22, 5, Normal),
                    Placement::place(S, 21, 4, Normal),
                    Placement::place(O, 18, 3, Normal),
                    Placement::place(Z, 21, 7, Normal),
                    Placement::place(L, 21, 9, Ccw),
                    Placement::place(J, 22, 2, Normal),
                    Placement::place(I, 21, 0, Ccw),
                ],
                T,
            ),

            /*** J/L DPC ***/
            PcSetup::new_with_save(
                "J/L DPC 1",
                vec![
                    Placement::place(J, 22, 8, Normal),
                    Placement::place(L, 20, 8, Ccw),
                    Placement::place(I, 20, 9, Ccw),
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(Z, 22, 5, Normal),
                    Placement::place(S, 21, 3, Normal),
                    Placement::place(J, 20, 1, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "J/L DPC 2",
                vec![
                    Placement::place(J, 22, 8, Normal),
                    Placement::place(L, 20, 8, Ccw),
                    Placement::place(I, 20, 9, Ccw),
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(S, 21, 2, Cw),
                    Placement::place(Z, 21, 4, Normal),
                    Placement::place(J, 19, 1, Ccw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "J/L DPC 3",
                vec![
                    Placement::place(J, 22, 4, Normal),
                    Placement::place(J, 22, 7, Normal),
                    Placement::place(I, 21, 9, Ccw),
                    Placement::place(O, 20, 8, Normal),
                    Placement::place(L, 20, 4, Ccw),
                    Placement::place(S, 21, 0, Cw),
                    Placement::place(Z, 20, 5, Cw),
                ],
                T,
            ),

            /*** T DPC ***/
            PcSetup::new_with_save(
                "T DPC 1",
                vec![
                    Placement::place(T, 22, 4, Normal),
                    Placement::place(J, 21, 8, Ccw),
                    Placement::place(I, 21, 9, Ccw),
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(Z, 21, 2, Cw),
                    Placement::place(S, 20, 2, Normal),
                    Placement::place(L, 20, 4, Cw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "T DPC 1",
                vec![
                    Placement::place(T, 22, 4, Normal),
                    Placement::place(J, 21, 8, Ccw),
                    Placement::place(I, 21, 9, Ccw),
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(Z, 21, 0, Cw),
                    Placement::place(O, 19, 2, Normal),
                    Placement::place(L, 20, 4, Cw),
                ],
                T,
            ),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
