use crate::bitboard::{BitBoard, EMPTY};
use lazy_static::lazy_static;

pub fn get_bishop_moves(bishop_bitboard: BitBoard, blockers: BitBoard) -> BitBoard {
    let mut moves = EMPTY;

    let mut up_right = bishop_bitboard;
    while up_right.get_rank() < 7 && up_right.get_file() < 7 {
        up_right = up_right.shift_up().shift_right();
        moves |= up_right;
        if up_right & blockers != EMPTY {
            break;
        }
    }

    let mut up_left = bishop_bitboard;
    while up_left.get_rank() < 7 && up_left.get_file() > 0 {
        up_left = up_left.shift_up().shift_left();
        moves |= up_left;
        if up_left & blockers != EMPTY {
            break;
        }
    }

    let mut down_right = bishop_bitboard;
    while down_right.get_rank() > 0 && down_right.get_file() < 7 {
        down_right = down_right.shift_down().shift_right();
        moves |= down_right;
        if down_right & blockers != EMPTY {
            break;
        }
    }

    let mut down_left = bishop_bitboard;
    while down_left.get_rank() > 0 && down_left.get_file() > 0 {
        down_left = down_left.shift_down().shift_left();
        moves |= down_left;
        if down_left & blockers != EMPTY {
            break;
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bishop_moves() {
        let bishop_bitboard = BitBoard::from_index(35);
        let blockers = BitBoard::from_index(28) | BitBoard::from_index(42);
        let moves = get_bishop_moves(bishop_bitboard, blockers);

        // All moves should be
        let expected = BitBoard::from_index(8)
            | BitBoard::from_index(17)
            | BitBoard::from_index(26)
            | BitBoard::from_index(28)
            | BitBoard::from_index(42)
            | BitBoard::from_index(44)
            | BitBoard::from_index(53)
            | BitBoard::from_index(62);

        assert_eq!(moves, expected);
    }
}
