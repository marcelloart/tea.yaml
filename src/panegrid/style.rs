//! `iced` compatible version of the button theme.



use crate::{
    Theme as Collection,
    color::Color,
};

use iced::{
    pane_grid::{
        Line, StyleSheet,
    },
};

use super::{ Component, Serial, LineSerial };



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Active state.
    pub picked: Option<LineTheme>,

    /// Hovered state.
    pub hovered: Option<LineTheme>,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(stylestr) => match theme.panegrid.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { picked, hovered } = serial;

                    // Get the picked state theme.
                    let picked = match picked {
                        Some(component) => match LineTheme::picked(theme, component.clone()) {
                            Some(t) => Some(t),
                            _ => Some( LineTheme::DEFAULT ),
                        },
                        _ => None,
                    };


                    // Get the hovered state theme.
                    let hovered = match hovered {
                        Some(component) => match LineTheme::hovered(theme, component.clone()) {
                            Some(t) => Some(t),
                            _ => Some( LineTheme::DEFAULT ),
                        },
                        _ => None,
                    };

                    Some( Theme { picked, hovered } )
                },

                _ => None,
            },

            _ => None,
        }
    }
}

impl StyleSheet for Theme {
    fn picked_split(&self) -> Option<Line> {
        match self.picked {
            Some(l) => Some(l.into()),
            _ => None,
        }
    }

    fn hovered_split(&self) -> Option<Line> {
        match self.picked {
            Some(l) => Some(l.into()),
            _ => None,
        }
    }
}



#[derive(Clone, Copy, Debug)]
pub struct LineTheme {
    /// Line color.
    pub color: Color,

    /// Line width.
    pub width: f32,
}

impl LineTheme {
    /// Default state theme.
    pub const DEFAULT: LineTheme = LineTheme { color: Color::BLACK, width: 1.0 };

    /// Gets the `LineTheme` from a defined serial.
    pub(self) fn defined(theme: &Collection, serial: LineSerial) -> Self {
        // Destructure the serial.
        let LineSerial { color, width } = serial;

        // Get the line color.
        let color = match theme.color.get(&color) {
            Some(c) => *c,
            _ => Color::BLACK,
        };

        LineTheme { color, width }
    }

    /// Gets the `LineTheme` from the picked component.
    pub(self) fn picked(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(panegrid) => panegrid.picked.clone(),
                _ => None,
            },

            Component::None => None,
        }
    }

    /// Gets the `LineTheme` from the hovered component.
    pub(self) fn hovered(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(button) => button.hovered.clone(),
                _ => None,
            },

            Component::None => None,
        }
    }
}

impl core::convert::Into<Line> for LineTheme {
    fn into(self) -> Line {
        Line {
            color: self.color.into(),
            width: self.width,
        }
    }
}
