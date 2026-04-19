//! Radial gravity system for hollow sphere world.
//!
//! In the inverted sphere world, gravity points outward from the center
//! toward the inner surface where players live.

use glam::Vec3;

/// Radius of the hollow sphere world in blocks.
pub const SPHERE_RADIUS: f32 = 4096.0;

/// Radius of the core (dead zone at center) in blocks.
pub const CORE_RADIUS: f32 = 512.0;

/// Standard gravity magnitude at the surface (m/s^2).
const SURFACE_GRAVITY: f32 = 9.81;

/// Calculate the gravity direction at a position.
///
/// Gravity always points outward from the sphere center toward the surface.
/// This is the opposite of normal planetary gravity since players live on
/// the inside of the sphere.
#[must_use]
pub fn calculate_gravity(position: Vec3, sphere_center: Vec3) -> Vec3 {
    let direction = position - sphere_center;
    direction.normalize_or_zero()
}

/// Calculate gravity magnitude based on distance from center.
///
/// Gravity is 0 at the center and increases linearly to 9.81 at the surface.
/// This creates a gradient where floating near the core has lower gravity.
#[must_use]
pub fn gravity_magnitude(distance_from_center: f32, sphere_radius: f32) -> f32 {
    if distance_from_center <= 0.0 {
        return 0.0;
    }
    let t = (distance_from_center / sphere_radius).clamp(0.0, 1.0);
    SURFACE_GRAVITY * t
}

/// Check if a position is on the sphere surface.
///
/// Returns true if the position is within 1.0 block of the surface.
#[must_use]
pub fn is_on_surface(position: Vec3, center: Vec3, radius: f32) -> bool {
    let dist = distance_from_center(position, center);
    (dist - radius).abs() <= 1.0
}

/// Calculate distance from position to sphere center.
#[must_use]
pub fn distance_from_center(position: Vec3, center: Vec3) -> f32 {
    (position - center).length()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CENTER: Vec3 = Vec3::ZERO;

    #[test]
    fn test_gravity_points_outward() {
        let pos = Vec3::new(100.0, 0.0, 0.0);
        let gravity = calculate_gravity(pos, CENTER);

        assert!(gravity.x > 0.99, "Gravity should point in +X direction");
        assert!(gravity.y.abs() < 0.01);
        assert!(gravity.z.abs() < 0.01);
    }

    #[test]
    fn test_gravity_normalized() {
        let pos = Vec3::new(100.0, 200.0, 300.0);
        let gravity = calculate_gravity(pos, CENTER);

        let length = gravity.length();
        assert!((length - 1.0).abs() < 0.001, "Gravity should be unit vector");
    }

    #[test]
    fn test_gravity_at_center_is_zero() {
        let gravity = calculate_gravity(CENTER, CENTER);
        assert!(gravity.length() < 0.001, "Gravity at center should be zero");
    }

    #[test]
    fn test_gravity_magnitude_at_surface() {
        let mag = gravity_magnitude(SPHERE_RADIUS, SPHERE_RADIUS);
        assert!((mag - 9.81).abs() < 0.001);
    }

    #[test]
    fn test_gravity_magnitude_at_center() {
        let mag = gravity_magnitude(0.0, SPHERE_RADIUS);
        assert!(mag.abs() < 0.001);
    }

    #[test]
    fn test_gravity_magnitude_linear_interpolation() {
        let half_radius = SPHERE_RADIUS / 2.0;
        let mag = gravity_magnitude(half_radius, SPHERE_RADIUS);
        let expected = 9.81 / 2.0;
        assert!((mag - expected).abs() < 0.01);
    }

    #[test]
    fn test_is_on_surface_true() {
        let pos = Vec3::new(SPHERE_RADIUS - 0.5, 0.0, 0.0);
        assert!(is_on_surface(pos, CENTER, SPHERE_RADIUS));
    }

    #[test]
    fn test_is_on_surface_false_too_far() {
        let pos = Vec3::new(SPHERE_RADIUS - 100.0, 0.0, 0.0);
        assert!(!is_on_surface(pos, CENTER, SPHERE_RADIUS));
    }

    #[test]
    fn test_distance_from_center() {
        let pos = Vec3::new(3.0, 4.0, 0.0);
        let dist = distance_from_center(pos, CENTER);
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_sphere_radius_constant() {
        assert!((SPHERE_RADIUS - 4096.0).abs() < 0.001);
        assert!((CORE_RADIUS - 512.0).abs() < 0.001);
    }
}
