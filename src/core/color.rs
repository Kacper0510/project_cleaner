/// Contains [ANSI 8-bit color index](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct IconColor(pub u8);

#[cfg(feature = "cli")]
impl From<IconColor> for ratatui::style::Color {
    fn from(value: IconColor) -> Self {
        ratatui::style::Color::Indexed(value.0)
    }
}
