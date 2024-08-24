#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ColorIndexed(pub u8);

#[cfg(feature = "cli")]
impl From<ColorIndexed> for ratatui::style::Color {
    fn from(val: ColorIndexed) -> Self {
        ratatui::style::Color::Indexed(val.0)
    }
}
