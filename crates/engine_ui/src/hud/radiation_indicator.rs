//! Radiation level indicator HUD for hollow sphere survival.
//!
//! Displays the current radiation level and shield status,
//! warning players of dangerous exposure near the core.

use egui::{Color32, Pos2, Rect, Rounding, Vec2};

/// Width of the radiation bar.
const BAR_WIDTH: f32 = 100.0;

/// Height of the radiation bar.
const BAR_HEIGHT: f32 = 12.0;

/// Padding around elements.
const PADDING: f32 = 4.0;

/// Colors for radiation levels.
const RAD_SAFE: Color32 = Color32::from_rgb(80, 180, 80);
const RAD_WARNING: Color32 = Color32::from_rgb(200, 200, 60);
const RAD_DANGER: Color32 = Color32::from_rgb(200, 120, 40);
const RAD_LETHAL: Color32 = Color32::from_rgb(200, 40, 40);
const BG_COLOR: Color32 = Color32::from_rgba_premultiplied(20, 20, 20, 180);
const SHIELD_COLOR: Color32 = Color32::from_rgb(100, 150, 220);
const TEXT_COLOR: Color32 = Color32::from_rgb(220, 220, 220);

/// Radiation warning level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiationWarning {
    /// Safe radiation level.
    Safe,
    /// Warning level - minor exposure.
    Warning,
    /// Danger level - significant exposure.
    Danger,
    /// Lethal level - critical exposure.
    Lethal,
}

impl RadiationWarning {
    /// Get the display text for this warning level.
    #[must_use]
    pub fn text(self) -> &'static str {
        match self {
            RadiationWarning::Safe => "SAFE",
            RadiationWarning::Warning => "WARNING",
            RadiationWarning::Danger => "DANGER",
            RadiationWarning::Lethal => "LETHAL",
        }
    }

    /// Get the color for this warning level.
    #[must_use]
    pub fn color(self) -> Color32 {
        match self {
            RadiationWarning::Safe => RAD_SAFE,
            RadiationWarning::Warning => RAD_WARNING,
            RadiationWarning::Danger => RAD_DANGER,
            RadiationWarning::Lethal => RAD_LETHAL,
        }
    }
}

/// Radiation indicator state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadiationIndicator {
    /// Current radiation level (0.0-1.0).
    pub radiation_level: f32,
    /// Effective radiation after shield (0.0-1.0).
    pub effective_level: f32,
    /// Shield reduction factor (0.0-1.0).
    pub shield_factor: f32,
    /// Whether the indicator should flash (taking damage).
    pub is_flashing: bool,
    /// Flash timer for animation.
    flash_timer: f32,
}

impl Default for RadiationIndicator {
    fn default() -> Self {
        Self {
            radiation_level: 0.0,
            effective_level: 0.0,
            shield_factor: 0.0,
            is_flashing: false,
            flash_timer: 0.0,
        }
    }
}

impl RadiationIndicator {
    /// Create a new radiation indicator.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the radiation indicator.
    pub fn update(&mut self, radiation_level: f32, shield_factor: f32, dt: f32) {
        let old_effective = self.effective_level;

        self.radiation_level = radiation_level.clamp(0.0, 1.0);
        self.shield_factor = shield_factor.clamp(0.0, 1.0);
        self.effective_level = (radiation_level * (1.0 - shield_factor)).clamp(0.0, 1.0);

        // Flash when taking radiation damage
        if self.effective_level > 0.1 && self.effective_level > old_effective {
            self.is_flashing = true;
            self.flash_timer = 0.3;
        }

        // Update flash timer
        if self.flash_timer > 0.0 {
            self.flash_timer -= dt;
            if self.flash_timer <= 0.0 {
                self.is_flashing = false;
            }
        }
    }

    /// Get the current warning level.
    #[must_use]
    pub fn warning_level(&self) -> RadiationWarning {
        warning_from_level(self.effective_level)
    }
}

/// Get warning level from radiation value.
#[must_use]
pub fn warning_from_level(level: f32) -> RadiationWarning {
    if level < 0.1 {
        RadiationWarning::Safe
    } else if level < 0.5 {
        RadiationWarning::Warning
    } else if level < 0.9 {
        RadiationWarning::Danger
    } else {
        RadiationWarning::Lethal
    }
}

/// Format radiation level as percentage.
#[must_use]
pub fn format_radiation(level: f32) -> String {
    format!("{:.0}%", level * 100.0)
}

