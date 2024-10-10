use std::{marker::PhantomData, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not}};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bitmask<T> {
    mask: u64,
    _marker: PhantomData<T>
}

pub(crate) struct WhitePieces;
pub(crate) struct WhitePawns;
pub(crate) struct WhiteKnights;
pub(crate) struct WhiteBishops;
pub(crate) struct WhiteRooks;
pub(crate) struct WhiteQueens;
pub(crate) struct WhiteKings;
pub(crate) struct BlackPieces;
pub(crate) struct BlackPawns;
pub(crate) struct BlackKnights;
pub(crate) struct BlackBishops;
pub(crate) struct BlackRooks;
pub(crate) struct BlackQueens;
pub(crate) struct BlackKings;

trait ArePieces {}
impl ArePieces for WhitePawns {}
impl ArePieces for WhiteKnights {}
impl ArePieces for WhiteBishops {}
impl ArePieces for WhiteRooks {}
impl ArePieces for WhiteQueens {}
impl ArePieces for WhiteKings {}
impl ArePieces for BlackPawns {}
impl ArePieces for BlackKnights {}
impl ArePieces for BlackBishops {}
impl ArePieces for BlackRooks {}
impl ArePieces for BlackQueens {}
impl ArePieces for BlackKings {}

trait AreWhitePieces {}
impl AreWhitePieces for WhitePawns {}
impl AreWhitePieces for WhiteKnights {}
impl AreWhitePieces for WhiteBishops {}
impl AreWhitePieces for WhiteRooks {}
impl AreWhitePieces for WhiteQueens {}
impl AreWhitePieces for WhiteKings {}

trait AreBlackPieces {}
impl AreBlackPieces for BlackPawns {}
impl AreBlackPieces for BlackKnights {}
impl AreBlackPieces for BlackBishops {}
impl AreBlackPieces for BlackRooks {}
impl AreBlackPieces for BlackQueens {}
impl AreBlackPieces for BlackKings {}

type WhitePieceBitmask = Bitmask<WhitePieces>;
type WhitePawnsBitmask = Bitmask<WhitePawns>;
type WhiteKnightsBitmask = Bitmask<WhiteKnights>;
type WhiteBishopsBitmask = Bitmask<WhiteBishops>;
type WhiteRooksBitmask = Bitmask<WhiteRooks>;
type WhiteQueensBitmask = Bitmask<WhiteQueens>;
type WhiteKingsBitmask = Bitmask<WhiteKings>;
type BlackPieceBitmask = Bitmask<BlackPieces>;
type BlackPawnsBitmask = Bitmask<BlackPawns>;
type BlackKnightsBitmask = Bitmask<BlackKnights>;
type BlackBishopsBitmask = Bitmask<BlackBishops>;
type BlackRooksBitmask = Bitmask<BlackRooks>;
type BlackQueensBitmask = Bitmask<BlackQueens>;
type BlackKingsBitmask = Bitmask<BlackKings>;

impl<T> Bitmask<T> {
    fn new() -> Self {
        Self {
            mask: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> From<u64> for Bitmask<T> {
    fn from(value: u64) -> Self {
        Self { mask: value, _marker: PhantomData }
    }
}

impl<T> BitOr for Bitmask<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask | rhs.mask,
            _marker: PhantomData
        }
    }
}

impl<T> BitAnd for Bitmask<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask & rhs.mask,
            _marker: PhantomData
        }
    }
}

impl<T> Not for Bitmask<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            mask: !self.mask,
            _marker: PhantomData,
        }
    }
}

impl<T> BitOrAssign for Bitmask<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask
    }
}

impl<T> BitAndAssign for Bitmask<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.mask &= rhs.mask
    }
}

