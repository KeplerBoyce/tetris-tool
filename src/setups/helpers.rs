use crate::search::Placement;
use crate::state::Piece::*;
use crate::state::Rotation;
use super::PcSetup;

pub fn add_mirrors(setups: &mut Vec<PcSetup>) {
    let mut mirrors: Vec<PcSetup> = Vec::new();

    for setup in setups.iter() {
        let new_placements = setup.placements.iter().map(|p| {

            if let Placement::Place { piece, row, col, rotation } = p {
                // Mirroring horizontally, so normal and 180 flip remain the same
                let mirror_rotation = match rotation {
                    Rotation::Normal => Rotation::Normal,
                    Rotation::Cw => Rotation::Ccw,
                    Rotation::Ccw => Rotation::Cw,
                    Rotation::Flip => Rotation::Flip,
                };
                // Different setup for the I piece due to offset center
                let i_rotation = match rotation {
                    Rotation::Normal => Rotation::Flip,
                    Rotation::Cw => Rotation::Cw,
                    Rotation::Ccw => Rotation::Ccw,
                    Rotation::Flip => Rotation::Normal,
                };
                // O gets shifted one tile to the left since it can't be rotated
                match piece {
                    I => Placement::place(I, *row, 9 - *col, i_rotation),
                    J => Placement::place(L, *row, 9 - *col, mirror_rotation),
                    L => Placement::place(J, *row, 9 - *col, mirror_rotation),
                    O => Placement::place(O, *row, 8 - *col, *rotation),
                    S => Placement::place(Z, *row, 9 - *col, mirror_rotation),
                    T => Placement::place(T, *row, 9 - *col, mirror_rotation),
                    Z => Placement::place(S, *row, 9 - *col, mirror_rotation),
                }
            } else {
                // Shouldn't ever happen but this will satisfy the compiler
                Placement::Hold
            }
        }).collect();
        mirrors.push(PcSetup::new(&format!("{}*", setup.name), new_placements));
    }
    setups.append(&mut mirrors);
}
