//! `iced` compatible version of the button theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    radio::{
        Style, StyleSheet,
    },
};

use super::{ Component, Serial, StateSerial };



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Active state.
    pub active: StateTheme,

    /// Hovered state.
    pub hovered: StateTheme,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(stylestr) => match theme.radio.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { active, hovered } = serial;

                    // Get the active state theme.
                    let active = match StateTheme::active(theme, active.clone()) {
                        Some(t) => t,
                        _ => StateTheme::DEFAULT,
                    };

                    // Get the hovered state theme.
                    let hovered = match StateTheme::hovered(theme, hovered.clone()) {
                        Some(t) => t,
                        _ => active.clone(),
                    };

                    Some( Theme { active, hovered } )
                },

                _ => None,
            },

            _ => None,
        }
    }
}

impl StyleSheet for Theme {
    fn active(&self) -> Style {
        self.active.into()
    }

    fn hovered(&self) -> Style {
        self.hovered.into()
    }
}



#[derive(Clone, Copy, Debug)]
pub struct StateTheme {
    /// Background color.
    pub background: Color,

    /// Dot color.
    pub dotcolor: Color,

    /// Border theme.
    pub border: Border,
}

impl StateTheme {
    /// Default state theme.
    pub const DEFAULT: StateTheme = StateTheme { background: Color::WHITE, dotcolor: Color::BLACK, border: Border::DEFAULT };

    /// Gets the `StateTheme` from a defined serial.
    pub(self) fn defined(theme: &Collection, serial: StateSerial) -> Self {
        // Destructure the serial.
        let StateSerial { background, dotcolor, border } = serial;

        // Get the background color.
        let background = match theme.color.get(&background) {
            Some(c) => *c,
            _ => Color::WHITE,
        };

        // Get the dot color.
        let dotcolor = match theme.color.get(&dotcolor) {
            Some(c) => *c,
            _ => Color::RED,
        };

        // Get the border theme.
        let border = match Border::extract(theme, border) {
            Some(b) => b,
            _ => Border::DEFAULT,
        };

        StateTheme { background, dotcolor, border }
    }

    /// Gets the `StateTheme` from the active component.
    pub(self) fn active(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(radio) => Some( radio.active.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }

    /// Gets the `StateTheme` from the hovered component.
    pub(self) fn hovered(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(radio) => Some( radio.hovered.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }
}

impl core::convert::Into<Style> for StateTheme {
    fn into(self) -> Style {
        Style {
            background: self.background.into(),
            border_color: self.border.color.into(),
            border_width: self.border.width,
            dot_color: self.dotcolor.into(),
        }
    }
}
