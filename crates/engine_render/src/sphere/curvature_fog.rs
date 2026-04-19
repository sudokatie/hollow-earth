//! Curvature-based fog for hollow earth world.
//!
//! In a hollow sphere, the horizon is curved upward. This module handles
//! fog that follows the curvature, preventing visibility past the horizon.

use std::f32::consts::PI;

/// Radius of the hollow sphere world in blocks.
const SPHERE_RADIUS: f32 = 4096.0;

/// Arc distance to the horizon (half the sphere circumference).
pub const HORIZON_ARC: f32 = PI * SPHERE_RADIUS;

/// Calculate fog intensity based on arc distance along the sphere surface.
///
/// Returns 0.0 at the player position, 1.0 at max visibility.
#[must_use]
pub fn curvature_fog_intensity(arc_distance: f32, max_visibility: f32) -> f32 {
    if max_visibility <= 0.0 {
        return 1.0;
    }

    let intensity = arc_distance / max_visibility;
    intensity.clamp(0.0, 1.0)
}

/// Check if a point is past the horizon (not visible due to curvature).
///
/// In a hollow sphere, the horizon is at half the circumference.
#[must_use]
pub fn is_past_horizon(arc_distance: f32) -> bool {
    arc_distance > HORIZON_ARC
}

/// Calculate the arc distance where curvature starts to obscure visibility.
///
/// Returns the distance at which objects start to dip below the visual horizon.
#[must_use]
pub fn curvature_fade_start(max_visibility: f32) -> f32 {
    // Start fading at 70% of max visibility or horizon, whichever is smaller
    (max_visibility * 0.7).min(HORIZON_ARC * 0.8)
}

/// Calculate fog factor considering both distance and curvature.
///
/// Combines linear distance fog with horizon cutoff.
#[must_use]
pub fn combined_fog_factor(arc_distance: f32, max_visibility: f32) -> f32 {
    if is_past_horizon(arc_distance) {
        return 1.0;
    }

    let distance_fog = curvature_fog_intensity(arc_distance, max_visibility);
    let horizon_factor = if arc_distance > HORIZON_ARC * 0.8 {
        let t = (arc_distance - HORIZON_ARC * 0.8) / (HORIZON_ARC * 0.2);
        t.clamp(0.0, 1.0)
    } else {
        0.0
    };

    (distance_fog + horizon_factor).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizon_arc_constant() {
        let expected = PI * 4096.0;
        assert!((HORIZON_ARC - expected).abs() < 0.001);
    }

    #[test]
    fn test_fog_intensity_at_player() {
        let intensity = curvature_fog_intensity(0.0, 2048.0);
        assert!((intensity).abs() < 0.001, "Fog at player should be zero");
    }

    #[test]
    fn test_fog_intensity_at_max_visibility() {
        let intensity = curvature_fog_intensity(2048.0, 2048.0);
        assert!((intensity - 1.0).abs() < 0.001, "Fog at max visibility should be 1.0");
    }

    #[test]
    fn test_fog_intensity_linear() {
        let half = curvature_fog_intensity(1024.0, 2048.0);
        assert!(
            (half - 0.5).abs() < 0.001,
            "Fog should be linear: expected 0.5, got {half}"
        );
    }

    #[test]
    fn test_is_past_horizon_false() {
        assert!(!is_past_horizon(1000.0), "1000 blocks should be visible");
        assert!(!is_past_horizon(HORIZON_ARC - 1.0), "Just before horizon should be visible");
    }

    #[test]
    fn test_is_past_horizon_true() {
        assert!(is_past_horizon(HORIZON_ARC + 1.0), "Past horizon should not be visible");
        assert!(is_past_horizon(20000.0), "Very far should not be visible");
    }
}
