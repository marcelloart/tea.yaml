//! `iced` compatible version of the scrollbar theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    scrollable::{
        Scrollbar, Scroller, StyleSheet,
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
            Some(stylestr) => match theme.scrollable.get(&stylestr.0) {
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
    fn active(&self) -> Scrollbar {
        self.active.into()
    }

    fn hovered(&self) -> Scrollbar {
        self.hovered.into()
    }

    fn dragging(&self) -> Scrollbar {
        self.dragging.into()
    }
}



#[derive(Clone, Copy, Debug)]
pub struct StateTheme {
    /// Background color.
    pub color: Color,

    /// Border theme.
    pub border: Border,

    /// Scroller color.
    pub scolor: Color,

    /// Scroller border theme.
    pub sborder: Border,
}

impl StateTheme {
    /// Default state theme.
    pub const DEFAULT: StateTheme = StateTheme { color: Color::WHITE, border: Border::DEFAULT, scolor: Color::RED, sborder: Border::DEFAULT };

    /// Gets the `StateTheme` from a defined serial.
    pub(self) fn defined(theme: &Collection, serial: StateSerial) -> Self {
        // Destructure the serial.
        let StateSerial { color, border, scolor, sborder } = serial;

        // Get the background color.
        let color = match theme.color.get(&color) {
            Some(c) => *c,
            _ => Color::WHITE,
        };

        // Get the border theme.
        let border = match Border::extract(theme, border) {
            Some(b) => b,
            _ => Border::DEFAULT,
        };

        // Get the scroller color.
        let scolor = match theme.color.get(&scolor) {
            Some(c) => *c,
            _ => Color::WHITE,
        };

        // Get the scroller border theme.
        let sborder = match Border::extract(theme, sborder) {
            Some(b) => b,
            _ => Border::DEFAULT,
        };

        StateTheme { color, border, scolor, sborder }
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

impl core::convert::Into<Scrollbar> for StateTheme {
    fn into(self) -> Scrollbar {
        Scrollbar {
            background: Some( self.color.into() ),
            border_color: self.border.color.into(),
            border_radius: self.border.radius,
            border_width: self.border.width,

            scroller: Scroller {
                border_color: self.sborder.color.into(),
                border_radius: self.sborder.radius,
                border_width: self.sborder.width,
                color: self.scolor.into(),
            },
        }
    }
}
