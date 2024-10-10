use std::marker::PhantomData;

use crate::chess_state::chess_pieces::piece_structs::*;
use crate::bitmask::generic::Bitmask;

pub(crate) trait AreBlackPieces {}

impl AreBlackPieces for BlackPawns {}
impl AreBlackPieces for BlackKnights {}
impl AreBlackPieces for BlackBishops {}
impl AreBlackPieces for BlackRooks {}
impl AreBlackPieces for BlackQueens {}
impl AreBlackPieces for BlackKings {}

impl<T: AreBlackPieces> From<Bitmask<T>> for Bitmask<BlackPieces> {
    fn from(value: Bitmask<T>) -> Self {
        Self {
            mask: value.mask,
            _marker: PhantomData
        }
    }
}

pub(crate) trait BitOpsForBlackPieces<T: AreBlackPieces> {
    fn bitor_black_pieces(self, rhs: Bitmask<T>) -> Bitmask<BlackPieces>;
    fn bitand_black_pieces(self, rhs: Bitmask<T>) -> Bitmask<BlackPieces>;
}

impl<T: AreBlackPieces, U: AreBlackPieces> BitOpsForBlackPieces<T> for Bitmask<U> {
    fn bitor_black_pieces(self, rhs: Bitmask<T>) -> Bitmask<BlackPieces> {
        Bitmask::<BlackPieces>::from(self) | Bitmask::<BlackPieces>::from(rhs)
    }
    
    fn bitand_black_pieces(self, rhs: Bitmask<T>) -> Bitmask<BlackPieces> {
        Bitmask::<BlackPieces>::from(self) & Bitmask::<BlackPieces>::from(rhs)
    }
}