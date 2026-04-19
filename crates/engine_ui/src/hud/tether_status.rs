//! Tether status HUD for hollow sphere traversal.
//!
//! Shows the status of attached tethers, including tension,
//! length, and connection health.

use egui::{Color32, Pos2, Rect, Rounding, Vec2};

/// Icon size for tether status.
const ICON_SIZE: f32 = 20.0;

/// Bar width for tension display.
const TENSION_BAR_WIDTH: f32 = 60.0;

/// Bar height for tension display.
const TENSION_BAR_HEIGHT: f32 = 6.0;

/// Padding around elements.
const PADDING: f32 = 4.0;

/// Colors for tether status.
const TETHER_HEALTHY: Color32 = Color32::from_rgb(80, 180, 80);
const TETHER_STRESSED: Color32 = Color32::from_rgb(200, 200, 60);
const TETHER_CRITICAL: Color32 = Color32::from_rgb(200, 80, 40);
const TETHER_BROKEN: Color32 = Color32::from_rgb(100, 100, 100);
const BG_COLOR: Color32 = Color32::from_rgba_premultiplied(20, 20, 20, 180);
const TEXT_COLOR: Color32 = Color32::from_rgb(220, 220, 220);

/// Tether health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetherHealth {
    /// Tether is healthy (tension < 50%).
    Healthy,
    /// Tether is stressed (tension 50-80%).
    Stressed,
    /// Tether is critical (tension > 80%).
    Critical,
    /// Tether has snapped.
    Broken,
}

impl TetherHealth {
    /// Get the color for this health status.
    #[must_use]
    pub fn color(self) -> Color32 {
        match self {
            TetherHealth::Healthy => TETHER_HEALTHY,
            TetherHealth::Stressed => TETHER_STRESSED,
            TetherHealth::Critical => TETHER_CRITICAL,
            TetherHealth::Broken => TETHER_BROKEN,
        }
    }

    /// Get display text for this status.
    #[must_use]
    pub fn text(self) -> &'static str {
        match self {
            TetherHealth::Healthy => "OK",
            TetherHealth::Stressed => "STRESSED",
            TetherHealth::Critical => "CRITICAL",
            TetherHealth::Broken => "BROKEN",
        }
    }
}

/// Single tether display info.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TetherInfo {
    /// Current tension (0.0-1.0 as ratio of max).
    pub tension_ratio: f32,
    /// Length of the tether in blocks.
    pub length: f32,
    /// Whether the tether is broken.
    pub is_broken: bool,
    /// Tether ID for display.
    pub id: u8,
}

impl TetherInfo {
    /// Get the health status of this tether.
    #[must_use]
    pub fn health(&self) -> TetherHealth {
        if self.is_broken {
            TetherHealth::Broken
        } else if self.tension_ratio > 0.8 {
            TetherHealth::Critical
        } else if self.tension_ratio > 0.5 {
            TetherHealth::Stressed
        } else {
            TetherHealth::Healthy
        }
    }
}

/// Tether status indicator state.
#[derive(Debug, Clone, PartialEq)]
pub struct TetherStatus {
    /// Currently attached tethers.
    pub tethers: Vec<TetherInfo>,
    /// Whether the player is tethered to any anchor.
    pub is_tethered: bool,
    /// Total number of tether slots available.
    pub max_tethers: u8,
}

impl Default for TetherStatus {
    fn default() -> Self {
        Self {
            tethers: Vec::new(),
            is_tethered: false,
            max_tethers: 4,
        }
    }
}

impl TetherStatus {
    /// Create a new tether status.
    #[must_use]
    pub fn new(max_tethers: u8) -> Self {
        Self {
            tethers: Vec::new(),
            is_tethered: false,
            max_tethers,
        }
    }

    /// Add a tether connection.
    pub fn add_tether(&mut self, tension_ratio: f32, length: f32) {
        if self.tethers.len() < self.max_tethers as usize {
            let id = self.tethers.len() as u8 + 1;
            self.tethers.push(TetherInfo {
                tension_ratio: tension_ratio.clamp(0.0, 1.0),
                length,
                is_broken: false,
                id,
            });
            self.is_tethered = true;
        }
    }

    /// Update a tether's tension.
    pub fn update_tether(&mut self, index: usize, tension_ratio: f32) {
        if let Some(tether) = self.tethers.get_mut(index) {
            tether.tension_ratio = tension_ratio.clamp(0.0, 1.0);
        }
    }

    /// Mark a tether as broken.
    pub fn break_tether(&mut self, index: usize) {
        if let Some(tether) = self.tethers.get_mut(index) {
            tether.is_broken = true;
        }
        self.update_tethered_state();
    }

    /// Remove a tether.
    pub fn remove_tether(&mut self, index: usize) {
        if index < self.tethers.len() {
            self.tethers.remove(index);
            // Re-number remaining tethers
            for (i, tether) in self.tethers.iter_mut().enumerate() {
                tether.id = i as u8 + 1;
            }
        }
        self.update_tethered_state();
    }

    /// Clear all tethers.
    pub fn clear(&mut self) {
        self.tethers.clear();
        self.is_tethered = false;
    }

    /// Get count of intact tethers.
    #[must_use]
    pub fn intact_count(&self) -> usize {
        self.tethers.iter().filter(|t| !t.is_broken).count()
    }

    fn update_tethered_state(&mut self) {
        self.is_tethered = self.tethers.iter().any(|t| !t.is_broken);
    }
}

/// Format tether length for display.
#[must_use]
pub fn format_length(length: f32) -> String {
    format!("{:.1}m", length)
}

