//! Radial shadow system for hollow earth world.
//!
//! Shadows in a hollow sphere radiate outward from the central core,
//! unlike normal worlds where shadows follow a directional sun.

use glam::Vec3;

/// Calculate shadow direction at a surface point.
///
/// In a hollow sphere, shadows point outward from the core center
/// toward the surface.
#[must_use]
pub fn shadow_direction(surface_point: Vec3, core_center: Vec3) -> Vec3 {
    (surface_point - core_center).normalize_or_zero()
}

/// Calculate shadow length based on distance from core and object height.
///
/// Objects closer to the core cast longer shadows due to the angle of light.
/// Objects at the surface cast shorter shadows.
#[must_use]
pub fn shadow_length(distance_from_core: f32, object_height: f32) -> f32 {
    if distance_from_core <= 0.0 || object_height <= 0.0 {
        return 0.0;
    }

    // Shadow length increases as you get closer to the core
    // At the surface (max distance), shadows are shortest
    // Near the core, shadows are longest

    // Simple geometric model: shadow_length = height * (sphere_radius / distance)
    // But we'll use a simpler linear falloff for gameplay purposes
    const SPHERE_RADIUS: f32 = 4096.0;

    let distance_factor = (SPHERE_RADIUS / distance_from_core).clamp(1.0, 10.0);
    object_height * distance_factor
}

/// Check if a point is in shadow from a blocker.
///
/// Returns true if `point` is between `blocker` and the core center,
/// meaning the blocker blocks light from the core.
#[must_use]
pub fn is_in_shadow(point: Vec3, blocker: Vec3, core_center: Vec3) -> bool {
    // Vector from core to blocker
    let core_to_blocker = blocker - core_center;
    let blocker_dist = core_to_blocker.length();

    if blocker_dist < 0.001 {
        return false; // Blocker at core center
    }

    // Vector from core to point
    let core_to_point = point - core_center;
    let point_dist = core_to_point.length();

    // Point must be farther from core than blocker to be shadowed
    if point_dist <= blocker_dist {
        return false;
    }

    // Check if point is in the shadow cone
    let blocker_dir = core_to_blocker.normalize();
    let point_dir = core_to_point.normalize();

    // Dot product tells us if they're in the same direction
    let alignment = blocker_dir.dot(point_dir);

    // Must be well-aligned (within shadow cone)
    alignment > 0.98
}

#[cfg(test)]
mod tests {
    use super::*;

    const CENTER: Vec3 = Vec3::ZERO;

    #[test]
    fn test_shadow_direction_normalized() {
        let surface = Vec3::new(100.0, 200.0, 300.0);
        let dir = shadow_direction(surface, CENTER);

        let length = dir.length();
        assert!((length - 1.0).abs() < 0.001, "Direction should be normalized");
    }

    #[test]
    fn test_shadow_direction_points_outward() {
        let surface = Vec3::new(100.0, 0.0, 0.0);
        let dir = shadow_direction(surface, CENTER);

        assert!(dir.x > 0.99, "Shadow should point outward from core");
    }

    #[test]
    fn test_shadow_length_increases_near_core() {
        let near_core = shadow_length(500.0, 2.0);
        let near_surface = shadow_length(4000.0, 2.0);

        assert!(
            near_core > near_surface,
            "Shadows should be longer near the core"
        );
    }

    #[test]
    fn test_is_in_shadow_basic() {
        let blocker = Vec3::new(500.0, 0.0, 0.0);
        let shadowed_point = Vec3::new(1000.0, 0.0, 0.0);
        let unshadowed_point = Vec3::new(0.0, 500.0, 0.0);

        assert!(
            is_in_shadow(shadowed_point, blocker, CENTER),
            "Point behind blocker should be shadowed"
        );
        assert!(
            !is_in_shadow(unshadowed_point, blocker, CENTER),
            "Point not behind blocker should not be shadowed"
        );
    }

    #[test]
    fn test_is_in_shadow_closer_than_blocker() {
        let blocker = Vec3::new(500.0, 0.0, 0.0);
        let point = Vec3::new(250.0, 0.0, 0.0);

        assert!(
            !is_in_shadow(point, blocker, CENTER),
            "Point closer to core than blocker cannot be shadowed"
        );
    }
}
