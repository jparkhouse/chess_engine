use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PieceEnum {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl fmt::Display for PieceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PieceEnum::*;
        let str: &str = match self {
            WhitePawn => "P",
            WhiteKnight => "N",
            WhiteBishop => "B",
            WhiteRook => "R",
            WhiteQueen => "Q",
            WhiteKing => "K",
            BlackPawn => "p",
            BlackKnight => "n",
            BlackBishop => "b",
            BlackRook => "r",
            BlackQueen => "q",
            BlackKing => "k",
        };
        write!(f, "{}", str)
    }
}

pub(crate) mod piece_structs {
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct Pieces;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct WhitePieces;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct WhitePawns;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct WhiteKnights;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct WhiteBishops;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct WhiteRooks;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct WhiteQueens;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct WhiteKings;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct BlackPieces;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct BlackPawns;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct BlackKnights;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct BlackBishops;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct BlackRooks;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct BlackQueens;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct BlackKings;
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_valid_string_when_using_to_string_method_from_display_trait() {
        use crate::PieceEnum::*;
        // arrange
        let all_pieces = [
            WhitePawn,
            WhiteKnight,
            WhiteBishop,
            WhiteRook,
            WhiteQueen,
            WhiteKing,
            BlackPawn,
            BlackKnight,
            BlackBishop,
            BlackRook,
            BlackQueen,
            BlackKing,
        ];
        let expected_output: Vec<String> = vec![
            String::from("P"), // WhitePawn
            String::from("N"), // WhiteKnight
            String::from("B"), // WhiteBishop
            String::from("R"), // WhiteRook
            String::from("Q"), // WhiteQueen
            String::from("K"), // WhiteKing
            String::from("p"), // BlackPawn
            String::from("n"), // BlackKnight
            String::from("b"), // BlackBishop
            String::from("r"), // BlackRook
            String::from("q"), // BlackQueen
            String::from("k"), // BlackKing
        ];
        // act
        let output: Vec<String> = all_pieces.into_iter().map(|p| p.to_string()).collect();
        // assert
        assert_eq!(output, expected_output)
    }
}
