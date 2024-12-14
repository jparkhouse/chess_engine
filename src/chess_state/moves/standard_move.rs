use crate::chess_state::{chess_pieces::PieceEnum, coordinate_point::CoordinatePosition};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct StandardMove {
    pub(crate) start_position: CoordinatePosition,
    pub(crate) end_position: CoordinatePosition,
    pub(crate) piece: PieceEnum,
    pub(crate) en_passant_target: Option<CoordinatePosition>,
    pub(crate) promotion: Option<PieceEnum>,
    pub(crate) takes: Option<(CoordinatePosition, PieceEnum)>,
}

impl StandardMove {
    pub(crate) fn new(
        start_position: CoordinatePosition,
        end_position: CoordinatePosition,
        piece: PieceEnum,
        en_passant_target: Option<CoordinatePosition>,
        promotion: Option<PieceEnum>,
        takes: Option<(CoordinatePosition, PieceEnum)>,
    ) -> Self {
        Self {
            start_position: start_position,
            end_position: end_position,
            promotion: promotion,
            takes: takes,
            piece: piece,
            en_passant_target: en_passant_target,
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
        format!(
            "{}{}{}{}",
            self.start_position, x, self.end_position, promotion
        )
    }
}