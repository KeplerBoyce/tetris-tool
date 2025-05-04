use std::collections::{HashSet, HashMap, VecDeque};
use crate::state::{Board, Piece, Rotation};
use super::SearchState;

// Returns a map of all possible final locations to the number of moves they take
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
        // Add this position if it hasn't been found already
        let dropped = state.drop(board);
        if !locations.contains_key(&dropped) {
            locations.insert(dropped, i);
        }
        for &successor in state.successors(board).iter() {
            if visited.contains(&successor) {
                continue;
            }
            q.push_back((successor, i + 1));
        }
    }
    locations
}

// Returns minimum number of moves for a certain placement
pub fn get_finesse_faults(board: &Board, piece: Piece, moves: u8, row: u8, col: u8, rotation: Rotation) -> u8 {
    let location_map = get_locations(board, piece);
    let target_state = SearchState::new(row as i8, col as i8, rotation, piece);
    match location_map.get(&target_state) {
        Some(&num_moves) => {
            println!("{}", num_moves);
            (moves as i8 - num_moves as i8).max(0) as u8
        },
        None => 0,
    }
}