/// Draw the radiation indicator HUD element.
pub fn draw_radiation_indicator(ctx: &egui::Context, indicator: &RadiationIndicator) {
    // Don't show if radiation is safe
    if indicator.radiation_level < 0.01 {
        return;
    }

    let screen_rect = ctx.screen_rect();

    // Position at top-center of screen
    let pos_x = (screen_rect.width() - BAR_WIDTH - PADDING * 2.0) / 2.0;
    let pos_y = 60.0;

    egui::Area::new(egui::Id::new("radiation_indicator"))
        .fixed_pos(Pos2::new(pos_x, pos_y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let warning = indicator.warning_level();
            let total_height = BAR_HEIGHT + PADDING * 2.0 + 16.0;
            let outer_rect = Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(BAR_WIDTH + PADDING * 2.0, total_height),
            );

            let painter = ui.painter();

            // Background (flash effect when taking damage)
            let bg = if indicator.is_flashing {
                Color32::from_rgba_premultiplied(100, 20, 20, 200)
            } else {
                BG_COLOR
            };
            painter.rect_filled(outer_rect, Rounding::same(4.0), bg);

            // Warning label
            let label_pos = Pos2::new(outer_rect.center().x, PADDING);
            painter.text(
                label_pos,
                egui::Align2::CENTER_TOP,
                warning.text(),
                egui::FontId::proportional(10.0),
                warning.color(),
            );

            // Radiation bar background
            let bar_rect = Rect::from_min_size(
                Pos2::new(PADDING, PADDING + 14.0),
                Vec2::new(BAR_WIDTH, BAR_HEIGHT),
            );
            painter.rect_filled(bar_rect, Rounding::same(2.0), Color32::from_rgb(40, 40, 40));

            // Base radiation level (before shield)
            let base_width = indicator.radiation_level * BAR_WIDTH;
            let base_rect = Rect::from_min_size(
                bar_rect.min,
                Vec2::new(base_width, BAR_HEIGHT),
            );
            painter.rect_filled(base_rect, Rounding::same(2.0), warning.color().gamma_multiply(0.5));

            // Effective radiation level (after shield)
            let effective_width = indicator.effective_level * BAR_WIDTH;
            let effective_rect = Rect::from_min_size(
                bar_rect.min,
                Vec2::new(effective_width, BAR_HEIGHT),
            );
            painter.rect_filled(effective_rect, Rounding::same(2.0), warning.color());

            // Shield indicator (if active)
            if indicator.shield_factor > 0.01 {
                let shield_text = format!("{:.0}%", indicator.shield_factor * 100.0);
                let shield_pos = Pos2::new(outer_rect.max.x - PADDING, bar_rect.center().y);
                painter.text(
                    shield_pos,
                    egui::Align2::RIGHT_CENTER,
                    shield_text,
                    egui::FontId::proportional(9.0),
                    SHIELD_COLOR,
                );
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_from_level_safe() {
        assert_eq!(warning_from_level(0.0), RadiationWarning::Safe);
        assert_eq!(warning_from_level(0.05), RadiationWarning::Safe);
    }

    #[test]
    fn test_warning_from_level_warning() {
        assert_eq!(warning_from_level(0.1), RadiationWarning::Warning);
        assert_eq!(warning_from_level(0.3), RadiationWarning::Warning);
    }

    #[test]
    fn test_warning_from_level_danger() {
        assert_eq!(warning_from_level(0.5), RadiationWarning::Danger);
        assert_eq!(warning_from_level(0.7), RadiationWarning::Danger);
    }

    #[test]
    fn test_warning_from_level_lethal() {
        assert_eq!(warning_from_level(0.9), RadiationWarning::Lethal);
        assert_eq!(warning_from_level(1.0), RadiationWarning::Lethal);
    }

    #[test]
    fn test_format_radiation() {
        assert_eq!(format_radiation(0.0), "0%");
        assert_eq!(format_radiation(0.5), "50%");
        assert_eq!(format_radiation(1.0), "100%");
    }

    #[test]
    fn test_indicator_update_with_shield() {
        let mut indicator = RadiationIndicator::new();
        indicator.update(1.0, 0.5, 0.0);
        assert!((indicator.effective_level - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_indicator_update_no_shield() {
        let mut indicator = RadiationIndicator::new();
        indicator.update(0.8, 0.0, 0.0);
        assert!((indicator.effective_level - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_indicator_warning_level() {
        let mut indicator = RadiationIndicator::new();
        indicator.update(0.6, 0.0, 0.0);
        assert_eq!(indicator.warning_level(), RadiationWarning::Danger);
    }

    #[test]
    fn test_warning_text() {
        assert_eq!(RadiationWarning::Safe.text(), "SAFE");
        assert_eq!(RadiationWarning::Lethal.text(), "LETHAL");
    }

    #[test]
    fn test_indicator_flash_on_damage() {
        let mut indicator = RadiationIndicator::new();
        indicator.update(0.5, 0.0, 0.0);
        assert!(indicator.is_flashing);
    }
}
