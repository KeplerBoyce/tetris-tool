use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref FIRST_PCS: Vec<PcSetup> = {
        let mut setups = vec![

            /*** PCO (save I) ***/
            PcSetup::new_with_save(
                "PCO (save I)",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(L, 19, 1, Flip),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                ],
                I,
            ),
            PcSetup::new_with_save(
                "PCO (save I)",
                vec![
                    Placement::place(L, 22, 1, Normal),
                    Placement::place(O, 20, 0, Normal),
                    Placement::place(J, 19, 1, Flip),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                ],
                I,
            ),

            /*** PCO ***/
            PcSetup::new(
                "PCO",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(L, 19, 1, Flip),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                    Placement::place(I, 21, 3, Ccw),
                ]),
            PcSetup::new(
                "PCO",
                vec![
                    Placement::place(L, 22, 1, Normal),
                    Placement::place(O, 20, 0, Normal),
                    Placement::place(J, 19, 1, Flip),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                    Placement::place(I, 21, 3, Ccw),
                ]),
            PcSetup::new(
                "PCO",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(L, 20, 0, Cw),
                    Placement::place(J, 20, 3, Ccw),
                    Placement::place(O, 19, 1, Normal),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                ]),
            PcSetup::new(
                "PCO",
                vec![
                    Placement::place(I, 19, 1, Normal),
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(J, 21, 3, Ccw),
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                ]),
            PcSetup::new(
                "PCO",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(O, 20, 1, Normal),
                    Placement::place(J, 20, 0, Cw),
                    Placement::place(L, 20, 3, Ccw),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                ]),
            PcSetup::new(
                "PCO",
                vec![
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(L, 21, 3, Ccw),
                    Placement::place(I, 19, 1, Normal),
                    Placement::place(Z, 22, 7, Normal),
                    Placement::place(T, 21, 9, Ccw),
                    Placement::place(S, 20, 8, Normal),
                ]),

            /*** Jigsaw Jaws Variant ***/
            PcSetup::new_with_save(
                "Jigsaw Jaws",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(Z, 21, 1, Cw),
                    Placement::place(S, 22, 3, Normal),
                    Placement::place(L, 19, 2, Flip),
                    Placement::place(O, 21, 5, Normal),
                    Placement::place(J, 21, 8, Flip),
                ],
                T,
            ),

            /*** Jigsaw PCO Variant ***/
            PcSetup::new_with_save(
                "Jigsaw PCO",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(S, 22, 3, Normal),
                    Placement::place(T, 21, 1, Cw),
                    Placement::place(Z, 20, 2, Normal),
                    Placement::place(O, 21, 5, Normal),
                    Placement::place(J, 21, 8, Flip),
                ],
                L,
            ),

            /*** Grace System ***/
            PcSetup::new_with_save(
                "Grace System",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(L, 22, 4, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(Z, 20, 5, Ccw),
                    Placement::place(O, 20, 2, Normal),
                    Placement::place(I, 19, 2, Normal),
                ],
                T,
            ),
            PcSetup::new_with_save(
                "Grace System",
                vec![
                    Placement::place(I, 22, 2, Normal),
                    Placement::place(O, 20, 2, Normal),
                    Placement::place(Z, 21, 1, Ccw),
                    Placement::place(S, 21, 4, Cw),
                    Placement::place(L, 19, 1, Flip),
                    Placement::place(J, 19, 4, Flip),
                ],
                T,
            ),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
