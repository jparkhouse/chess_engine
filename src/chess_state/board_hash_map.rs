use std::collections::HashMap;
use thiserror::Error;

use crate::{chess_state::{chess_pieces::PieceEnum, coordinate_point::CoordinatePosition}, shared};

#[derive(Debug, Error)]
enum BoardHashMapError {
    #[error("Tried to insert a piece to filled position {0}, containing {1}, without passing the replace flag")]
    InsertedIntoFilledBoardPositionWithNoReplace(CoordinatePosition, PieceEnum),
}

pub(crate) struct BoardHashMap {
    map: HashMap<u8, PieceEnum>,
}

impl BoardHashMap {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// allows the insertion of a single piece
    pub(crate) fn insert(
        &mut self,
        point: CoordinatePosition,
        piece: PieceEnum,
        replace: bool,
    ) -> Result<(), BoardHashMapError> {
        use BoardHashMapError::InsertedIntoFilledBoardPositionWithNoReplace;

        let key = shared::single_bit_bitmask_to_u8(&point.to_bitmask());
        let current = self.map.get(&key);
        if replace | current.is_none() {
            self.map.insert(key, piece);
            return Ok(());
        } else {
            return Err(InsertedIntoFilledBoardPositionWithNoReplace(
                point,
                current.copied().expect("Was not none"),
            ));
        }
    }

    /// Returns all the positions containing a Piece and the Piece it contains as a (CoordinatePosition, PieceEnum)
    /// tuple.
    pub(crate) fn to_iter<'a>(&'a self) -> impl Iterator<Item = (CoordinatePosition, PieceEnum)> + 'a {
        self.map.iter().map(|(&coord, &piece)| {
            (
                CoordinatePosition::from_bitmask(1u64 << coord)
                    .expect("Came from a CoordinatePosition, so must be in range"),
                piece,
            )
        })
    }
}