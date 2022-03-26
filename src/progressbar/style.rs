//! `iced` compatible version of the progress bar theme.



use crate::{
    Theme as Collection,
    color::Color,
};

use iced::{
    progress_bar::{
        Style, StyleSheet,
    },
};

use super::Serial;



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Color of the background.
    pub background: Color,

    /// Color of the bar.
    pub bar: Color,

    /// Radius of the theme.
    pub radius: f32,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(ref stylestr) => match theme.progressbar.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { background, bar, radius } = serial;

                    // Get the background color.
                    let background = match theme.color.get(background) {
                        Some(c) => *c,
                        _ => Color::WHITE,
                    };

                    // Get the background color.
                    let bar = match theme.color.get(bar) {
                        Some(c) => *c,
                        _ => Color::BLACK,
                    };

                    Some( Theme { background, bar, radius: *radius } )
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
            background: self.background.into(),
            bar: self.bar.into(),
            border_radius: self.radius,
        }
    }
}
