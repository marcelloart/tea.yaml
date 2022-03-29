//! `iced` compatible version of the text input theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    text_input::{
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

    /// Focused state.
    pub focused: StateTheme,

    /// Colors of the theme.
    pub colors: [Color; 3],
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(stylestr) => match theme.textinput.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { active, hovered, focused, colors } = serial;

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

                    // Get the focused state theme.
                    let focused = match StateTheme::focused(theme, focused.clone()) {
                        Some(t) => t,
                        _ => active.clone(),
                    };

                    // Get the colors of the theme.
                    let colors = {
                        // Destructure the colors.
                        let [placeholder, value, selection] = colors;

                        let placeholder = match theme.color.get(placeholder) {
                            Some(c) => *c,
                            _ => Color::BLACK,
                        };

                        let value = match theme.color.get(value) {
                            Some(c) => *c,
                            _ => Color::BLACK,
                        };

                        let selection = match theme.color.get(selection) {
                            Some(c) => *c,
                            _ => Color::RED,
                        };

                        [placeholder, value, selection]
                    };

                    Some( Theme { active, hovered, focused, colors } )
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

    fn focused(&self) -> Style {
        self.focused.into()
    }

    fn placeholder_color(&self) -> iced::Color {
        self.colors[0].into()
    }

    fn value_color(&self) -> iced::Color {
        self.colors[1].into()
    }

    fn selection_color(&self) -> iced::Color {
        self.colors[2].into()
    }
}



#[derive(Clone, Copy, Debug)]
pub struct StateTheme {
    /// Background color.
    pub background: Color,

    /// Border theme.
    pub border: Border,
}

impl StateTheme {
    /// Default state theme.
    pub const DEFAULT: StateTheme = StateTheme { background: Color::WHITE, border: Border::DEFAULT };

    /// Gets the `StateTheme` from a defined serial.
    pub(self) fn defined(theme: &Collection, serial: StateSerial) -> Self {
        // Destructure the serial.
        let StateSerial { background, border } = serial;

        // Get the background color.
        let background = match theme.color.get(&background) {
            Some(c) => *c,
            _ => Color::WHITE,
        };

        // Get the border theme.
        let border = match Border::extract(theme, border) {
            Some(b) => b,
            _ => Border::DEFAULT,
        };

        StateTheme { background, border }
    }

    /// Gets the `StateTheme` from the active component.
    pub(self) fn active(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(button) => Some( button.active.clone() ),
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
                Some(button) => Some( button.hovered.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }

    /// Gets the `StateTheme` from the focused component.
    pub(self) fn focused(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(button) => Some( button.focused.clone() ),
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
            border_radius: self.border.radius,
            border_width: self.border.width,
        }
    }
}
