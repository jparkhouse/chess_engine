use std::collections::HashMap;
use thiserror::Error;

use crate::{
    chess_state::{chess_pieces::PieceEnum, coordinate_point::CoordinatePosition},
    shared,
};

#[derive(Debug, Error)]
pub(crate) enum BoardHashMapError {
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

    /// allows the insertion of a single piece. Uses the replace flag to help ensure that pieces cannot overlap,
    /// except in cases where this may specifically be required, like captures.
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
            Ok(())
        } else {
            Err(InsertedIntoFilledBoardPositionWithNoReplace(
                point,
                current.copied().expect("Was not none"),
            ))
        }
    }

    /// Returns all the positions containing a Piece and the Piece it contains as a (CoordinatePosition, PieceEnum)
    /// tuple.
    pub(crate) fn to_iter(
        &self,
    ) -> impl Iterator<Item = (CoordinatePosition, PieceEnum)> + '_ {
        self.map.iter().map(|(&coord, &piece)| {
            (
                CoordinatePosition::from_bitmask(1u64 << coord)
                    .expect("Came from a CoordinatePosition, so must be in range"),
                piece,
            )
        })
    }

    /// Allows access to the underlying hashmap for querying individual positions
    pub(crate) fn get(&self, position: CoordinatePosition) -> Option<&PieceEnum> {
        let key = shared::single_bit_bitmask_to_u8(&position.to_bitmask());
        self.map.get(&key)
    }
}

#[cfg(test)]
mod tests {
    mod new {
        use crate::chess_state::board_hash_map::BoardHashMap;

        #[test]
        fn creates_empty_hash_map_when_initialised() {
            // arrange & act
            let new_board_hash_map = BoardHashMap::new();

            // assert
            assert!(new_board_hash_map.map.is_empty())
        }
    }
}
