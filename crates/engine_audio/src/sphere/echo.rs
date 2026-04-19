//! Sphere echo and reverb system for hollow world acoustics.
//!
//! Sound in a hollow sphere behaves uniquely - echoes can travel across
//! the entire diameter, crystals create resonance effects, and the core
//! emits a constant low-frequency hum.

use glam::Vec3;

/// Speed of sound in air (m/s).
pub const ECHO_SPEED_OF_SOUND: f32 = 343.0;

/// Default core hum frequency (Hz).
pub const CORE_HUM_FREQUENCY: f32 = 60.0;

/// Calculate base echo delay for sound traveling across sphere diameter and back.
///
/// delay = 2 * sphere_diameter / speed_of_sound
#[must_use]
pub fn echo_base_delay(sphere_radius: f32) -> f32 {
    let sphere_diameter = sphere_radius * 2.0;
    (2.0 * sphere_diameter) / ECHO_SPEED_OF_SOUND
}

/// Echo properties for a position within the sphere.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphereEcho {
    /// Delay before echo returns (seconds).
    pub echo_delay: f32,
    /// Intensity of reverb effect (0.0-1.0).
    pub reverb_intensity: f32,
    /// Frequency of core hum at this position (Hz).
    pub core_hum_frequency: f32,
}

impl Default for SphereEcho {
    fn default() -> Self {
        Self {
            echo_delay: 0.0,
            reverb_intensity: 0.0,
            core_hum_frequency: CORE_HUM_FREQUENCY,
        }
    }
}

/// Calculate echo properties for a position within the sphere.
///
/// Echo delay is proportional to distance to the opposite surface.
/// Reverb intensity is higher near the center (more surfaces to bounce off).
#[must_use]
pub fn calculate_echo(player_pos: Vec3, sphere_center: Vec3, sphere_radius: f32) -> SphereEcho {
    let to_center = sphere_center - player_pos;
    let distance_from_center = to_center.length();

    // Distance to opposite surface = radius + (radius - distance_from_center)
    // = 2 * radius - distance_from_center
    let distance_to_opposite = (2.0 * sphere_radius - distance_from_center).max(0.0);

    // Echo delay is time for sound to travel to opposite surface and back
    // delay = 2 * distance_to_opposite / speed_of_sound
    let echo_delay = (2.0 * distance_to_opposite) / ECHO_SPEED_OF_SOUND;

    // Reverb intensity is higher near center (more equidistant surfaces)
    // At center: 1.0, at surface: 0.0
    let normalized_distance = (distance_from_center / sphere_radius).clamp(0.0, 1.0);
    let reverb_intensity = 1.0 - normalized_distance;

    SphereEcho {
        echo_delay,
        reverb_intensity,
        core_hum_frequency: CORE_HUM_FREQUENCY,
    }
}

/// Resonance effect from crystals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResonanceEffect {
    /// Frequency of the resonance (Hz).
    pub frequency: f32,
    /// Volume of the effect (0.0-1.0).
    pub volume: f32,
    /// Whether this produces a chime sound.
    pub is_chime: bool,
}

impl Default for ResonanceEffect {
    fn default() -> Self {
        Self {
            frequency: 0.0,
            volume: 0.0,
            is_chime: false,
        }
    }
}

/// Calculate crystal resonance effect based on proximity.
///
/// Crystals emit a resonant frequency that creates chime sounds when close.
#[must_use]
pub fn crystal_resonance(near_crystal: bool, distance: f32) -> ResonanceEffect {
    if !near_crystal {
        return ResonanceEffect::default();
    }

    // Crystal resonance frequency (A4 = 440 Hz is pleasant)
    const CRYSTAL_BASE_FREQUENCY: f32 = 440.0;
    // Max distance for crystal effect
    const CRYSTAL_MAX_DISTANCE: f32 = 20.0;

    if distance > CRYSTAL_MAX_DISTANCE {
        return ResonanceEffect::default();
    }

    // Volume decreases with distance (inverse relationship)
    let normalized_distance = (distance / CRYSTAL_MAX_DISTANCE).clamp(0.0, 1.0);
    let volume = 1.0 - normalized_distance;

    // Chime effect when very close (< 5 blocks)
    let is_chime = distance < 5.0;

    // Frequency shifts slightly based on distance for ethereal effect
    let frequency = CRYSTAL_BASE_FREQUENCY * (1.0 + (1.0 - normalized_distance) * 0.1);

    ResonanceEffect {
        frequency,
        volume,
        is_chime,
    }
}

