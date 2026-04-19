//! Sphere curvature math for navigation and orientation.
//!
//! Provides utilities for calculating distances along the sphere surface,
//! coordinate conversions, and local reference frames.

use glam::Vec3;
use std::f32::consts::TAU;

/// Calculate the arc length (surface distance) between two points on a sphere.
///
/// Uses the great circle distance formula: arc_length = radius * angle_between.
#[must_use]
pub fn surface_distance(pos_a: Vec3, pos_b: Vec3, radius: f32) -> f32 {
    let dir_a = pos_a.normalize_or_zero();
    let dir_b = pos_b.normalize_or_zero();

    let dot = dir_a.dot(dir_b).clamp(-1.0, 1.0);
    let angle = dot.acos();

    radius * angle
}

/// Calculate a local orthonormal reference frame at a position on the sphere.
///
/// Returns (up, north, east) vectors where:
/// - up: points outward from center (radial direction)
/// - north: tangent direction toward the sphere's north pole (Y-axis)
/// - east: perpendicular to up and north (right-hand rule)
#[must_use]
pub fn local_frame(position: Vec3, center: Vec3) -> (Vec3, Vec3, Vec3) {
    let up = (position - center).normalize_or_zero();

    // Handle degenerate case at poles
    let world_up = Vec3::Y;

    // If we're at or near a pole, use X as the reference for north
    let north = if up.dot(world_up).abs() > 0.999 {
        let reference = Vec3::X;
        let east = up.cross(reference).normalize_or_zero();
        east.cross(up).normalize_or_zero()
    } else {
        // Project world_up onto tangent plane and normalize
        let projected = world_up - up * up.dot(world_up);
        projected.normalize_or_zero()
    };

    let east = up.cross(north).normalize_or_zero();

    (up, north, east)
}

/// Convert spherical coordinates to Cartesian coordinates.
///
/// - lat: latitude in radians (-PI/2 to PI/2, 0 at equator)
/// - lon: longitude in radians (0 to TAU)
/// - radius: distance from center
///
/// Assumes Y is up (north pole at +Y).
#[must_use]
pub fn spherical_to_cartesian(lat: f32, lon: f32, radius: f32) -> Vec3 {
    let cos_lat = lat.cos();
    Vec3::new(
        radius * cos_lat * lon.sin(),
        radius * lat.sin(),
        radius * cos_lat * lon.cos(),
    )
}

/// Convert Cartesian coordinates to spherical coordinates.
///
/// Returns (latitude, longitude, radius) where:
/// - latitude: radians from equator (-PI/2 to PI/2)
/// - longitude: radians around Y axis (0 to TAU)
/// - radius: distance from center
#[must_use]
pub fn cartesian_to_spherical(position: Vec3, center: Vec3) -> (f32, f32, f32) {
    let relative = position - center;
    let radius = relative.length();

    if radius < f32::EPSILON {
        return (0.0, 0.0, 0.0);
    }

    let lat = (relative.y / radius).clamp(-1.0, 1.0).asin();
    let lon = relative.x.atan2(relative.z);

    // Normalize longitude to 0..TAU
    let lon = if lon < 0.0 { lon + TAU } else { lon };

    (lat, lon, radius)
}

/// Calculate the tangent direction from one point to another along the sphere surface.
///
/// Returns the direction to travel along the surface (great circle path).
#[must_use]
pub fn tangent_direction(from: Vec3, to: Vec3, center: Vec3) -> Vec3 {
    let from_rel = from - center;
    let to_rel = to - center;

    let up = from_rel.normalize_or_zero();

    // Project destination onto tangent plane at 'from'
    let to_direction = to_rel.normalize_or_zero();
    let projected = to_direction - up * up.dot(to_direction);

    projected.normalize_or_zero()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    const CENTER: Vec3 = Vec3::ZERO;
    const RADIUS: f32 = 1000.0;

    #[test]
    fn test_surface_distance_quarter_sphere() {
        let pos_a = Vec3::new(RADIUS, 0.0, 0.0);
        let pos_b = Vec3::new(0.0, RADIUS, 0.0);

        let dist = surface_distance(pos_a, pos_b, RADIUS);
        let expected = RADIUS * FRAC_PI_2; // Quarter circumference

        assert!((dist - expected).abs() < 0.1);
    }

    #[test]
    fn test_surface_distance_same_point() {
        let pos = Vec3::new(RADIUS, 0.0, 0.0);
        let dist = surface_distance(pos, pos, RADIUS);
        assert!(dist.abs() < 0.001);
    }

    #[test]
    fn test_surface_distance_opposite_points() {
        let pos_a = Vec3::new(RADIUS, 0.0, 0.0);
        let pos_b = Vec3::new(-RADIUS, 0.0, 0.0);

        let dist = surface_distance(pos_a, pos_b, RADIUS);
        let expected = RADIUS * PI; // Half circumference

        assert!((dist - expected).abs() < 0.1);
    }

    #[test]
    fn test_local_frame_orthonormal() {
        let pos = Vec3::new(RADIUS, 100.0, 200.0);
        let (up, north, east) = local_frame(pos, CENTER);

        // Check orthogonality
        assert!(up.dot(north).abs() < 0.001);
        assert!(up.dot(east).abs() < 0.001);
        assert!(north.dot(east).abs() < 0.001);

        // Check unit length
        assert!((up.length() - 1.0).abs() < 0.001);
        assert!((north.length() - 1.0).abs() < 0.001);
        assert!((east.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_local_frame_up_points_outward() {
        let pos = Vec3::new(RADIUS, 0.0, 0.0);
        let (up, _, _) = local_frame(pos, CENTER);

        assert!(up.x > 0.99);
    }

    #[test]
    fn test_spherical_to_cartesian_equator() {
        let pos = spherical_to_cartesian(0.0, 0.0, RADIUS);

        assert!(pos.x.abs() < 0.1);
        assert!(pos.y.abs() < 0.1);
        assert!((pos.z - RADIUS).abs() < 0.1);
    }

    #[test]
    fn test_spherical_to_cartesian_north_pole() {
        let pos = spherical_to_cartesian(FRAC_PI_2, 0.0, RADIUS);

        assert!(pos.x.abs() < 0.1);
        assert!((pos.y - RADIUS).abs() < 0.1);
        assert!(pos.z.abs() < 0.1);
    }

    #[test]
    fn test_cartesian_to_spherical_roundtrip() {
        let original = Vec3::new(500.0, 300.0, 700.0);
        let (lat, lon, radius) = cartesian_to_spherical(original, CENTER);
        let recovered = spherical_to_cartesian(lat, lon, radius);

        assert!((original - recovered).length() < 0.1);
    }

    #[test]
    fn test_tangent_direction_normalized() {
        let from = Vec3::new(RADIUS, 0.0, 0.0);
        let to = Vec3::new(0.0, RADIUS, 0.0);

        let tangent = tangent_direction(from, to, CENTER);
        assert!((tangent.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_tangent_direction_perpendicular_to_up() {
        let from = Vec3::new(RADIUS, 0.0, 0.0);
        let to = Vec3::new(0.0, RADIUS, 0.0);

        let tangent = tangent_direction(from, to, CENTER);
        let up = from.normalize();

        assert!(tangent.dot(up).abs() < 0.001);
    }
}
