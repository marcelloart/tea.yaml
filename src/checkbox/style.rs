//! `iced` compatible version of the checkbox theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    checkbox::{
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
            Some(stylestr) => match theme.checkbox.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { active, hovered } = serial;

                    // Get the active state theme.
                    let active = match Self::active(theme, active.clone()) {
                        Some(t) => t,
                        _ => Style {
                            background: iced::Color::WHITE.into(),
                            checkmark_color: iced::Color::from_rgb8(255, 0, 0),
                            border_radius: 0.0,
                            border_width: 0.0,
                            border_color: iced::Color::BLACK,
                            text_color: None,
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

    /// Creates or inherits the active `Style`.
    fn active(theme: &Collection, component: Component) -> Option<Style> {
        match component {
            Component::Defined(serial) => Some( Self::style(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(checkbox) => Some( checkbox.active.clone() ),
                _ => None,
            },

            _ => None,
        }
    }

    /// Creates or inherits the hovered `Style`.
    fn hovered(theme: &Collection, component: Component) -> Option<Style> {
        match component {
            Component::Defined(serial) => Some( Self::style(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(checkbox) => Some( checkbox.hovered.clone() ),
                _ => None,
            },

            _ => None,
        }
    }

    /// Attempts to create a `Style` from the given serial style.
    fn style(theme: &Collection, serial: StateSerial) -> Style {
        // Destructure the serial.
        let StateSerial { background, checkmark, textcolor, border } = serial;

        // Get the background color.
        let background = match theme.color.get(&background) {
            Some(c) => (*c).into(),
            _ => Color::WHITE.into(),
        };

        // Get the checkmark color.
        let checkmark_color = match theme.color.get(&checkmark) {
            Some(c) => (*c).into(),
            _ => Color::BLACK.into(),
        };

        // Get the text color.
        let text_color = match textcolor {
            Some(s) => match theme.color.get(&s) {
                Some(c) => Some((*c).into()),
                _ => None,
            },
            _ => None,
        };

        // Get the border theme.
        let (border_color, border_radius, border_width) = match Border::extract(theme, border) {
            Some(b) => (b.color.into(), b.radius, b.width),
            _ => {
                let b = Border::DEFAULT;
                (b.color.into(), b.radius, b.width)
            },
        };

        Style {
            background,
            checkmark_color,
            border_radius,
            border_width,
            border_color,
            text_color,
        }
    }
}

impl StyleSheet for Theme {
    fn active(&self, checked: bool) -> Style {
        if checked {
            return self.active;
        }

        let mut unchecked = self.active.clone();
        unchecked.checkmark_color = iced::Color::from_rgba(0.0, 0.0, 0.0, 0.0);
        unchecked
    }

    fn hovered(&self, checked: bool) -> Style {
        if checked {
            return self.hovered;
        }

        let mut unchecked = self.hovered.clone();
        unchecked.checkmark_color = iced::Color::from_rgba(0.0, 0.0, 0.0, 0.0);
        unchecked
    }
}
