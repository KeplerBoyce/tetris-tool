use std::collections::{HashSet, HashMap, VecDeque};
use crate::state::{Board, Piece, Rotation};
use super::SearchState;

// Returns a list of (row, col, rotation)
pub fn get_locations(board: &Board, piece: Piece) -> HashMap<SearchState, u8> {
    // Queue of possible final placements
    let mut locations: HashMap<SearchState, u8> = HashMap::new();
    // Queue of (row, col, rotation) states in search tree
    let mut q: VecDeque<(SearchState, u8)> = VecDeque::from(
        vec![(SearchState::new(1, 4, Rotation::Normal, piece), 1)]
    );
    let mut visited: HashSet<SearchState> = HashSet::new();

    while let Some((state, i)) = q.pop_front() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);
        locations.insert(state.drop(board), i);
        for &successor in state.successors(board).iter() {
            if visited.contains(&successor) {
                continue;
            }
            q.push_back((successor, i + 1));
        }
    }
    locations
}
