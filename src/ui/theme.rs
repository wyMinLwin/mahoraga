use ratatui::style::Color;

/// Gold/primary color
pub const PRIMARY: Color = Color::Rgb(212, 175, 55);

/// Secondary text color
pub const SECONDARY: Color = Color::Rgb(176, 176, 176);

/// Muted/dimmed text color
pub const MUTED: Color = Color::Rgb(102, 102, 102);

/// Success color (green)
pub const SUCCESS: Color = Color::Rgb(34, 197, 94);

/// Error color (red)
pub const ERROR: Color = Color::Rgb(239, 68, 68);

/// Warning color (yellow)
pub const WARNING: Color = Color::Rgb(234, 179, 8);

/// Background color (dark)
pub const BACKGROUND: Color = Color::Rgb(17, 17, 17);

/// Surface color (slightly lighter than background)
pub const SURFACE: Color = Color::Rgb(30, 30, 30);

/// Border color
pub const BORDER: Color = Color::Rgb(64, 64, 64);

/// Get color based on score
pub fn score_color(score: u8) -> Color {
    match score {
        90..=100 => SUCCESS,
        70..=89 => Color::Rgb(34, 197, 94),  // green
        50..=69 => WARNING,
        30..=49 => Color::Rgb(249, 115, 22), // orange
        _ => ERROR,
    }
}

/// Get score label based on score
pub fn score_label(score: u8) -> &'static str {
    match score {
        90..=100 => "Excellent",
        70..=89 => "Good",
        50..=69 => "Fair",
        30..=49 => "Poor",
        _ => "Very Poor",
    }
}
