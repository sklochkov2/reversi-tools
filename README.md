# Crate Structure

This crate provides a single module called `position`, which contains the following functions:

## Functions

### `apply_move(white: u64, black: u64, move_bit: u64, is_white_move: bool) -> Result<(u64, u64), &'static str>`

Applies a move and updates the positions of white and black pieces.

- **Parameters:**
  - `white` - Bitmask containing the white pieces' positions.
  - `black` - Bitmask containing the black pieces' positions.
  - `move_bit` - Bitmask representing the move.
  - `is_white_move` - Boolean indicating whether it's White's turn to move.

- **Returns:**
  - `Ok(new_white_pos_bitmask, new_black_pos_bitmask)` - Updated positions after the move.
  - `Err(str)` - Error message if the move is invalid.

---

### `compute_moves(me: u64, opp: u64) -> u64`

Finds possible moves for the active player in the current position.

- **Parameters:**
  - `me` - Bitmask of the active player's pieces.
  - `opp` - Bitmask of the opponent's pieces.

- **Returns:**
  - A bitmask representing all possible moves.

---

### `check_game_status(white: u64, black: u64, is_white_move: bool) -> u64`

Checks the current game status, including whether the active player has valid moves or if the game has ended.

- **Parameters:**
  - `white` - Bitmask containing the white pieces' positions.
  - `black` - Bitmask containing the black pieces' positions.
  - `is_white_move` - Boolean indicating whether it's White's turn to move.

- **Returns:**
  - A bitmask representing the current active player's possible moves.
  - Special values:
    - `u64::MAX` - The current player has no valid moves and must pass their turn.
    - `u64::MAX - 1` - Black wins the game.
    - `u64::MAX - 2` - White wins the game.
    - `u64::MAX - 3` - The game is a draw.

---

### `move_to_algebraic(move_bit: u64) -> Option<String>`

Converts a move bitmask into its algebraic notation.

- **Parameters:**
  - `move_bit` - Bitmask representing the move.

- **Returns:**
  - `Some(String)` - The move in algebraic notation (e.g., `1 -> "a1"`).
  - `None` - If the move is not valid.

---

### `move_to_bitmap(move_notation: &str) -> Result<u64, &str>`

Converts a move from algebraic notation to its bitmask representation.

- **Parameters:**
  - `move_notation` - A string containing the move's algebraic notation (e.g., "a1").

- **Returns:**
  - `Ok(u64)` - Bitmask representation of the move.
  - `Err(&str)` - Error message if the notation is invalid.


