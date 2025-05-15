use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref FOURTH_PCS: Vec<PcSetup> = {
        let mut setups = vec![

            /*** No IL ***/
            PcSetup::new_with_save(
                "No IL (save J)",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(S, 20, 8, Normal),
                ],
                J,
            ),
            PcSetup::new_with_save(
                "No IL (save T)",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(J, 19, 8, Flip),
                ],
                T,
            ),

            /*** No IO ***/
            PcSetup::new_with_save(
                "No IO (save T)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(L, 22, 8, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No IO (save T)",
                vec![
                    Placement::place(Z, 21, 0, Cw),
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(L, 19, 1, Flip),
                    Placement::place(J, 21, 9, Ccw),
                ],
                T,
            ),

            /*** No IT ***/
            PcSetup::new_with_save(
                "No IT (save Z)",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(S, 21, 1, Cw),
                    Placement::place(J, 19, 1, Flip),
                    Placement::place(O, 21, 3, Normal),
                ],
                Z,
            ),
            PcSetup::new_with_save(
                "No IT (save Z)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(L, 20, 2, Ccw),
                    Placement::place(O, 21, 3, Normal),
                ],
                Z,
            ),
            PcSetup::new_with_save(
                "No IT (save Z)",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(S, 21, 1, Cw),
                    Placement::place(J, 19, 1, Flip),
                    Placement::place(O, 21, 8, Normal),
                ],
                Z,
            ),
            PcSetup::new_with_save(
                "No IT (save Z)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(L, 20, 2, Ccw),
                    Placement::place(O, 21, 8, Normal),
                ],
                Z,
            ),

            /*** No IZ ***/
            PcSetup::new_with_save(
                "No IZ (save J)",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(T, 22, 8, Normal),
                    Placement::place(S, 21, 7, Normal),
                    Placement::place(L, 20, 9, Ccw),
                ],
                J,
            ),
            PcSetup::new_with_save(
                "No IZ (save L)",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(J, 21, 9, Ccw),
                    Placement::place(T, 21, 7, Flip),
                    Placement::place(S, 20, 8, Normal),
                ],
                L,
            ),
            PcSetup::new_with_save(
                "No IZ (save T)",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(S, 21, 1, Cw),
                    Placement::place(J, 19, 1, Flip),
                    Placement::place(O, 21, 3, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No IZ (save T)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(L, 20, 2, Ccw),
                    Placement::place(O, 21, 3, Normal),
                ],
                T,
            ),

            /*** No JL ***/
            PcSetup::new_with_save(
                "No JL (save T)",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(I, 22, 7, Normal),
                    Placement::place(S, 22, 5, Normal),
                    Placement::place(Z, 20, 6, Normal),
                ],
                T,
            ),

            /*** No LO ***/
            PcSetup::new_with_save(
                "No LO (save T)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(J, 22, 2, Normal),
                    Placement::place(S, 20, 1, Cw),
                    Placement::place(Z, 22, 4, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No LO (save T)",
                vec![
                    Placement::place(I, 20, 9, Cw),
                    Placement::place(S, 21, 8, Ccw),
                    Placement::place(Z, 22, 6, Normal),
                    Placement::place(J, 19, 7, Flip),
                ],
                T,
            ),

            /*** No LS ***/
            PcSetup::new_with_save(
                "No LS (save J)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(Z, 22, 8, Normal),
                    Placement::place(T, 20, 9, Ccw),
                ],
                J,
            ),
            PcSetup::new_with_save(
                "No LS (save J)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                ],
                J,
            ),

            /*** No LT ***/
            PcSetup::new_with_save(
                "No LT (save I)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(O, 21, 8, Normal),
                ],
                I,
            ),
            PcSetup::new_with_save(
                "No LT (save O)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(S, 21, 9, Ccw),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(J, 19, 8, Flip),
                ],
                O,
            ),

            /*** No LZ ***/
            PcSetup::new_with_save(
                "No LZ (save J)",
                vec![
                    Placement::place(S, 22, 1, Normal),
                    Placement::place(T, 20, 0, Cw),
                    Placement::place(O, 21, 7, Normal),
                    Placement::place(I, 20, 9, Cw),
                ],
                J,
            ),
            PcSetup::new_with_save(
                "No LZ (save J)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(O, 21, 7, Normal),
                    Placement::place(I, 20, 9, Cw),
                ],
                T,
            ),

            /*** No OT ***/
            PcSetup::new_with_save(
                "No OT (save Z)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(L, 21, 1, Cw),
                    Placement::place(S, 21, 2, Cw),
                    Placement::place(J, 19, 2, Flip),
                ],
                Z,
            ),
            PcSetup::new_with_save(
                "No OT (save Z)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(J, 22, 2, Normal),
                    Placement::place(S, 20, 1, Cw),
                    Placement::place(L, 20, 3, Ccw),
                ],
                Z,
            ),
            PcSetup::new_with_save(
                "No OT (save Z)",
                vec![
                    Placement::place(L, 22, 2, Normal),
                    Placement::place(S, 21, 2, Normal),
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(I, 19, 1, Normal),
                ],
                Z,
            ),
            PcSetup::new_with_save(
                "No OT (save Z)",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(J, 20, 3, Ccw),
                    Placement::place(S, 21, 1, Normal),
                    Placement::place(L, 19, 1, Flip),
                ],
                Z,
            ),

            /*** No OZ ***/
            PcSetup::new_with_save(
                "No OZ (save T)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(L, 21, 1, Cw),
                    Placement::place(S, 21, 2, Cw),
                    Placement::place(J, 19, 2, Flip),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No OZ (save T)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(J, 22, 2, Normal),
                    Placement::place(S, 20, 1, Cw),
                    Placement::place(L, 20, 3, Ccw),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No OZ (save T)",
                vec![
                    Placement::place(L, 22, 2, Normal),
                    Placement::place(S, 21, 2, Normal),
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(I, 19, 1, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No OZ (save T)",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(J, 20, 3, Ccw),
                    Placement::place(S, 21, 1, Normal),
                    Placement::place(L, 19, 1, Flip),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No OZ (save T)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(L, 22, 7, Normal),
                    Placement::place(I, 20, 9, Cw),
                ],
                T,
            ),

            /*** No SZ ***/
            PcSetup::new_with_save(
                "No SZ (save T)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(J, 22, 2, Normal),
                    Placement::place(O, 20, 2, Normal),
                    Placement::place(L, 19, 2, Flip),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No SZ (save T)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(L, 22, 2, Normal),
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(J, 19, 2, Flip),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No SZ (save T)",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(L, 20, 0, Cw),
                    Placement::place(J, 20, 3, Ccw),
                    Placement::place(O, 19, 1, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No SZ (save T)",
                vec![
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(J, 20, 0, Cw),
                    Placement::place(L, 20, 3, Ccw),
                    Placement::place(I, 19, 1, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "No SZ (save T)",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(I, 20, 9, Cw),
                    Placement::place(O, 21, 7, Normal),
                    Placement::place(J, 19, 7, Flip),
                ],
                T,
            ),

            /*** No TZ ***/
            PcSetup::new_with_save(
                "No TZ (save O)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(L, 21, 1, Cw),
                    Placement::place(S, 21, 2, Cw),
                    Placement::place(J, 19, 2, Flip),
                ],
                O,
            ),
            PcSetup::new_with_save(
                "No TZ (save O)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(J, 22, 2, Normal),
                    Placement::place(S, 20, 1, Cw),
                    Placement::place(L, 20, 3, Ccw),
                ],
                O,
            ),
            PcSetup::new_with_save(
                "No TZ (save O)",
                vec![
                    Placement::place(L, 22, 2, Normal),
                    Placement::place(S, 21, 2, Normal),
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(I, 19, 1, Normal),
                ],
                O,
            ),
            PcSetup::new_with_save(
                "No TZ (save O)",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(J, 20, 3, Ccw),
                    Placement::place(S, 21, 1, Normal),
                    Placement::place(L, 19, 1, Flip),
                ],
                O,
            ),
            PcSetup::new_with_save(
                "No TZ (save S)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(J, 22, 2, Normal),
                    Placement::place(O, 20, 2, Normal),
                    Placement::place(L, 19, 2, Flip),
                ],
                S,
            ),
            PcSetup::new_with_save(
                "No TZ (save S)",
                vec![
                    Placement::place(I, 20, 0, Cw),
                    Placement::place(L, 22, 2, Normal),
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(J, 19, 2, Flip),
                ],
                S,
            ),
            PcSetup::new_with_save(
                "No TZ (save S)",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(L, 20, 0, Cw),
                    Placement::place(J, 20, 3, Ccw),
                    Placement::place(O, 19, 1, Normal),
                ],
                S,
            ),
            PcSetup::new_with_save(
                "No TZ (save S)",
                vec![
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(J, 20, 0, Cw),
                    Placement::place(L, 20, 3, Ccw),
                    Placement::place(I, 19, 1, Normal),
                ],
                S,
            ),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
