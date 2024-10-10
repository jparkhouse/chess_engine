pub(crate) mod generic;
pub(crate) mod pieces;
pub(crate) mod white_pieces;
pub(crate) mod black_pieces;

#[cfg(test)]
mod bitmask_tests {

    #[test]
    fn can_use_from() {
        use super::{
            generic::Bitmask,
            white_pieces::BitOpsForWhitePieces
        };

        use crate::chess_state::chess_pieces::piece_structs::{WhiteKnights, WhitePawns};

        let white_pawns = Bitmask::<WhitePawns>::from_u64(2);
        let white_knights = Bitmask::<WhiteKnights>::from_u64(4);
        let output = white_pawns.bitor_white_pieces(white_knights);

        assert_eq!(output.mask, 6)
    }
}