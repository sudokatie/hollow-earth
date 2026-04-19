//! Gravity direction indicator HUD for hollow sphere navigation.
//!
//! Shows the current gravity direction relative to the camera,
//! helping players orient themselves in the inverted sphere world.

use egui::{Color32, Pos2, Rect, Rounding, Vec2};
use glam::Vec3;

/// Size of the gravity indicator.
const INDICATOR_SIZE: f32 = 64.0;

/// Arrow size within the indicator.
const ARROW_SIZE: f32 = 24.0;

/// Colors for the indicator.
const BG_COLOR: Color32 = Color32::from_rgba_premultiplied(20, 20, 20, 180);
const ARROW_NORMAL: Color32 = Color32::from_rgb(80, 180, 80);
const ARROW_DISORIENTED: Color32 = Color32::from_rgb(200, 80, 80);
const TEXT_COLOR: Color32 = Color32::from_rgb(220, 220, 220);

/// Gravity indicator state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravityIndicator {
    /// Gravity direction in world space.
    pub direction: Vec3,
    /// Gravity direction relative to camera view.
    pub relative_to_camera: Vec3,
    /// Whether the player is disoriented.
    pub is_disoriented: bool,
}

impl Default for GravityIndicator {
    fn default() -> Self {
        Self {
            direction: Vec3::NEG_Y,
            relative_to_camera: Vec3::NEG_Y,
            is_disoriented: false,
        }
    }
}

impl GravityIndicator {
    /// Create a new gravity indicator.
    #[must_use]
    pub fn new(direction: Vec3, relative_to_camera: Vec3) -> Self {
        Self {
            direction: direction.normalize_or_zero(),
            relative_to_camera: relative_to_camera.normalize_or_zero(),
            is_disoriented: false,
        }
    }

    /// Set the disorientation state.
    pub fn set_disoriented(&mut self, disoriented: bool) {
        self.is_disoriented = disoriented;
    }

    /// Update the gravity direction.
    pub fn update(&mut self, direction: Vec3, relative_to_camera: Vec3) {
        self.direction = direction.normalize_or_zero();
        self.relative_to_camera = relative_to_camera.normalize_or_zero();
    }
}

/// Format a direction vector as a human-readable string.
///
/// Returns cardinal direction based on the dominant axis:
/// - "Down" for -Y dominant
/// - "Up" for +Y dominant
/// - "North/South/East/West" for horizontal directions
#[must_use]
pub fn format_direction(direction: Vec3) -> String {
    let abs_x = direction.x.abs();
    let abs_y = direction.y.abs();
    let abs_z = direction.z.abs();

    // Find dominant axis
    if abs_y >= abs_x && abs_y >= abs_z {
        if direction.y < 0.0 {
            "Down".to_string()
        } else {
            "Up".to_string()
        }
    } else if abs_x >= abs_z {
        if direction.x > 0.0 {
            "East".to_string()
        } else {
            "West".to_string()
        }
    } else if direction.z > 0.0 {
        "South".to_string()
    } else {
        "North".to_string()
    }
}

/// Draw the gravity indicator HUD element.
pub fn draw_gravity_indicator(ctx: &egui::Context, indicator: &GravityIndicator) {
    let screen_rect = ctx.screen_rect();

    // Position at bottom-left corner
    let pos_x = 20.0;
    let pos_y = screen_rect.height() - INDICATOR_SIZE - 20.0;

    egui::Area::new(egui::Id::new("gravity_indicator"))
        .fixed_pos(Pos2::new(pos_x, pos_y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let rect = Rect::from_min_size(Pos2::ZERO, Vec2::splat(INDICATOR_SIZE));

            let painter = ui.painter();

            // Background circle
            painter.circle_filled(
                rect.center(),
                INDICATOR_SIZE / 2.0,
                BG_COLOR,
            );

            // Arrow color based on disorientation
            let arrow_color = if indicator.is_disoriented {
                ARROW_DISORIENTED
            } else {
                ARROW_NORMAL
            };

            // Draw arrow pointing in gravity direction (projected to 2D)
            let rel = indicator.relative_to_camera;
            let arrow_dir = Vec2::new(rel.x, -rel.y).normalized();

            if arrow_dir.length() > 0.1 {
                let center = rect.center();
                let arrow_tip = center + arrow_dir * (INDICATOR_SIZE / 2.0 - 8.0);
                let arrow_base = center - arrow_dir * (INDICATOR_SIZE / 4.0);

                // Arrow shaft
                painter.line_segment(
                    [arrow_base, arrow_tip],
                    egui::Stroke::new(3.0, arrow_color),
                );

                // Arrow head
                let perp = Vec2::new(-arrow_dir.y, arrow_dir.x);
                let head_size = 8.0;
                let head_back = arrow_tip - arrow_dir * head_size;

                painter.add(egui::Shape::convex_polygon(
                    vec![
                        arrow_tip,
                        head_back + perp * head_size * 0.5,
                        head_back - perp * head_size * 0.5,
                    ],
                    arrow_color,
                    egui::Stroke::NONE,
                ));
            }

            // Direction label
            let direction_text = format_direction(indicator.direction);
            let text_pos = Pos2::new(rect.center().x, rect.max.y + 4.0);
            painter.text(
                text_pos,
                egui::Align2::CENTER_TOP,
                direction_text,
                egui::FontId::proportional(12.0),
                TEXT_COLOR,
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_direction_down() {
        let dir = Vec3::new(0.0, -1.0, 0.0);
        assert_eq!(format_direction(dir), "Down");
    }

    #[test]
    fn test_format_direction_up() {
        let dir = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(format_direction(dir), "Up");
    }

    #[test]
    fn test_format_direction_east() {
        let dir = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(format_direction(dir), "East");
    }

    #[test]
    fn test_format_direction_west() {
        let dir = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(format_direction(dir), "West");
    }

    #[test]
    fn test_format_direction_north() {
        let dir = Vec3::new(0.0, 0.0, -1.0);
        assert_eq!(format_direction(dir), "North");
    }

    #[test]
    fn test_format_direction_south() {
        let dir = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(format_direction(dir), "South");
    }

    #[test]
    fn test_format_direction_dominant_axis() {
        // Y dominant even with some X/Z
        let dir = Vec3::new(0.3, -0.9, 0.2);
        assert_eq!(format_direction(dir), "Down");
    }

    #[test]
    fn test_gravity_indicator_default() {
        let indicator = GravityIndicator::default();
        assert!(!indicator.is_disoriented);
        assert!((indicator.direction.y - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_gravity_indicator_update() {
        let mut indicator = GravityIndicator::default();
        indicator.update(Vec3::X, Vec3::X);
        assert!((indicator.direction.x - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_gravity_indicator_disoriented() {
        let mut indicator = GravityIndicator::default();
        indicator.set_disoriented(true);
        assert!(indicator.is_disoriented);
    }
}
