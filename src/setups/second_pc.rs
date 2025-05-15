use lazy_static::lazy_static;
use super::{PcSetup, add_mirrors};
use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation::*;

lazy_static! {
    pub static ref SECOND_PCS: Vec<PcSetup> = {
        let mut setups = vec![

            /*** Tub (save I) ***/
            PcSetup::new_with_save(
                "Tub (save I)",
                vec![
                    Placement::place(T, 21, 0, Cw),
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(J, 21, 4, Ccw),
                ],
                I,
            ),
            PcSetup::new_with_save(
                "Tub (save I)",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(T, 21, 2, Flip),
                    Placement::place(J, 21, 4, Ccw),
                ],
                I,
            ),

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

            /*** Tub (save O) ***/
            PcSetup::new_with_save(
                "Tub (save O)",
                vec![
                    Placement::place(T, 21, 0, Cw),
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(J, 21, 4, Ccw),
                ],
                O,
            ),
            PcSetup::new_with_save(
                "Tub (save O)",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(T, 21, 2, Flip),
                    Placement::place(J, 21, 4, Ccw),
                ],
                O,
            ),

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

            /*** LST (save J) ***/
            PcSetup::new_with_save(
                "LST (save J)",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(S, 22, 3, Normal),
                    Placement::place(T, 22, 5, Normal),
                ],
                J,
            ),

            /*** Hat + Mound ***/
            PcSetup::new(
                "Hat + Mound",
                vec![
                    Placement::place(I, 22, 2, Normal),
                    Placement::place(O, 20, 2, Normal),
                    Placement::place(S, 22, 6, Normal),
                    Placement::place(T, 22, 8, Normal),
                ]),
            PcSetup::new(
                "Hat + Mound",
                vec![
                    Placement::place(J, 21, 2, Ccw),
                    Placement::place(L, 21, 3, Cw),
                    Placement::place(S, 22, 6, Normal),
                    Placement::place(T, 22, 8, Normal),
                ]),

            /*** Hat + Broken Mound ***/
            PcSetup::new(
                "Hat + Broken Mound",
                vec![
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(T, 22, 5, Normal),
                    Placement::place(I, 22, 7, Normal),
                    Placement::place(O, 20, 7, Normal),
                ]),
            PcSetup::new(
                "Hat + Broken Mound",
                vec![
                    Placement::place(S, 22, 2, Normal),
                    Placement::place(T, 22, 5, Normal),
                    Placement::place(J, 21, 7, Ccw),
                    Placement::place(L, 21, 8, Cw),
                ]),

            /*** Mound + I (save O) ***/
            PcSetup::new_with_save(
                "Mound + I (save O)",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(S, 22, 4, Normal),
                    Placement::place(T, 22, 6, Normal),
                ],
                O,
            ),

            /*** STZ/SLT (save I) ***/
            PcSetup::new_with_save(
                "STZ/SLT (save I)",
                vec![
                    Placement::place(S, 22, 1, Normal),
                    Placement::place(T, 22, 3, Normal),
                    Placement::place(Z, 21, 4, Normal),
                ],
                I,
            ),
            PcSetup::new_with_save(
                "STZ/SLT (save I)",
                vec![
                    Placement::place(S, 22, 1, Normal),
                    Placement::place(L, 21, 3, Ccw),
                    Placement::place(T, 21, 4, Cw),
                ],
                I,
            ),
            
            /*** Hat ***/
            PcSetup::new(
                "Hat",
                vec![
                    Placement::place(T, 22, 1, Normal),
                    Placement::place(S, 22, 4, Normal),
                    Placement::place(Z, 21, 2, Normal),
                    Placement::place(I, 22, 6, Normal),
                ]),
            PcSetup::new(
                "Hat",
                vec![
                    Placement::place(J, 21, 1, Ccw),
                    Placement::place(T, 21, 2, Cw),
                    Placement::place(S, 21, 4, Normal),
                    Placement::place(I, 22, 6, Normal),
                ]),
            PcSetup::new(
                "Hat",
                vec![
                    Placement::place(J, 21, 1, Ccw),
                    Placement::place(L, 21, 2, Cw),
                    Placement::place(T, 21, 4, Flip),
                    Placement::place(I, 22, 6, Normal),
                ]),

            /*** LTJO ***/
            PcSetup::new(
                "LTJO",
                vec![
                    Placement::place(L, 21, 1, Cw),
                    Placement::place(T, 22, 3, Normal),
                    Placement::place(J, 21, 6, Ccw),
                    Placement::place(O, 21, 7, Normal),
                ]),

            /*** Mound + O (save Z) ***/
            PcSetup::new_with_save(
                "STZ/SLT (save I)",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(S, 22, 4, Normal),
                    Placement::place(T, 22, 6, Normal),
                ],
                Z,
            ),

            /*** Horiz Box (save I) ***/
            PcSetup::new_with_save(
                "Horiz Box (save I)",
                vec![
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(L, 21, 3, Ccw),
                ],
                I,
            ),
            PcSetup::new_with_save(
                "Horiz Box (save I)",
                vec![
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(L, 22, 2, Normal),
                    Placement::place(S, 21, 2, Normal),
                ],
                I,
            ),

            /*** OZS (save I) ***/
            PcSetup::new_with_save(
                "OZS (save I)",
                vec![
                    Placement::place(O, 21, 0, Normal),
                    Placement::place(Z, 21, 2, Cw),
                    Placement::place(S, 22, 4, Normal),
                ],
                I,
            ),

            /*** Shoe + L ***/
            PcSetup::new(
                "Shoe + L",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(L, 22, 8, Normal),
                ]),

            /*** Horiz Box (save O) ***/
            PcSetup::new_with_save(
                "Horiz Box (save O)",
                vec![
                    Placement::place(J, 21, 0, Cw),
                    Placement::place(L, 22, 2, Normal),
                    Placement::place(S, 21, 2, Normal),
                ],
                O,
            ),

            /*** Cactus (save J) ***/
            PcSetup::new_with_save(
                "Cactus (save J)",
                vec![
                    Placement::place(I, 21, 0, Ccw),
                    Placement::place(O, 21, 1, Normal),
                    Placement::place(Z, 21, 4, Ccw),
                ],
                J,
            ),

            /*** Heart + Butter Tower ***/
            PcSetup::new(
                "Heart + Butter Tower",
                vec![
                    Placement::place(I, 22, 6, Normal),
                    Placement::place(J, 21, 8, Flip),
                    Placement::place(S, 21, 6, Normal),
                    Placement::place(O, 19, 8, Normal),
                ]),
            PcSetup::new(
                "Heart + Butter Tower",
                vec![
                    Placement::place(I, 22, 7, Normal),
                    Placement::place(J, 21, 8, Normal),
                    Placement::place(Z, 21, 5, Cw),
                    Placement::place(O, 19, 8, Normal),
                ]),
            PcSetup::new(
                "Heart + Butter Tower",
                vec![
                    Placement::place(I, 22, 7, Normal),
                    Placement::place(L, 21, 8, Normal),
                    Placement::place(Z, 21, 5, Cw),
                    Placement::place(S, 20, 8, Normal),
                ]),

            /*** Shoe + O ***/
            PcSetup::new(
                "Shoe + O",
                vec![
                    Placement::place(J, 22, 1, Normal),
                    Placement::place(S, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(O, 21, 8, Normal),
                ]),
            PcSetup::new(
                "Shoe + O",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(L, 20, 0, Cw),
                    Placement::place(Z, 22, 3, Normal),
                    Placement::place(O, 21, 8, Normal),
                ]),

            /*** Split Sneakers ***/
            PcSetup::new(
                "Split Sneakers",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(J, 21, 1, Normal),
                    Placement::place(S, 21, 4, Ccw),
                    Placement::place(O, 21, 8, Normal),
                ]),
            PcSetup::new(
                "Split Sneakers",
                vec![
                    Placement::place(L, 21, 0, Cw),
                    Placement::place(Z, 22, 2, Normal),
                    Placement::place(S, 21, 3, Cw),
                    Placement::place(O, 21, 8, Normal),
                ]),

            /*** Sneakers ***/
            PcSetup::new(
                "Sneakers",
                vec![
                    Placement::place(I, 22, 3, Normal),
                    Placement::place(J, 21, 3, Normal),
                    Placement::place(S, 21, 6, Ccw),
                    Placement::place(O, 21, 0, Normal),
                ]),
            PcSetup::new(
                "Sneakers",
                vec![
                    Placement::place(L, 21, 2, Cw),
                    Placement::place(Z, 22, 4, Normal),
                    Placement::place(S, 21, 5, Cw),
                    Placement::place(O, 21, 0, Normal),
                ]),
            PcSetup::new(
                "Sneakers",
                vec![
                    Placement::place(I, 22, 1, Normal),
                    Placement::place(L, 21, 1, Normal),
                    Placement::place(Z, 22, 4, Normal),
                    Placement::place(S, 21, 5, Cw),
                ]),
        ];
        add_mirrors(&mut setups);
        setups
    };
}
