//! Sphere rendering for hollow earth world.
//!
//! Provides core glow, interior sky, radial shadows, and curvature fog
//! systems for rendering the inverted sphere environment.

mod core_glow;
mod curvature_fog;
mod interior_sky;
mod radial_shadows;

pub use core_glow::{
    core_brightness, core_color, update_core, CoreState, CORE_RADIUS,
};
pub use curvature_fog::{
    combined_fog_factor, curvature_fade_start, curvature_fog_intensity, is_past_horizon,
    HORIZON_ARC,
};
pub use interior_sky::{
    are_features_visible, calculate_fog_color, calculate_visibility, fog_intensity, InteriorSky,
    MAX_VISIBILITY,
};
pub use radial_shadows::{is_in_shadow, shadow_direction, shadow_length};
