use std::ops::BitOr;

pub(crate) trait ChessMoves {
    fn shift_move(self, direction: ChessMove) -> Self;
}

pub(crate) enum ChessMove {
    // cardinal directions
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,

    // knight moves based on clock
    KnightOne,
    KnightTwo,

    KnightFour,
    KnightFive,

    KnightSeven,
    KnightEight,

    KnightTen,
    KnightEleven,
}

impl ChessMoves for u64 {
    #[inline]
    fn shift_move(self, direction: ChessMove) -> Self {
        use ChessMove::*;
        match direction {
            Up => self << 8,
            Right => self >> 1,
            Down => self >> 8,
            Left => self << 1,

            UpRight => self.shift_move(Up).shift_move(Right),
            DownRight => self.shift_move(Down).shift_move(Right),
            DownLeft => self.shift_move(Down).shift_move(Left),
            UpLeft => self.shift_move(Up).shift_move(Left),

            KnightOne => self.shift_move(Up).shift_move(Up).shift_move(Right),
            KnightTwo => self.shift_move(Right).shift_move(Right).shift_move(Up),
            KnightFour => self.shift_move(Right).shift_move(Right).shift_move(Down),
            KnightFive => self.shift_move(Down).shift_move(Down).shift_move(Right),
            KnightSeven => self.shift_move(Down).shift_move(Down).shift_move(Left),
            KnightEight => self.shift_move(Left).shift_move(Left).shift_move(Down),
            KnightTen => self.shift_move(Left).shift_move(Left).shift_move(Up),
            KnightEleven => self.shift_move(Up).shift_move(Up).shift_move(Left),
        }
    }
}

#[cfg(test)]
mod tests {
    mod chess_move_for_u64 {
        use crate::chess_state::{
            coordinates::{XCoordinate::*, YCoordinate::*},
            moves::chess_move::{ChessMove::*, ChessMoves},
        };

        #[test]
        fn moves_up_when_directed_up() {
            // arrange
            let start = (A as u64) & (Two as u64);
            let expected_output = (A as u64) & (Three as u64);
            // act
            let output = start.shift_move(Up);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_left_when_directed_left() {
            // arrange
            let start = (B as u64) & (Two as u64);
            let expected_output = (A as u64) & (Two as u64);
            // act
            let output = start.shift_move(Left);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_right_when_directed_right() {
            // arrange
            let start = (A as u64) & (Two as u64);
            let expected_output = (B as u64) & (Two as u64);
            // act
            let output = start.shift_move(Right);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_down_when_directed_down() {
            // arrange
            let start = (A as u64) & (Three as u64);
            let expected_output = (A as u64) & (Two as u64);
            // act
            let output = start.shift_move(Down);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_up_right_when_directed_up_right() {
            // arrange
            let start = (A as u64) & (Two as u64);
            let expected_output = (B as u64) & (Three as u64);
            // act
            let output = start.shift_move(UpRight);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_up_left_when_directed_up_left() {
            // arrange
            let start = (C as u64) & (Three as u64);
            let expected_output = (B as u64) & (Four as u64);
            // act
            let output = start.shift_move(UpLeft);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_down_right_when_directed_down_right() {
            // arrange
            let start = (F as u64) & (Five as u64);
            let expected_output = (G as u64) & (Four as u64);
            // act
            let output = start.shift_move(DownRight);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_down_left_when_directed_down_left() {
            // arrange
            let start = (C as u64) & (Three as u64);
            let expected_output = (B as u64) & (Two as u64);
            // act
            let output = start.shift_move(DownLeft);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_one_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (E as u64) & (Six as u64);
            // act
            let output = start.shift_move(KnightOne);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_two_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (F as u64) & (Five as u64);
            // act
            let output = start.shift_move(KnightTwo);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_four_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (F as u64) & (Three as u64);
            // act
            let output = start.shift_move(KnightFour);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_five_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (E as u64) & (Two as u64);
            // act
            let output = start.shift_move(KnightFive);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_seven_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (C as u64) & (Two as u64);
            // act
            let output = start.shift_move(KnightSeven);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_eight_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (B as u64) & (Three as u64);
            // act
            let output = start.shift_move(KnightEight);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_ten_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (B as u64) & (Five as u64);
            // act
            let output = start.shift_move(KnightTen);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_eleven_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (C as u64) & (Six as u64);
            // act
            let output = start.shift_move(KnightEleven);
            // assert
            assert_eq!(output, expected_output)
        }
    }
}
