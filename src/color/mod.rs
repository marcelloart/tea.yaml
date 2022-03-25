//! Color theme.



use iced::{
    Background as IcedBackground,
    Color as IcedColor,
};

use serde::{ Deserialize, Serialize };



#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Color(u8, u8, u8, f32);

impl Color {
    /// Default color black.
    pub const BLACK: Color = Color(0, 0, 0, 1.0);

    /// Default color white.
    pub const RED: Color = Color(255, 0, 0, 1.0);

    /// Default color white.
    pub const WHITE: Color = Color(255, 255, 255, 1.0);
}

impl Into<IcedColor> for Color {
    fn into(self) -> IcedColor {
        let Color(r, g, b, a) = self;

        IcedColor::from_rgba8(r, g, b, a)
    }
}

impl Into<IcedBackground> for Color {
    fn into(self) -> IcedBackground {
        IcedBackground::Color(self.into())
    }
}
