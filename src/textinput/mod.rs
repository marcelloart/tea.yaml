//! text input theme.



pub(crate) mod serial;



use crate::{ Border, Color, Theme };

use iced_native::{
    widget::{
        text_input::{
            Appearance, StyleSheet,
        },
    },
};

use serial::Component;


#[derive(Clone, Copy, Debug)]
pub struct TextInput {
    /// State Themes of the text input.
    /// In order: active, hovered, focused.
    pub state: [State; 3],

    /// Colors of the text input.
    pub colors: [Color; 3],
}

impl TextInput {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::TextInput, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let active  = Self::state( &serial.active , theme, 0 )?;
        let hovered = Self::state( &serial.hovered, theme, 1 )?;
        let focused = Self::state( &serial.focused, theme, 2 )?;
        
        // Find the first state theme that is not None.
        let default = match (active, hovered, focused) {
            (Some(d), _, _) => d,
            (_, Some(d), _) => d,
            (_, _, Some(d)) => d,

            _ => return Err(()),
        };

        // Get the placeholder color.
        let placeholder = match theme.color.get( &serial.placeholder ) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the placeholder color.
        let value = match theme.color.get( &serial.value ) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the placeholder color.
        let selection = match theme.color.get( &serial.selection ) {
            Some(color) => *color,
            _ => return Err(()),
        };

        Ok(TextInput {
            state: [
                if active.is_some()  { active.unwrap()  } else { default },
                if hovered.is_some() { hovered.unwrap() } else { default },
                if focused.is_some() { focused.unwrap() } else { default },
            ],

            colors: [
                placeholder,
                value,
                selection,
            ],
        })
    }

    fn state(serial: &Component, theme: &Theme, index: usize) -> Result<Option<State>, ()> {
        match serial {
            Component::Defined( state ) => Ok( Some( State::from(&state, &theme)? ) ),

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
            background: self.state[0].background.into(),
            border_radius: self.state[0].border.radius,
            border_width: self.state[0].border.width,
            border_color: self.state[0].border.color.into(),
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: self.state[1].background.into(),
            border_radius: self.state[1].border.radius,
            border_width: self.state[1].border.width,
            border_color: self.state[1].border.color.into(),
        }
    }

    fn focused(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: self.state[2].background.into(),
            border_radius: self.state[2].border.radius,
            border_width: self.state[2].border.width,
            border_color: self.state[2].border.color.into(),
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> iced::Color {
        self.colors[0].into()
    }

    fn value_color(&self, _: &Self::Style) -> iced::Color {
        self.colors[1].into()
    }

    fn selection_color(&self, _: &Self::Style) -> iced::Color {
        self.colors[2].into()
    }
}



#[derive(Clone, Copy, Debug)]
pub struct State {
    /// Background color.
    pub background: Color, 

    /// Border theme.
    pub border: Border,
}

impl State {
    /// Attempts to create a theme from its serialized version.
    fn from(serial: &serial::State, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let background = match theme.color.get(&serial.background) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(&serial.border) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok( State { background, border } )
    }
}
