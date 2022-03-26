//! `iced` compatible version of the rule theme.



use crate::{
    Theme as Collection,
    color::Color,
};

use iced::{
    rule::{
        FillMode, Style, StyleSheet,
    },
};

use super::Serial;



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Color of the rule.
    pub color: Color,

    /// Fill mode of the rule.
    pub fillmode: FillMode,

    /// Radius of the rule.
    pub radius: f32,

    /// Width of the rule.
    pub width: u16,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(ref stylestr) => match theme.rule.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { color, fillmode, radius, width } = serial;

                    // Get the color of the rule.
                    let color = match theme.color.get(color) {
                        Some(c) => *c,
                        _ => Color::WHITE,
                    };

                    Some( Theme { color, fillmode: fillmode.convert(), radius: *radius, width: *width } )
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
            color: self.color.into(),
            width: self.width,
            radius: self.radius,
            fill_mode: self.fillmode,
        }
    }
}
