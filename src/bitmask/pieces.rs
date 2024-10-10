use super::generic::Bitmask;
use crate::chess_state::chess_pieces::piece_structs::*;

pub(crate) trait ArePieces {}
impl ArePieces for WhitePieces {}
impl ArePieces for WhitePawns {}
impl ArePieces for WhiteKnights {}
impl ArePieces for WhiteBishops {}
impl ArePieces for WhiteRooks {}
impl ArePieces for WhiteQueens {}
impl ArePieces for WhiteKings {}
impl ArePieces for BlackPieces {}
impl ArePieces for BlackPawns {}
impl ArePieces for BlackKnights {}
impl ArePieces for BlackBishops {}
impl ArePieces for BlackRooks {}
impl ArePieces for BlackQueens {}
impl ArePieces for BlackKings {}

impl<T: ArePieces> From<Bitmask<T>> for Bitmask<Pieces> {
    fn from(value: Bitmask<T>) -> Self {
        Self {
            mask: value.mask,
            _marker: std::marker::PhantomData,
        }
    }
}

pub(crate) trait BitOpsForPieces<T: ArePieces> {
    fn bitor_pieces(self, rhs: Bitmask<T>) -> Bitmask<Pieces>;
    fn bitand_pieces(self, rhs: Bitmask<T>) -> Bitmask<Pieces>;
}

impl<T: ArePieces, U: ArePieces> BitOpsForPieces<T> for Bitmask<U> {
    fn bitor_pieces(self, rhs: Bitmask<T>) -> Bitmask<Pieces> {
        Bitmask::<Pieces>::from(self) | Bitmask::<Pieces>::from(rhs)
    }

    fn bitand_pieces(self, rhs: Bitmask<T>) -> Bitmask<Pieces> {
        Bitmask::<Pieces>::from(self) & Bitmask::<Pieces>::from(rhs)
    }
}
