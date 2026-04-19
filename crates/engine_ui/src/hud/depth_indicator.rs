//! Depth indicator HUD for hollow sphere navigation.
//!
//! Shows the player's depth within the sphere - distance from the
//! inner surface and proximity to the core.

use egui::{Color32, Pos2, Rect, Rounding, Vec2};

/// Height of the depth bar.
const BAR_HEIGHT: f32 = 120.0;

/// Width of the depth bar.
const BAR_WIDTH: f32 = 16.0;

/// Padding around the bar.
const BAR_PADDING: f32 = 4.0;

/// Colors for different depth zones.
const ZONE_SURFACE: Color32 = Color32::from_rgb(80, 180, 80);
const ZONE_MID: Color32 = Color32::from_rgb(180, 180, 80);
const ZONE_DEEP: Color32 = Color32::from_rgb(180, 120, 60);
const ZONE_CORE: Color32 = Color32::from_rgb(200, 60, 60);
const BG_COLOR: Color32 = Color32::from_rgba_premultiplied(20, 20, 20, 180);
const MARKER_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
const TEXT_COLOR: Color32 = Color32::from_rgb(220, 220, 220);

/// Depth zones within the hollow sphere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthZone {
    /// Near the inner surface (safe zone).
    Surface,
    /// Middle depth (moderate danger).
    Mid,
    /// Deep within the sphere (high danger).
    Deep,
    /// Near the core (extreme danger).
    Core,
}

impl DepthZone {
    /// Get the display name for this zone.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            DepthZone::Surface => "Surface",
            DepthZone::Mid => "Mid",
            DepthZone::Deep => "Deep",
            DepthZone::Core => "Core",
        }
    }

    /// Get the color for this zone.
    #[must_use]
    pub fn color(self) -> Color32 {
        match self {
            DepthZone::Surface => ZONE_SURFACE,
            DepthZone::Mid => ZONE_MID,
            DepthZone::Deep => ZONE_DEEP,
            DepthZone::Core => ZONE_CORE,
        }
    }
}

/// Depth indicator state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DepthIndicator {
    /// Distance from sphere center (blocks).
    pub distance_from_center: f32,
    /// Sphere radius (blocks).
    pub sphere_radius: f32,
    /// Core radius (blocks).
    pub core_radius: f32,
    /// Current depth zone.
    pub zone: DepthZone,
}

impl Default for DepthIndicator {
    fn default() -> Self {
        Self {
            distance_from_center: 4096.0,
            sphere_radius: 4096.0,
            core_radius: 512.0,
            zone: DepthZone::Surface,
        }
    }
}

impl DepthIndicator {
    /// Create a new depth indicator.
    #[must_use]
    pub fn new(sphere_radius: f32, core_radius: f32) -> Self {
        Self {
            distance_from_center: sphere_radius,
            sphere_radius,
            core_radius,
            zone: DepthZone::Surface,
        }
    }

    /// Update the depth indicator with current position.
    pub fn update(&mut self, distance_from_center: f32) {
        self.distance_from_center = distance_from_center.clamp(0.0, self.sphere_radius);
        self.zone = calculate_depth_zone(distance_from_center, self.sphere_radius, self.core_radius);
    }

    /// Get the normalized depth (0.0 = surface, 1.0 = core).
    #[must_use]
    pub fn normalized_depth(&self) -> f32 {
        let depth_range = self.sphere_radius - self.core_radius;
        if depth_range <= 0.0 {
            return 0.0;
        }
        let depth = self.sphere_radius - self.distance_from_center;
        (depth / depth_range).clamp(0.0, 1.0)
    }

    /// Get the altitude (distance from surface in blocks).
    #[must_use]
    pub fn altitude(&self) -> f32 {
        self.sphere_radius - self.distance_from_center
    }
}

/// Calculate the depth zone based on distance from center.
#[must_use]
pub fn calculate_depth_zone(distance_from_center: f32, sphere_radius: f32, core_radius: f32) -> DepthZone {
    let depth_range = sphere_radius - core_radius;
    let depth = sphere_radius - distance_from_center;
    let normalized = depth / depth_range;

    if normalized < 0.25 {
        DepthZone::Surface
    } else if normalized < 0.5 {
        DepthZone::Mid
    } else if normalized < 0.75 {
        DepthZone::Deep
    } else {
        DepthZone::Core
    }
}

/// Format altitude for display.
#[must_use]
pub fn format_altitude(altitude: f32) -> String {
    if altitude.abs() < 1.0 {
        "Surface".to_string()
    } else if altitude > 0.0 {
        format!("{:.0}m deep", altitude)
    } else {
        format!("{:.0}m above", -altitude)
    }
}

