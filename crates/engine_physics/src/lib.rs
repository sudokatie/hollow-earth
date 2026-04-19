//! Physics system for the Lattice game engine.
//!
//! Provides collision detection, rigid body dynamics, and character physics.

pub mod collision;
pub mod raycast;
pub mod simulation;
pub mod sphere;

pub use sphere::{
    calculate_gravity, cartesian_to_spherical, distance_from_center, gravity_magnitude,
    is_on_surface, local_frame, spherical_to_cartesian, surface_distance, tangent_direction,
    CORE_RADIUS, SPHERE_RADIUS,
};
