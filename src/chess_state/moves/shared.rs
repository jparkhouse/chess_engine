use thiserror::Error;

use crate::chess_state::{
    board_bitmask::BoardBitmasks, chess_pieces::PieceEnum, coordinate_point::CoordinatePosition,
    coordinates::{CoordinateError, YCoordinate, XCoordinate}, moves::standard_move::StandardMove,
};

use super::chess_move::ChessMove;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CastleType {
    ShortCastle,
    LongCastle,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Check {
    None,
    Check,
    Checkmate,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Move {
    StandardMove(StandardMove),
    Castle(CastleType),
}

#[derive(Debug, Error)]
pub(crate) enum MoveError {
    #[error("Pawn found on rank one or eight (moved backwards or failed promotion")]
    PawnOnOneOrEight,
    #[error("Coordinate error: {0}")]
    CoordinateError(#[from] CoordinateError),
    #[error("Capture piece not found at {0}")]
    CapturePieceNotFound(CoordinatePosition),
    #[error("Piece {0} cannot be moved diagonally")]
    PieceCannotMoveDiagonally(PieceEnum),
}

impl BoardBitmasks {
    pub(crate) fn get_piece_type_for_capture(
        &self,
        capture_position: CoordinatePosition,
    ) -> Result<PieceEnum, MoveError> {
        if (capture_position.to_bitmask() & self.all_pieces.to_u64()) == 0 {
            Err(MoveError::CapturePieceNotFound(capture_position))
        } else {
            match capture_position {
                _ if capture_position.to_bitmask() & self.white_pieces.to_u64() > 0 => {
                    // tis a white piece
                    match capture_position {
                        _ if capture_position.to_bitmask() & self.white_pawns.to_u64() > 0 => {
                            Ok(PieceEnum::WhitePawn)
                        }
                        _ if capture_position.to_bitmask() & self.white_knights.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteKnight)
                        }
                        _ if capture_position.to_bitmask() & self.white_bishops.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteBishop)
                        }
                        _ if capture_position.to_bitmask() & self.white_rooks.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteRook)
                        }
                        _ if capture_position.to_bitmask() & self.white_queens.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteQueen)
                        }
                        _ if capture_position.to_bitmask() & self.white_kings.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteKing)
                        }
                        _ => Err(MoveError::CapturePieceNotFound(capture_position)),
                    }
                }
                _ => {
                    // must be a black piece
                    match capture_position {
                        _ if capture_position.to_bitmask() & self.black_pawns.to_u64() > 0 => {
                            Ok(PieceEnum::BlackPawn)
                        }
                        _ if capture_position.to_bitmask() & self.black_knights.to_u64() > 0 => {
                            Ok(PieceEnum::BlackKnight)
                        }
                        _ if capture_position.to_bitmask() & self.black_bishops.to_u64() > 0 => {
                            Ok(PieceEnum::BlackBishop)
                        }
                        _ if capture_position.to_bitmask() & self.black_rooks.to_u64() > 0 => {
                            Ok(PieceEnum::BlackRook)
                        }
                        _ if capture_position.to_bitmask() & self.black_queens.to_u64() > 0 => {
                            Ok(PieceEnum::BlackQueen)
                        }
                        _ if capture_position.to_bitmask() & self.black_kings.to_u64() > 0 => {
                            Ok(PieceEnum::BlackKing)
                        }
                        _ => Err(MoveError::CapturePieceNotFound(capture_position)),
                    }
                }
            }
        }
    }

    pub(crate) fn piece_enum_to_bitmask(&self, piece_type: PieceEnum) -> u64 {
        match piece_type {
            PieceEnum::WhitePawn => self.white_pawns.mask,
            PieceEnum::WhiteKnight => self.white_knights.mask,
            PieceEnum::WhiteBishop => self.white_bishops.mask,
            PieceEnum::WhiteRook => self.white_rooks.mask,
            PieceEnum::WhiteQueen => self.white_queens.mask,
            PieceEnum::WhiteKing => self.white_kings.mask,
            PieceEnum::BlackPawn => self.black_pawns.mask,
            PieceEnum::BlackKnight => self.black_knights.mask,
            PieceEnum::BlackBishop => self.black_bishops.mask,
            PieceEnum::BlackRook => self.black_rooks.mask,
            PieceEnum::BlackQueen => self.black_queens.mask,
            PieceEnum::BlackKing => self.black_kings.mask,
        }
    }
}

fn get_valid_space(move_type: ChessMove) -> u64 {
    use XCoordinate::*;
    use YCoordinate::*;
    match move_type {
        ChessMove::Up => !(Eight as u64),
        ChessMove::UpRight => !(Eight as u64 | H as u64),
        ChessMove::Right => !(H as u64),
        ChessMove::DownRight => !(One as u64 | H as u64),
        ChessMove::Down => !(One as u64),
        ChessMove::DownLeft => !(One as u64 | A as u64),
        ChessMove::Left => !(A as u64),
        ChessMove::UpLeft => !(A as u64 | Eight as u64),
        // I have no idea if these are correct
        ChessMove::KnightOne => !(H as u64 | Seven as u64 | Eight as u64),
        ChessMove::KnightTwo => !(G as u64 | H as u64 | Eight as u64),
        ChessMove::KnightFour => !(G as u64 | H as u64 | One as u64),
        ChessMove::KnightFive => !(H as u64 | Seven as u64 | Eight as u64),
        ChessMove::KnightSeven => !(A as u64 | One as u64 | Two as u64),
        ChessMove::KnightEight => !(A as u64 | B as u64 | One as u64),
        ChessMove::KnightTen => !(A as u64 | B as u64 | Seven as u64),
        ChessMove::KnightEleven => !(A as u64 | Seven as u64 | Eight as u64),
    }
}
