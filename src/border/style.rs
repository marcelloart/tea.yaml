//! `iced` compatible version of the container theme.



use crate::{
    Theme as Collection,
    color::Color,
};

use super::Serial;



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Border color.
    pub color: Color,

    /// Radius of the border.
    pub radius: f32,

    /// Width of the border.
    pub width: f32,
}

impl Theme {
    /// Default border for a button.
    pub const DEFAULT: Theme = Theme { color: Color::BLACK, radius: 0.0, width: 1.0 };

    /// Default border for a container.
    pub const BORDERLESS: Theme = Theme { color: Color::BLACK, radius: 0.0, width: 0.0 };

    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(stylestr) => match theme.border.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { color: colorstr, radius, width } = serial;

                    // Get the background color.
                    let color = match theme.color.get(colorstr) {
                        Some(c) => *c,
                        _ => Color::BLACK,
                    };

                    Some( Theme { color, radius: *radius, width: *width } )
                },

                _ => None,
            },

            _ => None,
        }
    }
}
