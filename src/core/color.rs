#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct IconColor {
    pub normal: u8,
    pub selected: u8,
}

impl IconColor {
    pub const fn new(normal: u8, selected: u8) -> Self {
        IconColor {
            normal,
            selected,
        }
    }
}

#[cfg(feature = "cli")]
impl IconColor {
    pub fn normal(self) -> ratatui::style::Color {
        ratatui::style::Color::Indexed(self.normal)
    }

    pub fn selected(self) -> ratatui::style::Color {
        ratatui::style::Color::Indexed(self.selected)
    }
}
