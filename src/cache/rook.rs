use crate::bitboard::{BitBoard, EMPTY};
use lazy_static::lazy_static;

pub fn get_rook_moves(rook_bitboard: BitBoard, blockers: BitBoard) -> BitBoard {
    let mut moves = EMPTY;

    let mut up = rook_bitboard;
    while up.get_rank() < 7 {
        up = up.shift_up();
        moves |= up;
        if up & blockers != EMPTY {
            break;
        }
    }

    let mut down = rook_bitboard;
    while down.get_rank() > 0 {
        down = down.shift_down();
        moves |= down;
        if down & blockers != EMPTY {
            break;
        }
    }

    let mut right = rook_bitboard;
    while right.get_file() < 7 {
        right = right.shift_right();
        moves |= right;
        if right & blockers != EMPTY {
            break;
        }
    }

    let mut left = rook_bitboard;
    while left.get_file() > 0 {
        left = left.shift_left();
        moves |= left;
        if left & blockers != EMPTY {
            break;
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rook_moves() {
        let rook_bitboard = BitBoard::from_index(35);
        let blockers = BitBoard::from_index(34) | BitBoard::from_index(27);
        let moves = get_rook_moves(rook_bitboard, blockers);

        // All moves should be
        let expected = BitBoard::from_index(27)
            | BitBoard::from_index(34)
            | BitBoard::from_index(36)
            | BitBoard::from_index(37)
            | BitBoard::from_index(38)
            | BitBoard::from_index(39)
            | BitBoard::from_index(43)
            | BitBoard::from_index(51)
            | BitBoard::from_index(59);

        assert_eq!(moves, expected);
    }
}