/// Core hum properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoreHum {
    /// Frequency of the hum (Hz).
    pub frequency: f32,
    /// Volume of the hum (0.0-1.0).
    pub volume: f32,
}

impl Default for CoreHum {
    fn default() -> Self {
        Self {
            frequency: CORE_HUM_FREQUENCY,
            volume: 0.0,
        }
    }
}

/// Calculate core hum volume based on distance from center.
///
/// The core emits a constant 60 Hz hum that increases in volume
/// as the player approaches the center.
#[must_use]
pub fn core_hum(distance_from_center: f32, sphere_radius: f32) -> CoreHum {
    // Hum starts becoming audible at half radius
    const HUM_START_RATIO: f32 = 0.5;

    let hum_start_distance = sphere_radius * HUM_START_RATIO;

    if distance_from_center > hum_start_distance {
        return CoreHum::default();
    }

    // Volume increases as we approach the center
    // At center: 1.0, at hum_start_distance: 0.0
    let normalized = (distance_from_center / hum_start_distance).clamp(0.0, 1.0);
    let volume = 1.0 - normalized;

    CoreHum {
        frequency: CORE_HUM_FREQUENCY,
        volume,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPHERE_RADIUS: f32 = 4096.0;
    const CENTER: Vec3 = Vec3::ZERO;

    #[test]
    fn test_echo_speed_of_sound_constant() {
        assert!((ECHO_SPEED_OF_SOUND - 343.0).abs() < 0.001);
    }

    #[test]
    fn test_echo_base_delay_calculation() {
        // delay = 2 * (2 * radius) / 343 = 4 * 4096 / 343
        let delay = echo_base_delay(SPHERE_RADIUS);
        let expected = (4.0 * SPHERE_RADIUS) / ECHO_SPEED_OF_SOUND;
        assert!((delay - expected).abs() < 0.001);
    }

    #[test]
    fn test_echo_at_surface() {
        let pos = Vec3::new(SPHERE_RADIUS - 1.0, 0.0, 0.0);
        let echo = calculate_echo(pos, CENTER, SPHERE_RADIUS);

        // At surface, distance to opposite is ~2*radius
        // Echo delay should be significant
        assert!(echo.echo_delay > 20.0, "Echo delay at surface should be > 20s");
        // Reverb should be low at surface
        assert!(echo.reverb_intensity < 0.1, "Reverb at surface should be low");
    }

    #[test]
    fn test_echo_at_center() {
        let echo = calculate_echo(CENTER, CENTER, SPHERE_RADIUS);

        // At center, reverb should be maximum
        assert!(
            (echo.reverb_intensity - 1.0).abs() < 0.001,
            "Reverb at center should be 1.0"
        );
    }

    #[test]
    fn test_crystal_resonance_near() {
        let effect = crystal_resonance(true, 3.0);

        assert!(effect.is_chime, "Should be chime when very close");
        assert!(effect.volume > 0.8, "Volume should be high when close");
        assert!(effect.frequency > 440.0, "Frequency should be above base");
    }

    #[test]
    fn test_crystal_resonance_far() {
        let effect = crystal_resonance(true, 25.0);

        assert!(!effect.is_chime, "Should not chime when far");
        assert!(effect.volume < 0.001, "Volume should be zero when beyond range");
    }

    #[test]
    fn test_core_hum_at_center() {
        let hum = core_hum(0.0, SPHERE_RADIUS);

        assert!((hum.frequency - 60.0).abs() < 0.001, "Core hum should be 60 Hz");
        assert!((hum.volume - 1.0).abs() < 0.001, "Volume at center should be 1.0");
    }

    #[test]
    fn test_core_hum_at_surface() {
        let hum = core_hum(SPHERE_RADIUS, SPHERE_RADIUS);

        assert!(hum.volume < 0.001, "Core hum should be silent at surface");
    }
}
