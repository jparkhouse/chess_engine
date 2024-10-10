use crate::chess_state::chess_pieces::piece_structs::*;

use super::generic::Bitmask;

pub(crate) trait AreWhitePieces {}

impl AreWhitePieces for WhitePawns {}
impl AreWhitePieces for WhiteKnights {}
impl AreWhitePieces for WhiteBishops {}
impl AreWhitePieces for WhiteRooks {}
impl AreWhitePieces for WhiteQueens {}
impl AreWhitePieces for WhiteKings {}

impl<T: AreWhitePieces> From<Bitmask<T>> for Bitmask<WhitePieces> {
    fn from(value: Bitmask<T>) -> Self {
        Self {
            mask: value.mask,
            _marker: std::marker::PhantomData,
        }
    }
}

pub(crate) trait BitOpsForWhitePieces<T: AreWhitePieces> {
    fn bitor_white_pieces(self, rhs: Bitmask<T>) -> Bitmask<WhitePieces>;
    fn bitand_white_pieces(self, rhs: Bitmask<T>) -> Bitmask<WhitePieces>;
}

impl<T: AreWhitePieces, U: AreWhitePieces> BitOpsForWhitePieces<T> for Bitmask<U> {
    fn bitor_white_pieces(self, rhs: Bitmask<T>) -> Bitmask<WhitePieces> {
        Bitmask::<WhitePieces>::from(self) | Bitmask::<WhitePieces>::from(rhs)
    }
    
    fn bitand_white_pieces(self, rhs: Bitmask<T>) -> Bitmask<WhitePieces> {
        Bitmask::<WhitePieces>::from(self) & Bitmask::<WhitePieces>::from(rhs)
    }
}