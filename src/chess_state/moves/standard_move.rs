use crate::chess_state::{
    chess_pieces::PieceEnum, coordinate_point::CoordinatePosition, moves::shared::CheckType,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct StandardMove {
    pub(crate) start_position: CoordinatePosition,
    pub(crate) end_position: CoordinatePosition,
    pub(crate) piece: PieceEnum,
    pub(crate) en_passant_target: Option<CoordinatePosition>,
    pub(crate) promotion: Option<PieceEnum>,
    pub(crate) takes: Option<(CoordinatePosition, PieceEnum)>,
    pub(crate) check: CheckType,
}

impl StandardMove {
    pub(crate) fn new(
        start_position: CoordinatePosition,
        end_position: CoordinatePosition,
        piece: PieceEnum,
        en_passant_target: Option<CoordinatePosition>,
        promotion: Option<PieceEnum>,
        takes: Option<(CoordinatePosition, PieceEnum)>,
        check: CheckType,
    ) -> Self {
        Self {
            start_position,
            end_position,
            promotion,
            takes,
            piece,
            en_passant_target,
            check,
        }
    }

    pub(crate) fn get_uci_move(&self) -> String {
        let x = match self.takes {
            Some(_) => "x",
            None => "",
        };
        let promotion = match self.promotion {
            Some(piece) => format!("={}", piece),
            None => "".to_string(),
        };
        let check = match self.check {
            CheckType::None => "",
            CheckType::Check => "+",
            CheckType::Checkmate => "#",
        };
        format!(
            "{}{}{}{}{}",
            self.start_position, x, self.end_position, promotion, check
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::chess_state::moves::shared::CheckType;

    use super::{CoordinatePosition, PieceEnum, StandardMove};

    #[test]
    fn generates_correct_uci_when_a_simple_move_is_executed() {
        // arrange
        let test_move = StandardMove {
            start_position: CoordinatePosition::from_str("e2").expect("valid coordinate"),
            end_position: CoordinatePosition::from_str("e4").expect("valid coordinate"),
            piece: PieceEnum::WhitePawn,
            en_passant_target: None,
            promotion: None,
            takes: None,
            check: CheckType::None,
        };

        // act
        let test_move_str = test_move.get_uci_move();

        // assert
        assert_eq!(test_move_str, "e2e4".to_string())
    }

    #[test]
    fn generates_correct_uci_when_a_capturing_move_is_executed() {
        // arrange
        let test_move = StandardMove {
            start_position: CoordinatePosition::from_str("e4").expect("valid coordinate"),
            end_position: CoordinatePosition::from_str("d5").expect("valid coordinate"),
            piece: PieceEnum::WhitePawn,
            en_passant_target: None,
            promotion: None,
            takes: Some((
                CoordinatePosition::from_str("d5").expect("valid coordinate"),
                PieceEnum::BlackPawn,
            )),
            check: CheckType::None,
        };

        // act
        let test_move_str = test_move.get_uci_move();

        // assert
        assert_eq!(test_move_str, "e4xd5".to_string())
    }

    #[test]
    fn generates_correct_uci_when_a_promotion_move_is_executed() {
        // arrange
        let test_move = StandardMove {
            start_position: CoordinatePosition::from_str("e7").expect("valid coordinate"),
            end_position: CoordinatePosition::from_str("e8").expect("valid coordinate"),
            piece: PieceEnum::WhitePawn,
            en_passant_target: None,
            promotion: Some(PieceEnum::WhiteQueen),
            takes: None,
            check: CheckType::None,
        };

        // act
        let test_move_str = test_move.get_uci_move();

        // assert
        assert_eq!(test_move_str, "e7e8=Q".to_string())
    }

    #[test]
    fn generates_correct_uci_when_a_promotion_and_capture_move_is_executed() {
        // arrange
        let test_move = StandardMove {
            start_position: CoordinatePosition::from_str("e2").expect("valid coordinate"),
            end_position: CoordinatePosition::from_str("d1").expect("valid coordinate"),
            piece: PieceEnum::BlackPawn,
            en_passant_target: None,
            promotion: Some(PieceEnum::BlackQueen),
            takes: Some((
                CoordinatePosition::from_str("d1").expect("valid coordinate"),
                PieceEnum::WhiteQueen,
            )),
            check: CheckType::None,
        };

        // act
        let test_move_str = test_move.get_uci_move();

        // assert
        assert_eq!(test_move_str, "e2xd1=q".to_string())
    }

    #[test]
    fn generates_correct_uci_when_a_checking_move_is_executed() {
        // arrange
        let test_move = StandardMove {
            start_position: CoordinatePosition::from_str("h6").expect("valid coordinate"),
            end_position: CoordinatePosition::from_str("g7").expect("valid coordinate"),
            piece: PieceEnum::BlackBishop,
            en_passant_target: None,
            promotion: None,
            takes: None,
            check: CheckType::Check,
        };

        // act
        let test_move_str = test_move.get_uci_move();

        // assert
        assert_eq!(test_move_str, "h6g7+".to_string())
    }

    #[test]
    fn generates_correct_uci_when_a_checkmate_move_is_executed() {
        // arrange
        let test_move = StandardMove {
            start_position: CoordinatePosition::from_str("d8").expect("valid coordinate"),
            end_position: CoordinatePosition::from_str("d3").expect("valid coordinate"),
            piece: PieceEnum::BlackQueen,
            en_passant_target: None,
            promotion: None,
            takes: None,
            check: CheckType::Checkmate,
        };

        // act
        let test_move_str = test_move.get_uci_move();

        // assert
        assert_eq!(test_move_str, "d8d3#".to_string())
    }

    #[test]
    fn generates_correct_uci_when_a_checkmate_move_with_capture_is_executed() {
        // arrange
        let test_move = StandardMove {
            start_position: CoordinatePosition::from_str("d8").expect("valid coordinate"),
            end_position: CoordinatePosition::from_str("d3").expect("valid coordinate"),
            piece: PieceEnum::BlackQueen,
            en_passant_target: None,
            promotion: None,
            takes: Some((
                CoordinatePosition::from_str("d3").expect("valid coordinate"),
                PieceEnum::WhitePawn,
            )),
            check: CheckType::Checkmate,
        };

        // act
        let test_move_str = test_move.get_uci_move();

        // assert
        assert_eq!(test_move_str, "d8xd3#".to_string())
    }
}
