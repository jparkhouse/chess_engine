use crate::chess_state::{board_bitmask::BoardBitmasks, moves::{chess_move::ChessMove, shared::MoveError}};

impl BoardBitmasks {
    fn calculate_knight_moves(&self, white: bool) {
        // choose the correct knights
        let local_knights = match white {
            true => self.white_knights.mask,
            false => self.black_knights.mask,
        };
        // choose the correct captures bitmask
        let local_captures = match white {
            true => self.black_pieces.mask,
            false => self.white_pieces.mask
        };
        // choose the correct occupied bitmasks
        let local_occupied = match white {
            true => self.white_pieces.mask,
            false => self.black_pieces.mask,
        };
    }
}

fn helper_calculate_knight_move(knights: u64, captures: u64, occupied: u64, direction: ChessMove) {
    let invalid_map: u64 = match direction {
        ChessMove::KnightOne => todo!(),
        ChessMove::KnightTwo => todo!(),
        ChessMove::KnightFour => todo!(),
        ChessMove::KnightFive => todo!(),
        ChessMove::KnightSeven => todo!(),
        ChessMove::KnightEight => todo!(),
        ChessMove::KnightTen => todo!(),
        ChessMove::KnightEleven => todo!(),
        _ => {}
    }
}