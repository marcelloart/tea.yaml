//! Serialized color.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Color(u8, u8, u8, f32);

impl Color {
    /// Default color black.
    pub const BLACK: Color = Color(0, 0, 0, 1.0);

    /// Default color red.
    pub const RED: Color = Color(255, 0, 0, 1.0);

    /// Default color blue.
    pub const BLUE: Color = Color(0, 0, 255, 1.0);

    /// Default color white.
    pub const WHITE: Color = Color(255, 255, 255, 1.0);
}

impl Into<iced::Color> for Color {
    fn into(self) -> iced::Color {
        let Color(r, g, b, a) = self;

        iced::Color::from_rgba8(r, g, b, a)
    }
}

impl Into<iced::Background> for Color {
    fn into(self) -> iced::Background {
        iced::Background::Color(self.into())
    }
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str( &format!("R: {:>3} | G: {:>3} | B: {:>3} | A: {:.3}", self.0, self.1, self.2, self.3) )
    }
}