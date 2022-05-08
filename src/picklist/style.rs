//! `iced` compatible version of the picklist theme.



use crate::{
    Theme as Collection,
    color::Color,
    border::Theme as Border,
};

use iced::{
    pick_list::{
        Menu, Style, StyleSheet,
    },
};

use super::{ MenuComponent, MenuSerial, StateComponent, Serial, StateSerial };



#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Active state.
    pub active: Style,

    /// Hovered state.
    pub hovered: Style,

    /// Menu theme.
    pub menu: Menu,
}

impl Theme {
    /// Default style of the theme.
    pub const DEFAULTSTYLE: Style = Style {
        background: iced::Background::Color( iced::Color::WHITE ),
        text_color: iced::Color::BLACK,

        placeholder_color: iced::Color::BLACK,

        border_color: iced::Color::BLACK,
        border_radius: 0.0,
        border_width: 1.0,

        icon_size: 1.0,
    };

    /// Default style of the menu.
    pub const DEFAULTMENU: Menu = Menu {
        background: iced::Background::Color( iced::Color::WHITE ),
        text_color: iced::Color::BLACK,

        border_color: iced::Color::BLACK,
        border_width: 1.0,

        selected_background: iced::Background::Color( iced::Color::WHITE ),
        selected_text_color: iced::Color::BLACK,
    };

    /// Gets the background theme with the given key (if it exists).
    pub fn extract(theme: &Collection, name: String) -> Option<Self> {
        match theme.styles.get(&name) {
            Some(stylestr) => match theme.picklist.get(&stylestr.0) {
                Some(serial) => {
                    // Destructure the serialized version.
                    let Serial { active, hovered, menu } = serial;

                    // Get the active state theme.
                    let active = match Self::active(theme, active.clone()) {
                        Some(t) => t,
                        _ => Self::DEFAULTSTYLE,
                    };

                    // Get the hovered state theme.
                    let hovered = match Self::hovered(theme, hovered.clone()) {
                        Some(t) => t,
                        _ => active.clone(),
                    };

                    // Get the menu theme.
                    let menu = match Self::menu(theme, menu.clone()) {
                        Some(t) => t,
                        _ => Self::DEFAULTMENU,
                    };

                    Some( Theme { active, hovered, menu } )
                },

                _ => None,
            },

            _ => None,
        }
    }

    /// Attempts to create a `Menu` from the given serial style.
    pub fn menu(theme: &Collection, component: MenuComponent) -> Option<Menu> {
        match component {
            MenuComponent::Defined(serial) => {
                // Destructure the serial.
                let MenuSerial { background, textcolor, border, sbackground, stextcolor } = serial;

                // Get the background color.
                let background = match theme.color.get(&background) {
                    Some(c) => (*c).into(),
                    _ => Color::WHITE.into(),
                };

                // Get the text color.
                let text_color = match theme.color.get(&textcolor) {
                    Some(c) => (*c).into(),
                    _ => Color::BLACK.into(),
                };

                // Get the selected background color.
                let selected_background = match theme.color.get(&sbackground) {
                    Some(c) => (*c).into(),
                    _ => Color::WHITE.into(),
                };

                // Get the selected text color.
                let selected_text_color = match theme.color.get(&stextcolor) {
                    Some(c) => (*c).into(),
                    _ => Color::BLACK.into(),
                };

                // Get the border theme.
                let (border_color, border_width) = match Border::extract(theme, border) {
                    Some(b) => {
                        (b.color.into(), b.width)
                    },
                    _ => {
                        let b = Border::DEFAULT;
                        (b.color.into(), b.width)
                    },
                };

                Some( Menu {
                    background,
                    text_color,

                    border_color,
                    border_width,

                    selected_background,
                    selected_text_color,
                })
            },

            MenuComponent::Inherited(name) => match Theme::extract(theme, name) {
                Some(picklist) => Some( picklist.menu.clone() ),
                _ => None,
            },

            MenuComponent::None => None,
        }
    }

    /// Creates or inherits the active `Style`.
    pub fn active(theme: &Collection, component: StateComponent) -> Option<Style> {
        match component {
            StateComponent::Defined(serial) => Some( Self::style(theme, serial) ),

            StateComponent::Inherited(name) => match Theme::extract(theme, name) {
                Some(picklist) => Some( picklist.active.clone() ),
                _ => None,
            },

            _ => None,
        }
    }

    /// Creates or inherits the hovered `Style`.
    pub fn hovered(theme: &Collection, component: StateComponent) -> Option<Style> {
        match component {
            StateComponent::Defined(serial) => Some( Self::style(theme, serial) ),

            StateComponent::Inherited(name) => match Theme::extract(theme, name) {
                Some(picklist) => Some( picklist.hovered.clone() ),
                _ => None,
            },

            _ => None,
        }
    }

    /// Attempts to create a `Style` from the given serial style.
    pub fn style(theme: &Collection, serial: StateSerial) -> Style {
        // Destructure the serial.
        let StateSerial { background, placeholdercolor, textcolor, border, iconsize } = serial;

        // Get the background color.
        let background = match theme.color.get(&background) {
            Some(c) => (*c).into(),
            _ => Color::WHITE.into(),
        };

        // Get the text color.
        let text_color = match theme.color.get(&textcolor) {
            Some(c) => (*c).into(),
            _ => Color::BLACK.into(),
        };

        // Get the placeholder color.
        let placeholder_color = match theme.color.get(&placeholdercolor) {
            Some(c) => (*c).into(),
            _ => Color::BLACK.into(),
        };

        // Get the border theme.
        let (border_color, border_radius, border_width) = match Border::extract(theme, border) {
            Some(b) => {
                (b.color.into(), b.radius, b.width)
            },
            _ => {
                let b = Border::DEFAULT;
                (b.color.into(), b.radius, b.width)
            },
        };

        Style {
            background,
            text_color,

            placeholder_color,

            border_color,
            border_radius,
                    border_width,

            icon_size: iconsize,
        }
    }
}

impl StyleSheet for Theme {
    fn active(&self) -> Style {
        self.active.clone()
    }

    fn hovered(&self) -> Style {
        self.hovered.clone()
    }

    fn menu(&self) -> Menu {
        self.menu.clone()
    }
}
