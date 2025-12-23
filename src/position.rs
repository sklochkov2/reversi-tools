fn flip_in_dir(move_bit: u64, me: u64, opp: u64, shift: fn(u64) -> u64) -> u64 {
    let mut mask = shift(move_bit);
    let mut flipped = 0_u64;

    while (mask & opp) != 0 {
        flipped |= mask;
        mask = shift(mask);
    }

    if (mask & me) != 0 {
        flipped
    } else {
        0
    }
}

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

#[inline]
fn shift_north(x: u64) -> u64 {
    x << 8
}
#[inline]
fn shift_south(x: u64) -> u64 {
    x >> 8
}
#[inline]
fn shift_east(x: u64) -> u64 {
    (x & NOT_H_FILE) << 1
}
#[inline]
fn shift_west(x: u64) -> u64 {
    (x & NOT_A_FILE) >> 1
}
#[inline]
fn shift_ne(x: u64) -> u64 {
    (x & NOT_H_FILE) << 9
}
#[inline]
fn shift_nw(x: u64) -> u64 {
    (x & NOT_A_FILE) << 7
}
#[inline]
fn shift_se(x: u64) -> u64 {
    (x & NOT_H_FILE) >> 7
}
#[inline]
fn shift_sw(x: u64) -> u64 {
    (x & NOT_A_FILE) >> 9
}

macro_rules! flip_unrolled_dir {
    ($move_bit:expr, $me:expr, $opp:expr, $shift:ident) => {{
        let mut x = $shift($move_bit) & $opp;
        if x == 0 {
            0
        } else {
            x |= $shift(x) & $opp;
            x |= $shift(x) & $opp;
            x |= $shift(x) & $opp;
            x |= $shift(x) & $opp;
            x |= $shift(x) & $opp;
            if ($shift(x) & $me) != 0 {
                x
            } else {
                0
            }
        }
    }};
}

#[inline(always)]
pub fn apply_move_unchecked(
    white: u64,
    black: u64,
    move_bit: u64,
    is_white_move: bool,
) -> (u64, u64) {
    let (me, opp) = if is_white_move {
        (white, black)
    } else {
        (black, white)
    };

    let flip_mask = flip_unrolled_dir!(move_bit, me, opp, shift_north)
        | flip_unrolled_dir!(move_bit, me, opp, shift_south)
        | flip_unrolled_dir!(move_bit, me, opp, shift_east)
        | flip_unrolled_dir!(move_bit, me, opp, shift_west)
        | flip_unrolled_dir!(move_bit, me, opp, shift_ne)
        | flip_unrolled_dir!(move_bit, me, opp, shift_nw)
        | flip_unrolled_dir!(move_bit, me, opp, shift_se)
        | flip_unrolled_dir!(move_bit, me, opp, shift_sw);

    debug_assert!(flip_mask != 0);

    let new_me = me | move_bit | flip_mask;
    let new_opp = opp & !flip_mask;

    if is_white_move {
        (new_me, new_opp)
    } else {
        (new_opp, new_me)
    }
}

pub fn apply_move(
    white: u64,
    black: u64,
    move_bit: u64,
    is_white_move: bool,
) -> Result<(u64, u64), &'static str> {
    let occupied = white | black;
    if (move_bit & occupied) != 0 {
        return Err("Square already occupied");
    }

    let (me, opp) = if is_white_move {
        (white, black)
    } else {
        (black, white)
    };

    let mut flip_mask = 0_u64;
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_north);
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_south);
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_east);
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_west);
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_ne);
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_nw);
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_se);
    flip_mask |= flip_in_dir(move_bit, me, opp, shift_sw);

    if flip_mask == 0 {
        return Err("No flips!");
    }

    let new_me = me | move_bit | flip_mask;
    let new_opp = opp & !flip_mask;

    if is_white_move {
        Ok((new_me, new_opp))
    } else {
        Ok((new_opp, new_me))
    }
}

