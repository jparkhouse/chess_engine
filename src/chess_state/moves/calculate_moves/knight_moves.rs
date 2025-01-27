use crate::chess_state::{board_bitmask::BoardBitmasks, moves::{chess_move::ChessDirection, shared::MoveError}};

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

fn helper_calculate_knight_move(knights: u64, captures: u64, occupied: u64, direction: ChessDirection) {
    let invalid_map: u64 = match direction {
        ChessDirection::KnightOne => todo!(),
        ChessDirection::KnightTwo => todo!(),
        ChessDirection::KnightFour => todo!(),
        ChessDirection::KnightFive => todo!(),
        ChessDirection::KnightSeven => todo!(),
        ChessDirection::KnightEight => todo!(),
        ChessDirection::KnightTen => todo!(),
        ChessDirection::KnightEleven => todo!(),
        _ => {todo!()}
    };
}