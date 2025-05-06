use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use crossbeam_channel::Sender;
use crate::state::{Board, Game, Piece, Rotation};
use super::{Movement, Pc, PcState, SearchState};

// Returns a set of all possible final locations
pub fn get_locations(board: &Board, piece: Piece) -> HashSet<SearchState> {
    // Set of possible final placements
    let mut locations: HashSet<SearchState> = HashSet::new();
    // Queue of states in search tree
    let mut q: VecDeque<SearchState> = VecDeque::from(
        vec![SearchState::new(1, 4, Rotation::Normal, piece)]
    );
    let mut visited: HashSet<SearchState> = HashSet::new();

    while let Some(state) = q.pop_front() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);
        // Add this position if it hasn't been found already
        let dropped = state.drop(board);
        if !locations.contains(&dropped) && !locations.contains(&dropped.symmetrical()) {
            locations.insert(dropped);
        }
        for &(successor, _) in state.successors(board).iter() {
            if visited.contains(&successor) {
                continue;
            }
            q.push_back(successor);
        }
    }
    locations
}

// Returns a map of all possible final locations to the optimal sequence of moves to place it there
pub fn get_locations_with_finesse(board: &Board, piece: Piece) -> HashMap<SearchState, Vec<Movement>> {
    // Map of possible final placements
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
            locations.insert(dropped, Vec::from(path.clone()));
            locations.insert(dropped.symmetrical(), Vec::from(path));
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
pub fn get_finesse_faults(
    board: &Board,
    piece: Piece,
    moves: u8,
    row: u8,
    col: u8,
    rotation: Rotation,
) -> (u8, Option<Vec<Movement>>) {
    let location_map = get_locations_with_finesse(board, piece);
    let target_state = SearchState::new(row as i8, col as i8, rotation, piece);
    match location_map.get(&target_state) {
        Some(path) => {
            let num_faults = (moves as i8 - path.len() as i8).max(0) as u8;
            (num_faults, if num_faults > 0 {
                Some(path.to_vec())
            } else {
                None
            })
        },
        None => (0, None),
    }
}

pub fn find_pcs(game: Game, tx: Sender<Vec<Pc>>) -> Arc<AtomicBool> {
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let cloned_flag = cancel_flag.clone();

    thread::spawn(move || {
        if let Some(pcs) = find_pcs_helper(&game, cloned_flag) {
            tx.send(pcs).unwrap();
        } else {
            return;
        }
    });
    cancel_flag
}

// Returns vec of all PC solves it can find from current position and queue
fn find_pcs_helper(game: &Game, cancel_flag: Arc<AtomicBool>) -> Option<Vec<Pc>> {
    // First, check if we should even search at all
    let initial_state = PcState::from(game, 4);
    if initial_state.fails_early(&Vec::from(game.queue.clone())) {
        return Some(Vec::new());
    }

    let mut solves: Vec<Pc> = Vec::new();
    let mut stack: Vec<PcState> = vec![initial_state];
    let mut visited: HashSet<PcState> = HashSet::new();
    let queue = Vec::from(game.queue.clone());

    while let Some(state) = stack.pop() {
        if cancel_flag.load(Ordering::Relaxed) {
            return None;
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        if state.is_solved() {
            solves.push(Pc::from(&state));
            println!("Found solve");
        }

        for &successor in state.successors(&queue).iter() {
            if visited.contains(&successor) {
                continue;
            }
            stack.push(successor);
        }
    }
    Some(solves)
}