pub fn compute_moves(me: u64, opp: u64) -> u64 {
    let empty = !(me | opp);

    let mut moves = 0_u64;

    let mut mask = shift_north(me) & opp;
    mask |= shift_north(mask) & opp;
    mask |= shift_north(mask) & opp;
    mask |= shift_north(mask) & opp;
    mask |= shift_north(mask) & opp;
    mask |= shift_north(mask) & opp;
    moves |= shift_north(mask) & empty;

    // South
    mask = shift_south(me) & opp;
    mask |= shift_south(mask) & opp;
    mask |= shift_south(mask) & opp;
    mask |= shift_south(mask) & opp;
    mask |= shift_south(mask) & opp;
    mask |= shift_south(mask) & opp;
    moves |= shift_south(mask) & empty;

    // East
    mask = shift_east(me) & opp;
    mask |= shift_east(mask) & opp;
    mask |= shift_east(mask) & opp;
    mask |= shift_east(mask) & opp;
    mask |= shift_east(mask) & opp;
    mask |= shift_east(mask) & opp;
    moves |= shift_east(mask) & empty;

    // West
    mask = shift_west(me) & opp;
    mask |= shift_west(mask) & opp;
    mask |= shift_west(mask) & opp;
    mask |= shift_west(mask) & opp;
    mask |= shift_west(mask) & opp;
    mask |= shift_west(mask) & opp;
    moves |= shift_west(mask) & empty;

    // Northeast
    mask = shift_ne(me) & opp;
    mask |= shift_ne(mask) & opp;
    mask |= shift_ne(mask) & opp;
    mask |= shift_ne(mask) & opp;
    mask |= shift_ne(mask) & opp;
    mask |= shift_ne(mask) & opp;
    moves |= shift_ne(mask) & empty;

    // Northwest
    mask = shift_nw(me) & opp;
    mask |= shift_nw(mask) & opp;
    mask |= shift_nw(mask) & opp;
    mask |= shift_nw(mask) & opp;
    mask |= shift_nw(mask) & opp;
    mask |= shift_nw(mask) & opp;
    moves |= shift_nw(mask) & empty;

    // Southeast
    mask = shift_se(me) & opp;
    mask |= shift_se(mask) & opp;
    mask |= shift_se(mask) & opp;
    mask |= shift_se(mask) & opp;
    mask |= shift_se(mask) & opp;
    mask |= shift_se(mask) & opp;
    moves |= shift_se(mask) & empty;

    // Southwest
    mask = shift_sw(me) & opp;
    mask |= shift_sw(mask) & opp;
    mask |= shift_sw(mask) & opp;
    mask |= shift_sw(mask) & opp;
    mask |= shift_sw(mask) & opp;
    mask |= shift_sw(mask) & opp;
    moves |= shift_sw(mask) & empty;

    moves
}

pub fn check_game_status(white: u64, black: u64, is_white_move: bool) -> u64 {
    let (me, opp) = if is_white_move {
        (white, black)
    } else {
        (black, white)
    };
    let my_moves: u64 = compute_moves(me, opp);
    if my_moves > 0 {
        return my_moves;
    }
    let opp_moves: u64 = compute_moves(opp, me);
    if opp_moves > 0 {
        return u64::MAX;
    }
    let white_count = white.count_ones();
    let black_count = black.count_ones();

    if white_count > black_count {
        return u64::MAX - 2;
    } else if black_count > white_count {
        return u64::MAX - 1;
    } else {
        return u64::MAX - 3;
    };
}

pub fn move_to_algebraic(move_bit: u64) -> Option<String> {
    if move_bit.count_ones() != 1 {
        return None;
    }

    let pos = move_bit.trailing_zeros() as usize;
    let file = (pos % 8) as u8 + b'a';
    let rank = (pos / 8) as u8 + b'1';

    Some(format!("{}{}", file as char, rank as char))
}

