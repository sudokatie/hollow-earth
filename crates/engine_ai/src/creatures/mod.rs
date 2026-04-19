//! Creature behavior systems.

mod hostile;
mod passive;

pub use hostile::{
    BiomeRestriction, HostileAI, HostileAction, HostileDrop, HostileState, HollowEarthHostile,
    HollowEarthHostileType, SpecialAbility, GRAVITY_MANIP_FORCE, GRAVITY_MANIP_RANGE,
    RADIATION_AURA_DPS, RADIATION_AURA_RANGE, SPORE_CLOUD_DAMAGE, SPORE_CLOUD_RANGE,
};
pub use passive::{
    HollowEarthPassive, HollowEarthPassiveType, PassiveAI, PassiveBehavior, PassiveDrop,
    PassiveState,
};
