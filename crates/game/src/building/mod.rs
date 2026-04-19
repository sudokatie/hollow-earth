//! Block placement and breaking systems.

mod placement;
mod tether;

pub use placement::BlockInteraction;
pub use tether::{
    apply_force, chain_length, create_tether, is_intact, is_vacuum_breach, snap_tether,
    Tether, TetherAnchor, TetherError, DEFAULT_TENSILE_STRENGTH, MAX_TETHER_LENGTH,
};
