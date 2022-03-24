//! `iced` compatible version of the button theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    Vector,
    button::{
        Style, StyleSheet,
    },
};

use super::{ Component, Serial, StateSerial };



#[derive(Clone, Debug)]
pub struct Theme {
    /// Active state.
    pub active: StateTheme,

    /// Hovered state.
    pub hovered: StateTheme,

    /// Pressed state.
    pub pressed: StateTheme,

    /// Disabled state.
    pub disabled: StateTheme,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(stylestr) => match theme.button.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { active, hovered, pressed, disabled } = serial;

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

                    // Get the pressed state theme.
                    let pressed = match StateTheme::pressed(theme, pressed.clone()) {
                        Some(t) => t,
                        _ => active.clone(),
                    };

                    // Get the disabled state theme.
                    let disabled = match StateTheme::disabled(theme, disabled.clone()) {
                        Some(t) => t,
                        _ => active.clone(),
                    };

                    Some( Theme { active, hovered, pressed, disabled } )
                },

                _ => None,
            },

            _ => None,
        }
    }
}

impl StyleSheet for Theme {
    fn active(&self) -> Style {
        Style {
            background: Some( self.active.background.into() ),
            border_color: self.active.border.color.into(),
            border_radius: self.active.border.radius,
            border_width: self.active.border.width,
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: self.active.textcolor.into(),
        }
    }
}



#[derive(Clone, Debug)]
pub struct StateTheme {
    /// Background color.
    background: Color,

    /// Text color.
    textcolor: Color,

    /// Border theme.
    border: Border,
}

impl StateTheme {
    /// Default state theme.
    pub const DEFAULT: StateTheme = StateTheme { background: Color::BUTTONBG, textcolor: Color::BUTTONTXT, border: Border::BUTTON };

    /// Gets the `StateTheme` from a defined serial.
    pub(self) fn defined(theme: &Collection, serial: StateSerial) -> Self {
        // Destructure the serial.
        let StateSerial { background, textcolor, border } = serial;

        // Get the background color.
        let background = match theme.color.get(&background) {
            Some(c) => *c,
            _ => Color::BUTTONBG,
        };

        // Get the text color.
        let textcolor = match theme.color.get(&textcolor) {
            Some(c) => *c,
            _ => Color::BUTTONTXT,
        };

        // Get the border theme.
        let border = match Border::extract(theme, border) {
            Some(b) => b,
            _ => Border::BUTTON,
        };

        StateTheme { background, textcolor, border }
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

    /// Gets the `StateTheme` from the pressed component.
    pub(self) fn pressed(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(button) => Some( button.pressed.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }

    /// Gets the `StateTheme` from the disabled component.
    pub(self) fn disabled(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(button) => Some( button.disabled.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }
}
