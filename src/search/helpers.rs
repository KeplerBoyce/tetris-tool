use std::collections::{HashSet, HashMap, VecDeque};
use crate::state::{Board, Piece, Rotation};
use super::{Movement, SearchState};

// Returns a map of all possible final locations to the number of moves they take
pub fn get_locations(board: &Board, piece: Piece) -> HashMap<SearchState, Vec<Movement>> {
    // Queue of possible final placements
    let mut locations: HashMap<SearchState, Vec<Movement>> = HashMap::new();
    // Queue of (state, prev_nodes index) states in search tree
    let mut q: VecDeque<(SearchState, usize)> = VecDeque::from(
        vec![(SearchState::new(1, 4, Rotation::Normal, piece), 0)]
    );
    let mut visited: HashSet<SearchState> = HashSet::new();
    // List of all already-visited nodes for reconstructing optimal paths at the end
    // Holds tuples of (action taken to get to this state, index of parent state)
    let mut prev_nodes: Vec<(Option<Movement>, usize)> = vec![(None, 0)];

    while let Some((state, index)) = q.pop_front() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);
        // Add this position if it hasn't been found already
        let dropped = state.drop(board);
        if !locations.contains_key(&dropped) {
            let mut path: VecDeque<Movement> = VecDeque::from(vec![Movement::HardDrop]);
            let mut curr_index = index;
            loop {
                let (action_option, prev_index) = prev_nodes[curr_index];
                if let Some(action) = action_option {
                    path.push_front(action);
                    curr_index = prev_index;
                } else {
                    break;
                }
            }
            locations.insert(dropped, Vec::from(path));
        }
        for &(successor, action) in state.successors(board).iter() {
            if visited.contains(&successor) {
                continue;
            }
            prev_nodes.push((Some(action), index));
            q.push_back((successor, prev_nodes.len() - 1));
        }
    }
    locations
}

// Returns minimum number of moves for a certain placement
pub fn get_finesse_faults(board: &Board, piece: Piece, moves: u8, row: u8, col: u8, rotation: Rotation) -> u8 {
    let location_map = get_locations(board, piece);
    let target_state = SearchState::new(row as i8, col as i8, rotation, piece);
    match location_map.get(&target_state) {
        Some(path) => {
            println!("{:?}", path);
            (moves as i8 - path.len() as i8).max(0) as u8
        },
        None => 0,
    }
}
