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
    pub active: StateTheme,

    /// Hovered state.
    pub hovered: StateTheme,
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
    fn active(&self, checked: bool) -> Style {
        self.active.style(checked)
    }

    fn hovered(&self, checked: bool) -> Style {
        self.hovered.style(checked)
    }
}



#[derive(Clone, Copy, Debug)]
pub struct StateTheme {
    /// Background color.
    pub background: Color,

    /// Checkmark color.
    pub checkmark: Color,

    /// Border theme.
    pub border: Border,
}

impl StateTheme {
    /// Default state theme.
    pub const DEFAULT: StateTheme = StateTheme { background: Color::WHITE, checkmark: Color::RED, border: Border::DEFAULT };

    /// Gets the `StateTheme` from a defined serial.
    pub(self) fn defined(theme: &Collection, serial: StateSerial) -> Self {
        // Destructure the serial.
        let StateSerial { background, checkmark, border } = serial;

        // Get the background color.
        let background = match theme.color.get(&background) {
            Some(c) => *c,
            _ => Color::WHITE,
        };

        // Get the checkmark color.
        let checkmark = match theme.color.get(&checkmark) {
            Some(c) => *c,
            _ => Color::RED,
        };

        // Get the border theme.
        let border = match Border::extract(theme, border) {
            Some(b) => b,
            _ => Border::DEFAULT,
        };

        StateTheme { background, checkmark, border }
    }

    /// Gets the `StateTheme` from the active component.
    pub(self) fn active(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(checkbox) => Some( checkbox.active.clone() ),
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
                Some(checkbox) => Some( checkbox.hovered.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }

    /// Converts into a style.
    pub(self) fn style(&self, checked: bool) -> Style {
        let checkmark_color = if checked { self.checkmark.into() }
            else { iced::Color::from_rgba(0.0, 0.0, 0.0, 0.0) };

        Style {
            background: self.background.into(),
            border_color: self.border.color.into(),
            border_radius: self.border.radius,
            border_width: self.border.width,
            checkmark_color,
        }
    }
}
