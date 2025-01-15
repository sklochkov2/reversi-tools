# reversi-tools
Utility library implementing various methods of manipulating a Reversi position in Rust

## Crate structure

The crate provides a single module called `position` which contains the following functions:

* `apply_move(white: u64, black: u64, move_bit: u64, is_white_move: bool) -> Result<(u64, u64), &'static str>`
** white, black - bitmasks containing the white and black pieces' positions, respectively.
** move_bit - the move bitmask containing.
** is_white_move - boolean denoting whether it's White to move.
** Returns a Result containing either Ok(new_white_pos_bitmask, new_black_pos_bitmask) or Err(str).
* `compute_moves(me: u64, opp: u64) -> u64` finds moves possible in a particular position.
** me, opp - bitmasks containing the active player's and their opponent's respective bitmasks.
** Returns a bitmask of available moves.
* `check_game_status(white: u64, black: u64, is_white_move: bool) -> u64` checks what's going on in the game.
** white, black - bitmasks containing the white and black pieces' positions, respectively. 
** is_white_move - boolean denoting whether it's White to move.
** Returns a bitmask of the current active player's moves or one of three special values.
** u64::MAX means that the current player has to pass their move.
** u64::MAX - 1 means that Black won.
** u64::MAX - 2 means that White won.
** u64::MAX - 3 means that the game is a draw.
* `move_to_algebraic(move_bit: u64) -> Option<String>` accepts a move bitmask and returns an Option containing either its algebraic representation, e. g. 1 -> "a1" or None if the move is not valid.
* `move_to_bitmap(move_notation: &str) -> Result<u64, &str>` accepts a string containing a move's algebraic notation (e. g. "a1") and returns its bitmask representation or an error.

