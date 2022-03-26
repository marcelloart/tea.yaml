//! `iced` compatible version of the tooltip theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    container::{
        Style, StyleSheet,
    },
};

use super::Serial;



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Background color.
    pub background: Color,

    /// Text color.
    pub textcolor: Color,

    /// Border theme.
    pub border: Border,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(ref stylestr) => match theme.tooltip.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { background, textcolor, border } = serial;

                    // Get the background color.
                    let background = match theme.color.get(background) {
                        Some(c) => *c,
                        _ => Color::WHITE,
                    };

                    // Get the text color.
                    let textcolor = match theme.color.get(textcolor) {
                        Some(c) => *c,
                        _ => Color::BLACK,
                    };

                    // Get the border style.
                    let border = match Border::extract(theme, border.clone()) {
                        Some(b) => b,
                        _ => Border::DEFAULT,
                    };

                    Some( Theme { background, textcolor, border } )
                },

                _ => None,
            },

            _ => None,
        }
    }
}

impl StyleSheet for Theme {
    fn style(&self) -> Style {
        Style {
            text_color: Some( self.textcolor.into() ),
            border_color: self.border.color.into(),
            background: Some( self.background.into() ),
            border_radius: self.border.radius,
            border_width: self.border.width,
        }
    }
}
