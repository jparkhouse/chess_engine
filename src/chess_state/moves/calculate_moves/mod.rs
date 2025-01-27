mod diagonal_moves;
mod king_moves;
mod knight_moves;
mod pawn_moves;
mod pinned_to_king;
mod straight_moves;

#[macro_export]
macro_rules! log_move_generation {
    ($expr:expr, $function_name:expr) => {
        match $expr {
            Ok(val) => {
                log::info!(
                    "Successfully calculated {} {} moves",
                    val.len(),
                    $function_name
                );
                log::debug!("Calculated {} moves: {:?}", $function_name, val);
                val
            }
            Err(err) => {
                log::error!("Error while calculating {}: {}", $function_name, err);
                return Err(err);
            }
        }
    };
}
