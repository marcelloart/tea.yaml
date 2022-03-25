//! `iced` compatible version of the container theme.



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
    /// Key to the background color.
    pub color: Color,

    /// Key to the border definition.
    pub border: Border,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(ref stylestr) => match theme.container.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { color: colorstr, border: borderstr } = serial;

                    // Get the background color.
                    let color = match theme.color.get(colorstr) {
                        Some(c) => *c,
                        _ => Color::WHITE,
                    };

                    // Get the border style.
                    let border = match Border::extract(theme, borderstr.clone()) {
                        Some(b) => b,
                        _ => Border::BORDERLESS,
                    };

                    Some( Theme { color, border } )
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
            border_color: self.border.color.into(),
            border_radius: self.border.radius,
            border_width: self.border.width,

            background: Some( self.color.into() ),

            text_color: None,
        }
    }
}
