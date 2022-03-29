//! `iced` compatible version of the scrollbar theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    slider::{
        Handle, HandleShape, Style, StyleSheet,
    },
};

use super::{ Component, Serial, StateSerial };



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Active state.
    pub active: StateTheme,

    /// Hovered state.
    pub hovered: StateTheme,

    /// Dragging state.
    pub dragging: StateTheme,
}

impl Theme {
    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(stylestr) => match theme.slider.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { active, hovered, dragging } = serial;

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

                    // Get the dragging state theme.
                    let dragging = match StateTheme::dragging(theme, dragging.clone()) {
                        Some(t) => t,
                        _ => hovered.clone(),
                    };

                    Some( Theme { active, hovered, dragging } )
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

    fn dragging(&self) -> Style {
        self.dragging.into()
    }
}



#[derive(Clone, Copy, Debug)]
pub struct StateTheme {
    /// Rail colors.
    pub colors: (Color, Color),

    /// Border of the handle theme.
    pub hborder: Border,

    /// Color of the handle.
    pub hcolor: Color,

    /// Handle shape.
    pub shape: HandleShape,
}

impl StateTheme {
    /// Default state theme.
    pub const DEFAULT: StateTheme = StateTheme { colors: (Color::WHITE, Color::BLACK), hborder: Border::DEFAULT, hcolor: Color::RED, shape: HandleShape::Circle { radius: 1.0 } };

    /// Gets the `StateTheme` from a defined serial.
    pub(self) fn defined(theme: &Collection, serial: StateSerial) -> Self {
        // Destructure the serial.
        let StateSerial { colors, hborder, hcolor, shape } = serial;

        // Get the colors.
        let colora = match theme.color.get(&colors.0) {
            Some(c) => *c,
            _ => Color::WHITE,
        };

        let colorb = match theme.color.get(&colors.1) {
            Some(c) => *c,
            _ => Color::BLACK,
        };

        let hcolor = match theme.color.get(&hcolor) {
            Some(c) => *c,
            _ => Color::BLACK,
        };

        // Get the border theme.
        let hborder = match Border::extract(theme, hborder) {
            Some(b) => b,
            _ => Border::DEFAULT,
        };

        StateTheme { colors: (colora, colorb), hborder, hcolor, shape: shape.convert() }
    }

    /// Gets the `StateTheme` from the active component.
    pub(self) fn active(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(scrollbar) => Some( scrollbar.active.clone() ),
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
                Some(scrollbar) => Some( scrollbar.hovered.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }

    /// Gets the `StateTheme` from the dragging component.
    pub(self) fn dragging(theme: &Collection, component: Component) -> Option<Self> {
        match component {
            Component::Defined(serial) => Some( Self::defined(theme, serial) ),

            Component::Inherited(name) => match Theme::extract(theme, name) {
                Some(scrollbar) => Some( scrollbar.dragging.clone() ),
                _ => None,
            },

            Component::None => None,
        }
    }
}

impl core::convert::Into<Style> for StateTheme {
    fn into(self) -> Style {
        Style {
            rail_colors: (self.colors.0.into(), self.colors.1.into()),
            handle: Handle {
                shape: self.shape,
                color: self.hcolor.into(),
                border_width: self.hborder.width,
                border_color: self.hborder.color.into(),
            },
        }
    }
}
