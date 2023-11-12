use crate::coord::Coord;
use std::fmt::Display;
use std::ops;

pub const EMPTY: BitBoard = BitBoard { board: 0 };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitBoard {
    board: u64,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard { board: 0 }
    }

    pub fn from(board: u64) -> BitBoard {
        BitBoard { board }
    }

    pub fn from_bitboard(bit_board: BitBoard) -> BitBoard {
        BitBoard {
            board: bit_board.board,
        }
    }

    pub fn from_coord(coord: Coord) -> BitBoard {
        BitBoard {
            board: 1 << coord.to_index(),
        }
    }

    pub fn from_index(index: u8) -> BitBoard {
        BitBoard { board: 1 << index }
    }

    #[inline(always)]
    pub fn trailing_zeros(&self) -> u8 {
        self.board.trailing_zeros() as u8
    }

    #[inline(always)]
    pub fn in_bounds(coord: Coord) -> bool {
        coord.x < 8 && coord.y < 8
    }

    #[inline(always)]
    pub fn set(&mut self, pos: Coord) {
        self.board |= 1 << pos.to_index()
    }

    #[inline(always)]
    pub fn set_index(&mut self, index: u8) {
        self.board |= 1 << index
    }

    #[inline(always)]
    pub fn unset(&mut self, pos: Coord) {
        self.board &= !(1 << pos.to_index())
    }

    #[inline(always)]
    pub fn get(&self, pos: Coord) -> bool {
        (self.board & (1 << pos.to_index())) != 0
    }

    #[inline(always)]
    pub fn get_index(&self, index: u8) -> bool {
        (self.board & (1 << index)) != 0
    }

    #[inline(always)]
    pub fn get_rank(&self) -> u8 {
        (self.board.trailing_zeros() / 8) as u8
    }

    #[inline(always)]
    pub fn get_file(&self) -> u8 {
        self.board.trailing_zeros() as u8 % 8
    }

    #[inline(always)]
    pub fn shift_down(&mut self) {
        self.board >>= 8;
    }
    #[inline(always)]
    pub fn shift_up(&mut self) {
        self.board <<= 8;
    }
    #[inline(always)]
    pub fn shift_left(&mut self) {
        self.board >>= 1;
    }
    #[inline(always)]
    pub fn shift_right(&mut self) {
        self.board <<= 1;
    }

    pub fn get_board(&self) -> u64 {
        self.board
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for y in 0..8 {
            for x in 0..8 {
                if self.get(Coord::new(x, y)) {
                    s.push('x');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl ops::BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.board |= rhs.board;
    }
}

impl ops::BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: BitBoard) {
        self.board &= rhs.board;
    }
}

impl ops::BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.board ^= rhs.board;
    }
}

impl ops::BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board | rhs.board,
        }
    }
}

impl ops::BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board & rhs.board,
        }
    }
}

impl ops::BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board ^ rhs.board,
        }
    }
}

impl ops::Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        BitBoard { board: !self.board }
    }
}

impl ops::Shl<u8> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: u8) -> BitBoard {
        BitBoard {
            board: self.board << rhs,
        }
    }
}

impl ops::Shr<u8> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: u8) -> BitBoard {
        BitBoard {
            board: self.board >> rhs,
        }
    }
}
