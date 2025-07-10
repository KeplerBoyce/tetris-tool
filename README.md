# Tetris Practice Tool
A tool for practicing Tetris strategies.

### Usage
No full releases are offered yet as there are still many key features in progress, but you can build from source yourself to use the program in the meantime. To do this, make sure you have Rust installed (1.88.0-nightly is the version currently used in development). Then, clone this repository and run or build using cargo:
```sh
git clone git@github.com:KeplerBoyce/tetris-tool.git
cd tetris-tool
cargo run --release
```

### Features
Currently offers finesse practice (including optimal finesse on complex soft drops) and perfect clear setups and solutions for 1st, 2nd, 3rd, 4th, and 5th PC.

### Planned features
- More PC setups as well as more advanced lookahead, such as identifying solve chances for various setups.
- Stricter requirements for certain setups.
  - For 3rd PC, the setup finder will sometimes suggest extra T setups, for example, even when the piece on hold from 2nd PC is not a T piece (the setup is buildable with the given queue, it just won't work as intended since there is no second T coming in the future).
  - Similarly, the solve finder will sometimes suggest solutions which don't use the bags as they should, which can have issues such as looping back to 1st PC with a duplicate piece rather than a full fresh bag.
- Score-aware solver to prioritize PC solutions with quads and extra T-spins.
- Practice tools for other general stacking strategies and T-spin setups.
  - Thinking to create as a little game where some randomly generated board state is given as well as a queue of pieces, and you are tasked with finding a solution to create a particular setup with that queue, such as a simple TSD or a more complex setup like fractal, Kaidan, or C-spin.
- May potentially add an opener practice tool as well.
