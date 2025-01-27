use super::chess_move::{ChessDirection, ChessShiftMove};

pub trait WhitePawnAttackMaps {
    fn calculate_unconstrained_white_pawn_attack_maps(self) -> Self;
}

pub trait BlackPawnAttackMaps {
    fn calculate_unconstrained_black_pawn_attack_maps(self) -> Self;
}

pub trait KnightAttackMaps {
    fn calculate_unconstrained_knight_maps(self) -> Self;
}

pub trait BishopAttackMaps {
    fn calculate_unconstrained_bishop_attack_maps(self) -> Self;
}

pub trait QueenAttackMaps {
    fn calculate_unconstrained_queen_attack_maps(self) -> Self;
}

pub trait RookAttackMaps {
    fn calculate_unconstrained_rook_attack_maps(self) -> Self;
}

pub trait KingAttackMaps {
    fn calculate_unconstrained_king_attack_maps(self) -> Self;
}

impl WhitePawnAttackMaps for u64 {
    fn calculate_unconstrained_white_pawn_attack_maps(self) -> Self {
        self.shift_move(ChessDirection::UpLeft) | self.shift_move(ChessDirection::UpRight)
    }
}

impl BlackPawnAttackMaps for u64 {
    fn calculate_unconstrained_black_pawn_attack_maps(self) -> Self {
        self.shift_move(ChessDirection::DownLeft) | self.shift_move(ChessDirection::DownRight)
    }
}

impl KnightAttackMaps for u64 {
    fn calculate_unconstrained_knight_maps(self) -> Self {
        self.shift_move(ChessDirection::KnightOne)
            | self.shift_move(ChessDirection::KnightTwo)
            | self.shift_move(ChessDirection::KnightFour)
            | self.shift_move(ChessDirection::KnightFive)
            | self.shift_move(ChessDirection::KnightSeven)
            | self.shift_move(ChessDirection::KnightEight)
            | self.shift_move(ChessDirection::KnightTen)
            | self.shift_move(ChessDirection::KnightEleven)
    }
}

impl BishopAttackMaps for u64 {
    fn calculate_unconstrained_bishop_attack_maps(self) -> Self {
        let up_right: u64 = (0..7).fold(self.shift_move(ChessDirection::UpRight), |current, _| {
            current | current.shift_move(ChessDirection::UpRight)
        });
        let down_right: u64 = (0..7)
            .fold(self.shift_move(ChessDirection::DownRight), |current, _| {
                current | current.shift_move(ChessDirection::DownRight)
            });
        let down_left: u64 = (0..7)
            .fold(self.shift_move(ChessDirection::DownLeft), |current, _| {
                current | current.shift_move(ChessDirection::DownLeft)
            });
        let up_left: u64 = (0..7).fold(self.shift_move(ChessDirection::UpLeft), |current, _| {
            current | current.shift_move(ChessDirection::UpLeft)
        });
        up_right | down_right | down_left | up_left
    }
}

impl RookAttackMaps for u64 {
    fn calculate_unconstrained_rook_attack_maps(self) -> Self {
        let up: u64 = (0..7).fold(self.shift_move(ChessDirection::Up), |current, _| {
            current | current.shift_move(ChessDirection::Up)
        });
        let right: u64 = (0..7).fold(self.shift_move(ChessDirection::Right), |current, _| {
            current | current.shift_move(ChessDirection::Right)
        });
        let down: u64 = (0..7).fold(self.shift_move(ChessDirection::Down), |current, _| {
            current | current.shift_move(ChessDirection::Down)
        });
        let left: u64 = (0..7).fold(self.shift_move(ChessDirection::Left), |current, _| {
            current | current.shift_move(ChessDirection::Left)
        });
        up | right | down | left
    }
}

