//! Interior sky rendering for hollow earth world.
//!
//! Unlike a normal sky, the interior sky shows the opposite side of the
//! sphere, with visibility affected by core brightness and atmospheric effects.

/// Maximum visibility distance in blocks.
pub const MAX_VISIBILITY: f32 = 2048.0;

/// Interior sky state for rendering.
#[derive(Debug, Clone, Copy)]
pub struct InteriorSky {
    /// Current visibility distance in blocks.
    pub visibility: f32,
    /// Fog color (RGB).
    pub fog_color: [f32; 3],
    /// Whether surface features on the far side are visible.
    pub surface_features_visible: bool,
}

impl Default for InteriorSky {
    fn default() -> Self {
        Self {
            visibility: MAX_VISIBILITY,
            fog_color: [0.15, 0.15, 0.18],
            surface_features_visible: true,
        }
    }
}

impl InteriorSky {
    /// Create a new interior sky state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the sky state based on core brightness and weather.
    #[must_use]
    pub fn update(core_brightness: f32, weather_factor: f32) -> Self {
        let visibility = calculate_visibility(core_brightness, weather_factor);
        let fog_color = calculate_fog_color(core_brightness);
        let surface_features_visible = visibility > MAX_VISIBILITY * 0.3;

        Self {
            visibility,
            fog_color,
            surface_features_visible,
        }
    }
}

/// Calculate visibility based on core brightness and weather.
///
/// Higher core brightness = better visibility.
/// Higher weather factor = worse visibility (fog, storms).
#[must_use]
pub fn calculate_visibility(core_brightness: f32, weather_factor: f32) -> f32 {
    let base_visibility = MAX_VISIBILITY * core_brightness.clamp(0.2, 2.0);
    let weather_reduction = 1.0 - weather_factor.clamp(0.0, 1.0) * 0.8;

    (base_visibility * weather_reduction).clamp(100.0, MAX_VISIBILITY)
}

/// Calculate fog intensity using exponential decay.
///
/// Returns fog factor (0.0 = no fog, 1.0 = fully fogged).
#[must_use]
pub fn fog_intensity(distance: f32, visibility: f32) -> f32 {
    if visibility <= 0.0 {
        return 1.0;
    }

    let decay_rate = 2.0 / visibility;
    let fog = 1.0 - (-distance * decay_rate).exp();

    fog.clamp(0.0, 1.0)
}

/// Calculate fog color based on core brightness.
///
/// Dark gray that shifts with core brightness.
#[must_use]
pub fn calculate_fog_color(core_brightness: f32) -> [f32; 3] {
    // Base dark gray
    let base = 0.1;

    // Add some brightness from the core
    let core_contribution = core_brightness.clamp(0.0, 1.0) * 0.15;

    // Slightly warmer tint when brighter
    let warmth = core_brightness.clamp(0.0, 1.0) * 0.05;

    [
        base + core_contribution + warmth,
        base + core_contribution,
        base + core_contribution - warmth * 0.5,
    ]
}

/// Check if surface features are visible at a given distance.
#[must_use]
pub fn are_features_visible(distance: f32, visibility: f32) -> bool {
    distance < visibility * 0.7
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_interior_sky() {
        let sky = InteriorSky::default();
        assert!((sky.visibility - MAX_VISIBILITY).abs() < 0.001);
        assert!(sky.surface_features_visible);
    }

    #[test]
    fn test_max_visibility_constant() {
        assert!((MAX_VISIBILITY - 2048.0).abs() < 0.001);
    }

    #[test]
    fn test_visibility_full_brightness() {
        let vis = calculate_visibility(1.0, 0.0);
        assert!(
            (vis - MAX_VISIBILITY).abs() < 0.001,
            "Full brightness, no weather should give max visibility"
        );
    }

    #[test]
    fn test_visibility_low_brightness() {
        let vis = calculate_visibility(0.2, 0.0);
        assert!(vis < MAX_VISIBILITY * 0.5, "Low brightness should reduce visibility");
        assert!(vis >= 100.0, "Visibility should have a minimum");
    }

    #[test]
    fn test_visibility_bad_weather() {
        let vis_clear = calculate_visibility(1.0, 0.0);
        let vis_storm = calculate_visibility(1.0, 1.0);

        assert!(
            vis_storm < vis_clear,
            "Bad weather should reduce visibility"
        );
    }

    #[test]
    fn test_fog_intensity_at_distance() {
        let intensity_near = fog_intensity(100.0, MAX_VISIBILITY);
        let intensity_far = fog_intensity(1500.0, MAX_VISIBILITY);

        assert!(intensity_near < 0.2, "Near fog should be minimal");
        assert!(intensity_far > intensity_near, "Fog should increase with distance");
    }

    #[test]
    fn test_fog_intensity_exponential() {
        let i1 = fog_intensity(500.0, MAX_VISIBILITY);
        let i2 = fog_intensity(1000.0, MAX_VISIBILITY);
        let i3 = fog_intensity(1500.0, MAX_VISIBILITY);

        // Exponential decay means differences should get smaller
        let diff1 = i2 - i1;
        let diff2 = i3 - i2;
        assert!(diff2 < diff1, "Fog increase should slow down (exponential decay)");
    }

    #[test]
    fn test_fog_color_shifts_with_brightness() {
        let color_dim = calculate_fog_color(0.2);
        let color_bright = calculate_fog_color(1.0);

        assert!(
            color_bright[0] > color_dim[0],
            "Bright core should lighten fog"
        );
    }
}