/// Draw the depth indicator HUD element.
pub fn draw_depth_indicator(ctx: &egui::Context, indicator: &DepthIndicator) {
    let screen_rect = ctx.screen_rect();

    // Position at right side of screen
    let pos_x = screen_rect.width() - BAR_WIDTH - BAR_PADDING * 2.0 - 20.0;
    let pos_y = (screen_rect.height() - BAR_HEIGHT) / 2.0;

    egui::Area::new(egui::Id::new("depth_indicator"))
        .fixed_pos(Pos2::new(pos_x, pos_y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let outer_rect = Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(BAR_WIDTH + BAR_PADDING * 2.0, BAR_HEIGHT + BAR_PADDING * 2.0),
            );
            let _bar_rect = Rect::from_min_size(
                Pos2::new(BAR_PADDING, BAR_PADDING),
                Vec2::new(BAR_WIDTH, BAR_HEIGHT),
            );

            let painter = ui.painter();

            // Background
            painter.rect_filled(outer_rect, Rounding::same(4.0), BG_COLOR);

            // Draw zone gradient
            let zone_height = BAR_HEIGHT / 4.0;
            for (i, zone) in [DepthZone::Surface, DepthZone::Mid, DepthZone::Deep, DepthZone::Core].iter().enumerate() {
                let zone_rect = Rect::from_min_size(
                    Pos2::new(BAR_PADDING, BAR_PADDING + i as f32 * zone_height),
                    Vec2::new(BAR_WIDTH, zone_height),
                );
                painter.rect_filled(zone_rect, Rounding::ZERO, zone.color());
            }

            // Draw current position marker
            let normalized_depth = indicator.normalized_depth();
            let marker_y = BAR_PADDING + normalized_depth * BAR_HEIGHT;
            let marker_rect = Rect::from_min_size(
                Pos2::new(BAR_PADDING - 2.0, marker_y - 2.0),
                Vec2::new(BAR_WIDTH + 4.0, 4.0),
            );
            painter.rect_filled(marker_rect, Rounding::same(2.0), MARKER_COLOR);

            // Zone label
            let zone_text = indicator.zone.name();
            let text_pos = Pos2::new(outer_rect.center().x, outer_rect.max.y + 4.0);
            painter.text(
                text_pos,
                egui::Align2::CENTER_TOP,
                zone_text,
                egui::FontId::proportional(12.0),
                indicator.zone.color(),
            );

            // Altitude label
            let altitude_text = format_altitude(indicator.altitude());
            let altitude_pos = Pos2::new(outer_rect.center().x, outer_rect.max.y + 18.0);
            painter.text(
                altitude_pos,
                egui::Align2::CENTER_TOP,
                altitude_text,
                egui::FontId::proportional(10.0),
                TEXT_COLOR,
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPHERE_RADIUS: f32 = 4096.0;
    const CORE_RADIUS: f32 = 512.0;

    #[test]
    fn test_depth_zone_surface() {
        let zone = calculate_depth_zone(SPHERE_RADIUS - 100.0, SPHERE_RADIUS, CORE_RADIUS);
        assert_eq!(zone, DepthZone::Surface);
    }

    #[test]
    fn test_depth_zone_mid() {
        let zone = calculate_depth_zone(SPHERE_RADIUS - 1200.0, SPHERE_RADIUS, CORE_RADIUS);
        assert_eq!(zone, DepthZone::Mid);
    }

    #[test]
    fn test_depth_zone_deep() {
        let zone = calculate_depth_zone(SPHERE_RADIUS - 2200.0, SPHERE_RADIUS, CORE_RADIUS);
        assert_eq!(zone, DepthZone::Deep);
    }

    #[test]
    fn test_depth_zone_core() {
        let zone = calculate_depth_zone(CORE_RADIUS + 100.0, SPHERE_RADIUS, CORE_RADIUS);
        assert_eq!(zone, DepthZone::Core);
    }

    #[test]
    fn test_normalized_depth_at_surface() {
        let mut indicator = DepthIndicator::new(SPHERE_RADIUS, CORE_RADIUS);
        indicator.update(SPHERE_RADIUS);
        assert!(indicator.normalized_depth() < 0.01);
    }

    #[test]
    fn test_normalized_depth_at_core() {
        let mut indicator = DepthIndicator::new(SPHERE_RADIUS, CORE_RADIUS);
        indicator.update(CORE_RADIUS);
        assert!(indicator.normalized_depth() > 0.99);
    }

    #[test]
    fn test_format_altitude_surface() {
        assert_eq!(format_altitude(0.5), "Surface");
    }

    #[test]
    fn test_format_altitude_deep() {
        assert_eq!(format_altitude(100.0), "100m deep");
    }

    #[test]
    fn test_zone_names() {
        assert_eq!(DepthZone::Surface.name(), "Surface");
        assert_eq!(DepthZone::Mid.name(), "Mid");
        assert_eq!(DepthZone::Deep.name(), "Deep");
        assert_eq!(DepthZone::Core.name(), "Core");
    }

    #[test]
    fn test_altitude_calculation() {
        let mut indicator = DepthIndicator::new(SPHERE_RADIUS, CORE_RADIUS);
        indicator.update(SPHERE_RADIUS - 500.0);
        assert!((indicator.altitude() - 500.0).abs() < 0.01);
    }
}
