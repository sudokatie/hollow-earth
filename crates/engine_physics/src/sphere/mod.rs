//! Sphere physics for hollow earth world.
//!
//! Provides radial gravity and curvature math for the inverted sphere world
//! where players live on the inside surface.

mod curvature;
mod gravity;

pub use curvature::{
    cartesian_to_spherical, local_frame, spherical_to_cartesian, surface_distance,
    tangent_direction,
};
pub use gravity::{
    calculate_gravity, distance_from_center, gravity_magnitude, is_on_surface, CORE_RADIUS,
    SPHERE_RADIUS,
};
