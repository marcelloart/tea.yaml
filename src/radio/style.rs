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
    pub active: Style,

    /// Hovered state.
    pub hovered: Style,
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
                    let active = match Self::active(theme, active.clone()) {
                        Some(t) => t,
                        _ => Style {
                            background: Color::WHITE.into(),
                            dot_color: Color::RED.into(),
                            text_color: None,
                            border_color: Color::BLACK.into(),
                            border_width: 1.0,
                        },
                    };

                    // Get the hovered state theme.
                    let hovered = match Self::hovered(theme, hovered.clone()) {
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

    /// Creates or inherits the active style.
    fn active(theme: &Collection, component: Component) -> Option<Style> {
        match component {
            Component::Defined(serial) => Some( Self::style(theme, serial) ),
            Component::Inherited(name) => match Theme::extract( theme, name ) {
                Some(radio) => Some( radio.active.clone() ),
                _ => None,
            },
            _ => None,
        }
    }

    /// Creates or inherits the hovered style.
    fn hovered(theme: &Collection, component: Component) -> Option<Style> {
        match component {
            Component::Defined(serial) => Some( Self::style(theme, serial) ),
            Component::Inherited(name) => match Theme::extract( theme, name ) {
                Some(radio) => Some( radio.hovered.clone() ),
                _ => None,
            },
            _ => None,
        }
    }

    /// Attempts to create a `Style` from the given serial style.
    fn style(theme: &Collection, serial: StateSerial) -> Style {
        // Destructure the serial.
        let StateSerial { background, dotcolor, textcolor, border } = serial;

        // Get the background color.
        let background = match theme.color.get(&background) {
            Some(c) => (*c).into(),
            _ => Color::WHITE.into(),
        };

        // Get the dot color.
        let dot_color = match theme.color.get(&dotcolor) {
            Some(c) => (*c).into(),
            _ => Color::RED.into(),
        };

        // Get the text color.
        let text_color = match textcolor {
            Some(name) => match theme.color.get(&name) {
                Some(c) => Some((*c).into()),
                _ => None,
            },
            _ => None,
        };

        // Get the border theme.
        let (border_color, _, border_width) = match Border::extract(theme, border) {
            Some(b) => {
                (b.color.into(), b.radius, b.width)
            },
            _ => {
                let b = Border::DEFAULT;
                (b.color.into(), b.radius, b.width)
            },
        };

        Style { background, dot_color, text_color, border_width, border_color }
    }
}

impl StyleSheet for Theme {
    fn active(&self) -> Style {
        self.active.clone()
    }

    fn hovered(&self) -> Style {
        self.hovered.clone()
    }
}