/// Draw the tether status HUD element.
pub fn draw_tether_status(ctx: &egui::Context, status: &TetherStatus) {
    // Don't show if no tethers
    if status.tethers.is_empty() {
        return;
    }

    let screen_rect = ctx.screen_rect();

    // Position at bottom-right of screen
    let entry_height = ICON_SIZE + PADDING;
    let total_height = status.tethers.len() as f32 * entry_height + PADDING;
    let total_width = ICON_SIZE + TENSION_BAR_WIDTH + PADDING * 4.0 + 40.0;

    let pos_x = screen_rect.width() - total_width - 20.0;
    let pos_y = screen_rect.height() - total_height - 20.0;

    egui::Area::new(egui::Id::new("tether_status"))
        .fixed_pos(Pos2::new(pos_x, pos_y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let outer_rect = Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(total_width, total_height),
            );

            let painter = ui.painter();

            // Background
            painter.rect_filled(outer_rect, Rounding::same(4.0), BG_COLOR);

            // Draw each tether
            for (i, tether) in status.tethers.iter().enumerate() {
                let y_offset = PADDING + i as f32 * entry_height;
                let health = tether.health();

                // Tether icon (simple circle)
                let icon_center = Pos2::new(PADDING + ICON_SIZE / 2.0, y_offset + ICON_SIZE / 2.0);
                painter.circle_filled(icon_center, ICON_SIZE / 2.0 - 2.0, health.color());

                // Tether number
                painter.text(
                    icon_center,
                    egui::Align2::CENTER_CENTER,
                    format!("{}", tether.id),
                    egui::FontId::proportional(10.0),
                    Color32::WHITE,
                );

                // Tension bar
                let bar_x = PADDING * 2.0 + ICON_SIZE;
                let bar_y = y_offset + (ICON_SIZE - TENSION_BAR_HEIGHT) / 2.0;
                let bar_rect = Rect::from_min_size(
                    Pos2::new(bar_x, bar_y),
                    Vec2::new(TENSION_BAR_WIDTH, TENSION_BAR_HEIGHT),
                );

                // Bar background
                painter.rect_filled(bar_rect, Rounding::same(2.0), Color32::from_rgb(40, 40, 40));

                // Tension fill
                if !tether.is_broken {
                    let fill_width = tether.tension_ratio * TENSION_BAR_WIDTH;
                    let fill_rect = Rect::from_min_size(
                        bar_rect.min,
                        Vec2::new(fill_width, TENSION_BAR_HEIGHT),
                    );
                    painter.rect_filled(fill_rect, Rounding::same(2.0), health.color());
                }

                // Length text
                let length_x = bar_x + TENSION_BAR_WIDTH + PADDING;
                let length_pos = Pos2::new(length_x, y_offset + ICON_SIZE / 2.0);
                painter.text(
                    length_pos,
                    egui::Align2::LEFT_CENTER,
                    format_length(tether.length),
                    egui::FontId::proportional(10.0),
                    TEXT_COLOR,
                );
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tether_health_healthy() {
        let tether = TetherInfo {
            tension_ratio: 0.3,
            length: 10.0,
            is_broken: false,
            id: 1,
        };
        assert_eq!(tether.health(), TetherHealth::Healthy);
    }

    #[test]
    fn test_tether_health_stressed() {
        let tether = TetherInfo {
            tension_ratio: 0.6,
            length: 10.0,
            is_broken: false,
            id: 1,
        };
        assert_eq!(tether.health(), TetherHealth::Stressed);
    }

    #[test]
    fn test_tether_health_critical() {
        let tether = TetherInfo {
            tension_ratio: 0.9,
            length: 10.0,
            is_broken: false,
            id: 1,
        };
        assert_eq!(tether.health(), TetherHealth::Critical);
    }

    #[test]
    fn test_tether_health_broken() {
        let tether = TetherInfo {
            tension_ratio: 0.3,
            length: 10.0,
            is_broken: true,
            id: 1,
        };
        assert_eq!(tether.health(), TetherHealth::Broken);
    }

    #[test]
    fn test_add_tether() {
        let mut status = TetherStatus::new(4);
        status.add_tether(0.3, 15.0);
        assert_eq!(status.tethers.len(), 1);
        assert!(status.is_tethered);
    }

    #[test]
    fn test_add_tether_respects_max() {
        let mut status = TetherStatus::new(2);
        status.add_tether(0.3, 10.0);
        status.add_tether(0.3, 10.0);
        status.add_tether(0.3, 10.0); // Should not add
        assert_eq!(status.tethers.len(), 2);
    }

    #[test]
    fn test_break_tether() {
        let mut status = TetherStatus::new(4);
        status.add_tether(0.3, 10.0);
        status.break_tether(0);
        assert!(status.tethers[0].is_broken);
        assert!(!status.is_tethered);
    }

    #[test]
    fn test_intact_count() {
        let mut status = TetherStatus::new(4);
        status.add_tether(0.3, 10.0);
        status.add_tether(0.3, 10.0);
        status.break_tether(0);
        assert_eq!(status.intact_count(), 1);
    }

    #[test]
    fn test_format_length() {
        assert_eq!(format_length(10.0), "10.0m");
        assert_eq!(format_length(15.5), "15.5m");
    }

    #[test]
    fn test_health_colors_unique() {
        assert_ne!(TetherHealth::Healthy.color(), TetherHealth::Broken.color());
        assert_ne!(TetherHealth::Stressed.color(), TetherHealth::Critical.color());
    }
}
