//! text input theme.



pub(crate) mod serial;



use crate::{ Border, Color, Theme };

use iced::{
    widget::{
        text_input::{
            Appearance, StyleSheet,
        },
    },
};

use serial::Component;

use std::sync::Arc;



#[derive(Clone, Debug)]
pub struct TextInput {
    /// State Themes of the text input.
    /// In order: active, hovered, focused.
    pub state: [Arc<State>; 4],

    /// Colors of the text input.
    pub colors: [Arc<Color>; 4],
}

impl TextInput {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::TextInput, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let active   = Self::state( &serial.active  , theme, 0 )?;
        let hovered  = Self::state( &serial.hovered , theme, 1 )?;
        let focused  = Self::state( &serial.focused , theme, 2 )?;
        let disabled = Self::state( &serial.disabled, theme, 2 )?;
        
        // Find the first state theme that is not None.
        let default = match (&active, &hovered, &focused, &disabled) {
            (Some(d), _, _, _) => d.clone(),
            (_, Some(d), _, _) => d.clone(),
            (_, _, Some(d), _) => d.clone(),
            (_, _, _, Some(d)) => d.clone(),

            _ => return Err(()),
        };

        // Get the placeholder color.
        let placeholder = match theme.color.get( &serial.placeholder ) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the value color.
        let value = match theme.color.get( &serial.value ) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the selection color.
        let selection = match theme.color.get( &serial.selection ) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the disabled color.
        let disabledc = match theme.color.get( &serial.disabledc ) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        Ok(TextInput {
            state: [
                active.unwrap_or(default.clone()),
                hovered.unwrap_or(default.clone()),
                focused.unwrap_or(default.clone()),
                disabled.unwrap_or(default.clone()),
            ],

            colors: [
                placeholder,
                value,
                selection,
                disabledc,
            ],
        })
    }

    fn state(serial: &Component, theme: &Theme, index: usize) -> Result<Option<Arc<State>>, ()> {
        match serial {
            Component::Defined( state ) => Ok( Some( Arc::new( State::from(&state, &theme)? ) ) ),

            Component::Inherited( name ) => match theme.textinput.get( name ) {
                Some( textinput ) => Ok( Some( textinput.state[index].clone() ) ),
                _ => Err(()),
            },

            Component::None => Ok( None ),
        }
    }
}

impl StyleSheet for TextInput {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: (*self.state[0].background).into(),
            border_radius: self.state[0].border.radius,
            border_width: self.state[0].border.width,
            border_color: (*self.state[0].border.color).into(),
            icon_color: (*self.state[0].icon).into(),
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: (*self.state[1].background).into(),
            border_radius: self.state[1].border.radius,
            border_width: self.state[1].border.width,
            border_color: (*self.state[1].border.color).into(),
            icon_color: (*self.state[1].icon).into(),
        }
    }

    fn focused(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: (*self.state[2].background).into(),
            border_radius: self.state[2].border.radius,
            border_width: self.state[2].border.width,
            border_color: (*self.state[2].border.color).into(),
            icon_color: (*self.state[2].icon).into(),
        }
    }

    fn disabled(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: (*self.state[3].background).into(),
            border_radius: self.state[3].border.radius,
            border_width: self.state[3].border.width,
            border_color: (*self.state[3].border.color).into(),
            icon_color: (*self.state[3].icon).into(),
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> iced::Color {
        (*self.colors[0]).into()
    }

    fn value_color(&self, _: &Self::Style) -> iced::Color {
        (*self.colors[1]).into()
    }

    fn selection_color(&self, _: &Self::Style) -> iced::Color {
        (*self.colors[2]).into()
    }

    fn disabled_color(&self, _: &Self::Style) -> iced::Color {
        (*self.colors[3]).into()
    }
}



#[derive(Clone, Debug)]
pub struct State {
    /// Background color.
    pub background: Arc<Color>,

    /// Border theme.
    pub border: Arc<Border>,

    /// Icon color.
    pub icon: Arc<Color>,
}

impl State {
    /// Attempts to create a theme from its serialized version.
    fn from(serial: &serial::State, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let background = match theme.color.get(&serial.background) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(&serial.border) {
            Some(border) => border.clone(),
            _ => return Err(()),
        };

        // Get the icon color.
        let icon = match theme.color.get(&serial.icon) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        Ok( State { background, border, icon } )
    }
}
