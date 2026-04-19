//! Sphere-specific game mechanics for hollow earth world.
//!
//! Provides radiation, orientation, and other sphere-specific systems.

pub mod orientation;
pub mod radiation;

pub use orientation::{
    disorientation_effects, orient_to_surface, update_orientation, OrientationEffects,
    PlayerOrientation,
};
pub use radiation::{
    calculate_radiation, effective_radiation, radiation_damage, radiation_zone, RadiationLevel,
    RadiationShield, RadiationZone, ShieldType,
};
