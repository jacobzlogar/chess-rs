pub const WHITE_ROOK_MOVES: [u64; 64] = rook_moves();
pub const BLACK_ROOK_MOVES: [u64; 64] = rook_moves();
pub const WHITE_PAWN_MOVES: [u64; 64] = white_pawn_moves();
pub const BLACK_PAWN_MOVES: [u64; 64] = black_pawn_moves();
pub const WHITE_KNIGHT_MOVES: [u64; 64] = knight_moves();
pub const BLACK_KNIGHT_MOVES: [u64; 64] = knight_moves();

pub fn bishop_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0u64; 64];
    moves
}

pub const fn knight_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0u64; 64];
    let mut i = 0;
    while i < 63 {
        let mut x: u64 = 1 << i;
        // skip last column & rows 7/8
        if i % 8 != 7 && i < 47 {
            x |= 1 << (i + 17); // up 2, left 1
        }
        // skip first column & rows 7/8
        if i % 8 != 0 && i < 48 {
            x |= 1 << (i + 15); // up 2, right 1
        }
        // skip seventh/eight column & row 8
        if i % 8 != 7 && i % 8 != 6 && i < 54 {
            x |= 1 << (i + 10); // up 1, left 2
        }
        // skip first/second column & row 8
        if i % 8 != 0 && i % 8 != 1 && i < 56 {
            x |= 1 << (i + 6); // up 1, right 2
        };
        // skip first column & rows 1/2
        if i % 8 != 0 && i > 16 {
            x |= 1 << (i - 17); // down 2, right 1
        }
        // skip last column & rows 1/2
        if i % 7 != 0 && i > 15 {
            x |= 1 << (i - 15); // down 2, left 1
        }
        // // skip first/second column & row 1
        if i % 8 != 0 && i % 8 != 1 && i > 9 {
            x |= 1 << (i - 10); // down 1, right 2
        }
        // // skip seventh/eigth column & row 1
        if i % 8 != 7 && i % 8 != 6 && i > 7 {
            x |= 1 << (i - 6); // down 1, left 2
        }

        moves[i] = x;
        i += 1;
    }

    moves
}

const fn white_pawn_moves() -> [u64; 64] {
    let mut i = 0;
    let mut moves: [u64; 64] = [0u64; 64];
    // TODO: figure out how to handle pawn swap logic, should a player be able to advance to the last square or not?
    // or should that be handled in game logic somewhere and not as a valid move
    while i < 63 {
        let mut x = 1 << i;
        // pawns can advance 2 spaces from their starting position
        if i >= 8 && i <= 16 {
            x |= (1u64).rotate_left(i + 8);
            x |= (1u64).rotate_left(i + 16);
        } else {
            x |= (1u64).rotate_left(i + 8);
        }
        moves[i as usize] = x;
        i += 1;
    }
    moves
}

const fn black_pawn_moves() -> [u64; 64] {
    let mut i = 63;
    let mut moves: [u64; 64] = [0u64; 64];
    while i > 0 {
        let mut x = 1 << i;
        // pawns can advance 2 spaces from their starting position
        if i <= 16 && i >= 8 {
            x |= (1u64).rotate_left(i + 8);
            x |= (1u64).rotate_left(i + 16);
        } else {
            x |= (1u64).rotate_left(i + 8);
        }
        moves[i as usize] = x;
        i -= 1;
    }
    moves
}

const fn rook_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0u64; 64];
    let mut i: i64 = 0;
    while i < 63 {
        let mut x: u64 = 1 << i;
        let mut j = 0;
        // horizontal up
        while j < 8 {
            if (i + (j * 8)) <= 63 {
                x |= 1 << (i + (j * 8));
                j += 1;
            } else {
                break;
            }
        }
        j = 0;
        // horizontal left
        while j < 8 {
            // add moves until we reach the left
            if (i + j) % 8 != 7 {
                x |= 1 << (i + j);
                j += 1;
            } else {
                // and then add one more move for the left edge
                x |= 1 << (i + j);
                break;
            }
        }
        j = 0;
        // horizontal right
        while j < 8 {
            // add moves until we reach the right
            if (i - j) % 8 != 0 {
                x |= 1 << (i - j);
                j += 1;
            } else {
                // and then add one more move for the right edge
                x |= 1 << (i - j);
                break;
            }
        }
        j = 0;
        // horizontal down
        while j < 8 {
            if (i - (j * 8)) >= 0 {
                x |= 1 << i - (j * 8); // down
                j += 1;
            } else {
                break;
            }
        }
        moves[i as usize] = x;
        i += 1;
    }
    moves
}