impl QueenAttackMaps for u64 {
    fn calculate_unconstrained_queen_attack_maps(self) -> Self {
        let up: u64 = (0..7).fold(self.shift_move(ChessDirection::Up), |current, _| {
            current | current.shift_move(ChessDirection::Up)
        });
        let up_right: u64 = (0..7).fold(self.shift_move(ChessDirection::UpRight), |current, _| {
            current | current.shift_move(ChessDirection::UpRight)
        });
        let right: u64 = (0..7).fold(self.shift_move(ChessDirection::Right), |current, _| {
            current | current.shift_move(ChessDirection::Right)
        });
        let down_right: u64 = (0..7)
            .fold(self.shift_move(ChessDirection::DownRight), |current, _| {
                current | current.shift_move(ChessDirection::DownRight)
            });
        let down: u64 = (0..7).fold(self.shift_move(ChessDirection::Down), |current, _| {
            current | current.shift_move(ChessDirection::Down)
        });
        let down_left: u64 = (0..7)
            .fold(self.shift_move(ChessDirection::DownLeft), |current, _| {
                current | current.shift_move(ChessDirection::DownLeft)
            });
        let left: u64 = (0..7).fold(self.shift_move(ChessDirection::Left), |current, _| {
            current | current.shift_move(ChessDirection::Left)
        });
        let up_left: u64 = (0..7).fold(self.shift_move(ChessDirection::UpLeft), |current, _| {
            current | current.shift_move(ChessDirection::UpLeft)
        });
        up | up_right | right | down_right | down | down_left | left | up_left
    }
}

impl KingAttackMaps for u64 {
    fn calculate_unconstrained_king_attack_maps(self) -> Self {
        self.shift_move(ChessDirection::Up)
            | self.shift_move(ChessDirection::UpRight)
            | self.shift_move(ChessDirection::Right)
            | self.shift_move(ChessDirection::DownRight)
            | self.shift_move(ChessDirection::Down)
            | self.shift_move(ChessDirection::DownLeft)
            | self.shift_move(ChessDirection::Left)
            | self.shift_move(ChessDirection::UpLeft)
    }
}

#[cfg(test)]
mod test {
    mod attack_maps_for_u64 {
        use crate::chess_state::{
            coordinates::{XCoordinate::*, YCoordinate::*},
            moves::attack_maps::{
                BishopAttackMaps, BlackPawnAttackMaps, KnightAttackMaps, WhitePawnAttackMaps,
            },
        };

        #[test]
        fn white_pawn_attack_map_correct_when_trait_used_on_u64() {
            // arrange
            let pawn = (D as u64) & (Four as u64);
            let expected_attack_map = ((C as u64) & (Five as u64)) | ((E as u64) & (Five as u64));

            // act
            let attack_map = pawn.calculate_unconstrained_white_pawn_attack_maps();

            // assert
            assert_eq!(attack_map, expected_attack_map)
        }

        #[test]
        fn black_pawn_attack_map_correct_when_trait_used_on_u64() {
            // arrange
            let pawn = (D as u64) & (Five as u64);
            let expected_attack_map = ((C as u64) & (Four as u64)) | ((E as u64) & (Four as u64));

            // act
            let attack_map = pawn.calculate_unconstrained_black_pawn_attack_maps();

            // assert
            assert_eq!(attack_map, expected_attack_map)
        }

        #[test]
        fn knight_attack_map_correct_when_trait_used_on_u64() {
            // arrange
            let knight = (C as u64) & (Three as u64);
            let expected_attack_map = ((D as u64) & (Five as u64))
                | ((E as u64) & (Four as u64))
                | ((E as u64) & (Two as u64))
                | ((D as u64) & (One as u64))
                | ((B as u64) & (One as u64))
                | ((A as u64) & (Two as u64))
                | ((A as u64) & (Four as u64))
                | ((B as u64) & (Five as u64));

            // act
            let attack_map = knight.calculate_unconstrained_knight_maps();

            // assert
            assert_eq!(attack_map, expected_attack_map)
        }

        #[test]
        fn bishop_attack_map_correct_when_trait_used_on_u64() {
            // arrange
            let bishop = (F as u64) & (Four as u64);
            let expected_attack_map = ((G as u64) & (Five as u64))
                | ((H as u64) & (Six as u64))
                | ((G as u64) & (Three as u64))
                | ((H as u64) & (Two as u64))
                | ((E as u64) & (Three as u64))
                | ((D as u64) & (Two as u64))
                | ((C as u64) & (One as u64))
                | ((E as u64) & (Five as u64))
                | ((D as u64) & (Six as u64))
                | ((C as u64) & (Seven as u64))
                | ((B as u64) & (Eight as u64));

            // act
            let attack_map = bishop.calculate_unconstrained_bishop_attack_maps();

            // assert
            assert_eq!(attack_map, expected_attack_map)
        }
    }
}