pub fn move_to_bitmap(move_notation: &str) -> Result<u64, &str> {
    if move_notation.len() != 2 {
        return Err("Invalid move notation");
    }
    let file = move_notation.chars().next().unwrap().to_ascii_lowercase() as usize - 'a' as usize;
    let rank = move_notation.chars().nth(1).unwrap().to_digit(10).unwrap() as usize - 1;
    if file >= 8 || rank >= 8 {
        return Err("Invalid move notation");
    }

    let move_pos = rank * 8 + file;
    let move_bit = 1u64 << move_pos;
    Ok(move_bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to_algebraic() {
        assert_eq!(move_to_algebraic(1).unwrap(), "a1");
        assert_eq!(move_to_algebraic(4).unwrap(), "c1");
        assert_eq!(move_to_algebraic(3), None);
    }

    #[test]
    fn test_move_to_bitmap() {
        assert_eq!(move_to_bitmap("a1").unwrap(), 1);
        assert_eq!(move_to_bitmap("foo"), Err("Invalid move notation"));
    }

    /// Helper bitwise shift functions which do not take board edges into account
    /// unlike similar functions from the base program.

    fn shift_left(x: u64) -> u64 {
        x << 1
    }

    fn shift_right(x: u64) -> u64 {
        x >> 1
    }

    fn shift_up(x: u64) -> u64 {
        x << 8
    }

    fn shift_down(x: u64) -> u64 {
        x >> 8
    }

    #[test]
    fn test_flip_in_dir_basic_single_flip_left() {
        let move_bit = 0b0010_0000;
        let me = 0b1000_0000;
        let opp = 0b0100_0000;

        let flipped = flip_in_dir(move_bit, me, opp, shift_left);
        assert_eq!(flipped, 0b0100_0000); // We expect exactly the opp bit to flip.
    }

    #[test]
    fn test_flip_in_dir_no_flip_left() {
        let move_bit = 0b0001_0000;
        let me = 0b1000_0000;
        let opp = 0b0010_0000;

        let flipped = flip_in_dir(move_bit, me, opp, shift_left);
        assert_eq!(flipped, 0); // No valid chain, so nothing flips.
    }

    #[test]
    fn test_flip_in_dir_multiple_flips_left() {
        let move_bit = 0b0000_1000;
        let opp = 0b0111_0000;
        let me = 0b1000_0000;

        let flipped = flip_in_dir(move_bit, me, opp, shift_left);
        assert_eq!(flipped, 0b0111_0000);
    }

    #[test]
    fn test_flip_in_dir_interrupted_chain_left() {
        let move_bit = 0b0000_1000;
        let opp = 0b0010_0000;
        let me = 0b1000_0000;

        let flipped = flip_in_dir(move_bit, me, opp, shift_left);
        assert_eq!(flipped, 0);
    }

    #[test]
    fn test_flip_in_dir_basic_single_flip_right() {
        let move_bit = 0x800;
        let opp = 0x400;
        let me = 0x200;

        let flipped = flip_in_dir(move_bit, me, opp, shift_right);
        assert_eq!(flipped, 0x400);
    }

    #[test]
    fn test_flip_in_dir_basic_single_flip_up() {
        let move_bit = 0x0001;
        let opp = 0x0100;
        let me = 0x010000;

        let flipped = flip_in_dir(move_bit, me, opp, shift_up);
        assert_eq!(flipped, 0x0100);
    }

    #[test]
    fn test_flip_in_dir_no_flip_up_due_to_gap() {
        let move_bit = 0x0001;
        let opp = 0x0200;
        let me = 0x010000;

        let flipped = flip_in_dir(move_bit, me, opp, shift_up);
        assert_eq!(flipped, 0);
    }

    #[test]
    fn test_flip_in_dir_multiple_flips_down() {
        let move_bit = 0x1000000000000000;
        let opp = 0x0010000000000000 | 0x0000100000000000;
        let me = 0x0000001000000000;

        let flipped = flip_in_dir(move_bit, me, opp, shift_down);
        assert_eq!(flipped, opp);
    }

    #[test]
    fn test_apply_move() {
        assert_eq!(
            apply_move(
                35253361508352,
                171935537184,
                move_to_bitmap("c4").unwrap(),
                true
            )
            .unwrap(),
            (35253562834944, 171801319456)
        );
        assert_eq!(
            apply_move(
                35253361508352,
                171935537184,
                move_to_bitmap("a1").unwrap(),
                true
            ),
            Err("No flips!")
        );
        assert_eq!(
            apply_move(
                35253361508352,
                171935537184,
                move_to_bitmap("a3").unwrap(),
                true
            ),
            Err("Square already occupied")
        );
    }

    #[test]
    fn test_compute_moves_no_possible_moves() {
        let me = 0x1;
        let opp = 0x2;

        //print_board(me, opp, 0, 0, false);
        let moves = compute_moves(opp, me);
        assert_eq!(moves, 0, "Expected no moves, got some bits set instead.");
    }

    #[test]
    fn test_compute_moves_simple_horizontal() {
        let me = 1 << 3;
        let opp = (1 << 2) | (1 << 1);

        let moves = compute_moves(me, opp);
        assert_eq!(
            moves, 1,
            "Expected bit 0 to be a valid move, but got something else."
        );
    }

    #[test]
    fn test_compute_moves_standard_othello_black_to_move() {
        // Black pieces (me)
        let me = (1 << 28) | (1 << 35);
        // White pieces (opp)
        let opp = (1 << 27) | (1 << 36);

        let moves = compute_moves(me, opp);

        let expected_moves = (1 << 19) | (1 << 26) | (1 << 37) | (1 << 44);

        assert_eq!(
            moves, expected_moves,
            "Black's standard opening moves did not match the expected bitmask."
        );
    }

    #[test]
    fn test_compute_moves_standard_othello_white_to_move() {
        // White pieces (me)
        let me = (1 << 27) | (1 << 36);
        // Black pieces (opp)
        let opp = (1 << 28) | (1 << 35);

        let moves = compute_moves(me, opp);

        let expected_moves = (1 << 20) | (1 << 29) | (1 << 34) | (1 << 43);
        assert_eq!(
            moves, expected_moves,
            "White's standard opening moves did not match the expected bitmask."
        );
    }

    #[test]
    fn test_compute_moves_all_filled_but_one() {
        let empty_bit = 12;
        let all_board = u64::MAX;
        let me = all_board & !(1 << empty_bit) & !(1 << 10) & !(1 << 11);
        let opp = (1 << 10) | (1 << 11);
        //print_board(me, opp, 0, 0, false);
        let moves = compute_moves(opp, me);
        assert_eq!(
            moves, 0,
            "Expected no valid moves on a nearly full board, got a nonzero mask."
        );
    }

    #[test]
    fn test_check_game_status_current_player_has_moves() {
        let white = (1 << 27) | (1 << 36); // (3,3) and (4,4)
        let black = (1 << 28) | (1 << 35); // (3,4) and (4,3)
        let is_white_move = true;

        let expected_moves = (1 << 20) | (1 << 29) | (1 << 34) | (1 << 43);

        let status = check_game_status(white, black, is_white_move);
        assert_eq!(
            status, expected_moves,
            "Expected White's standard opening moves, got something else."
        );
    }

    #[test]
    fn test_check_game_status_current_player_has_no_moves_opponent_does() {
        let white = 0x0000_FFFF_FFFF_F000u64;
        let black = 0x0000_0000_0000_FFFFu64;

        let is_white_move = true;

        let status = check_game_status(white, black, is_white_move);
        assert_eq!(
            status,
            u64::MAX,
            "Expected pass situation (u64::MAX) if current player has no moves but opponent does."
        );
    }

    #[test]
    fn test_check_game_status_both_sides_have_no_moves_white_wins() {
        let white = 14260085270048145407;
        let black = 67108864;
        let is_white_move = true;

        let status = check_game_status(white, black, is_white_move);
        assert_eq!(
            status,
            u64::MAX - 2,
            "Expected White to win => (u64::MAX - 2). Got something else."
        );
    }

    #[test]
    fn test_check_game_status_both_sides_have_no_moves_black_wins() {
        let white = 67108864;
        let black = 14260085270048145407;

        let is_white_move = false;
        //print_board(white, black, 0, 0, false);

        let status = check_game_status(white, black, is_white_move);
        assert_eq!(
            status,
            u64::MAX - 1,
            "Expected Black to win => (u64::MAX - 1). Got something else."
        );
    }

    #[test]
    fn test_check_game_status_both_sides_have_no_moves_tie() {
        let white = 0x0000_0000_FFFF_FFFFu64; // exactly 32 bits set
        let black = 0xFFFF_FFFF_0000_0000u64; // exactly 32 bits set

        let is_white_move = true; // or false, same result if no moves remain.

        let status = check_game_status(white, black, is_white_move);
        assert_eq!(
            status,
            u64::MAX - 3,
            "Expected tie => (u64::MAX - 3). Got something else."
        );
    }

    #[test]
    fn test_check_game_status_black_has_moves() {
        let white = (1 << 27) | (1 << 36); // (3,3) and (4,4)
        let black = (1 << 28) | (1 << 35); // (3,4) and (4,3)
        let is_white_move = false; // black to move

        let expected_moves = (1 << 19) | (1 << 26) | (1 << 37) | (1 << 44);

        let status = check_game_status(white, black, is_white_move);
        assert_eq!(
            status, expected_moves,
            "Expected black's moves in the standard opening, got something else."
        );
    }
}
