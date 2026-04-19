//! Player orientation system for hollow sphere world.
//!
//! Manages player orientation relative to the curved inner surface
//! and disorientation effects when in freefall.

use glam::Vec3;

/// Player orientation state.
#[derive(Debug, Clone, Copy)]
pub struct PlayerOrientation {
    /// Up vector relative to player (surface normal).
    pub up_vector: Vec3,
    /// Disorientation level from 0.0 (stable) to 1.0 (fully disoriented).
    pub disorientation_level: f32,
    /// Whether the player is currently on a surface.
    pub is_on_surface: bool,
}

impl PlayerOrientation {
    /// Create a new player orientation.
    #[must_use]
    pub fn new(up_vector: Vec3) -> Self {
        Self {
            up_vector: up_vector.normalize_or_zero(),
            disorientation_level: 0.0,
            is_on_surface: true,
        }
    }
}

impl Default for PlayerOrientation {
    fn default() -> Self {
        Self::new(Vec3::Y)
    }
}

/// Effects caused by disorientation.
#[derive(Debug, Clone, Copy)]
pub struct OrientationEffects {
    /// Screen wobble intensity (0.0 to 1.0).
    pub screen_wobble: f32,
    /// Input direction offset in radians.
    pub input_offset: f32,
    /// Whether nausea effect is active.
    pub nausea: bool,
}

impl OrientationEffects {
    /// No disorientation effects.
    #[must_use]
    pub fn none() -> Self {
        Self {
            screen_wobble: 0.0,
            input_offset: 0.0,
            nausea: false,
        }
    }
}

impl Default for OrientationEffects {
    fn default() -> Self {
        Self::none()
    }
}

/// Calculate the up vector for a position on the sphere surface.
///
/// Returns the normalized direction from sphere center to position,
/// which is the surface normal pointing "up" for inverted gravity.
#[must_use]
pub fn orient_to_surface(position: Vec3, sphere_center: Vec3) -> Vec3 {
    let direction = position - sphere_center;
    direction.normalize_or_zero()
}

/// Update player orientation based on surface contact.
///
/// - On surface: disorientation decreases at 20% per second (min 0.0)
/// - Off surface (freefall): disorientation increases at 10% per second (max 1.0)
pub fn update_orientation(orientation: &mut PlayerOrientation, is_on_surface: bool, delta: f32) {
    orientation.is_on_surface = is_on_surface;

    if is_on_surface {
        // Recovery: decrease disorientation by 20% per second
        orientation.disorientation_level = (orientation.disorientation_level - 0.2 * delta).max(0.0);
    } else {
        // Freefall: increase disorientation by 10% per second
        orientation.disorientation_level = (orientation.disorientation_level + 0.1 * delta).min(1.0);
    }
}

/// Calculate disorientation effects based on disorientation level.
///
/// - Level 0.0-0.3: no effects
/// - Level 0.3-0.7: mild wobble
/// - Level 0.7-1.0: heavy wobble, random input offset, nausea
#[must_use]
pub fn disorientation_effects(level: f32) -> OrientationEffects {
    if level < 0.3 {
        // No effects
        OrientationEffects::none()
    } else if level < 0.7 {
        // Mild effects: some wobble
        let wobble_t = (level - 0.3) / 0.4;
        OrientationEffects {
            screen_wobble: wobble_t * 0.3,
            input_offset: 0.0,
            nausea: false,
        }
    } else {
        // Heavy effects: full wobble, input offset, nausea
        let wobble_t = (level - 0.7) / 0.3;
        OrientationEffects {
            screen_wobble: 0.3 + wobble_t * 0.7,
            input_offset: wobble_t * 0.5,
            nausea: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CENTER: Vec3 = Vec3::ZERO;

    #[test]
    fn test_orient_to_surface_positive_x() {
        let position = Vec3::new(100.0, 0.0, 0.0);
        let up = orient_to_surface(position, CENTER);
        assert!((up.x - 1.0).abs() < 0.001);
        assert!(up.y.abs() < 0.001);
        assert!(up.z.abs() < 0.001);
    }

    #[test]
    fn test_orient_to_surface_normalized() {
        let position = Vec3::new(100.0, 200.0, 300.0);
        let up = orient_to_surface(position, CENTER);
        let length = up.length();
        assert!((length - 1.0).abs() < 0.001, "Up vector should be normalized");
    }

    #[test]
    fn test_orient_to_surface_at_center() {
        let up = orient_to_surface(CENTER, CENTER);
        assert!(up.length() < 0.001, "At center should return zero vector");
    }

    #[test]
    fn test_update_orientation_on_surface_decreases_disorientation() {
        let mut orientation = PlayerOrientation::new(Vec3::Y);
        orientation.disorientation_level = 0.5;

        update_orientation(&mut orientation, true, 1.0);

        assert!((orientation.disorientation_level - 0.3).abs() < 0.001);
    }

    #[test]
    fn test_update_orientation_freefall_increases_disorientation() {
        let mut orientation = PlayerOrientation::new(Vec3::Y);
        orientation.disorientation_level = 0.5;

        update_orientation(&mut orientation, false, 1.0);

        assert!((orientation.disorientation_level - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_update_orientation_clamps_min() {
        let mut orientation = PlayerOrientation::new(Vec3::Y);
        orientation.disorientation_level = 0.1;

        update_orientation(&mut orientation, true, 1.0);

        assert!(orientation.disorientation_level >= 0.0);
    }

    #[test]
    fn test_update_orientation_clamps_max() {
        let mut orientation = PlayerOrientation::new(Vec3::Y);
        orientation.disorientation_level = 0.95;

        update_orientation(&mut orientation, false, 1.0);

        assert!(orientation.disorientation_level <= 1.0);
    }

    #[test]
    fn test_disorientation_effects_low_level() {
        let effects = disorientation_effects(0.2);
        assert!((effects.screen_wobble - 0.0).abs() < 0.001);
        assert!((effects.input_offset - 0.0).abs() < 0.001);
        assert!(!effects.nausea);
    }

    #[test]
    fn test_disorientation_effects_medium_level() {
        let effects = disorientation_effects(0.5);
        assert!(effects.screen_wobble > 0.0);
        assert!((effects.input_offset - 0.0).abs() < 0.001);
        assert!(!effects.nausea);
    }

    #[test]
    fn test_disorientation_effects_high_level() {
        let effects = disorientation_effects(0.9);
        assert!(effects.screen_wobble > 0.3);
        assert!(effects.input_offset > 0.0);
        assert!(effects.nausea);
    }
}
