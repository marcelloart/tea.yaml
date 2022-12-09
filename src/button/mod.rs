//! Button theme.



pub(crate) mod serial;



use crate::{ Border, Color, Theme };

use iced_native::{
    Vector,

    widget::{
        button::{
            Appearance, StyleSheet
        },
    },
};

use serial::Component;


#[derive(Clone, Copy, Debug)]
pub struct Button {
    /// State Themes of the button.
    /// In order: active, hovered, pressed, disabled.
    pub state: [State; 4]
}

impl Button {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::Button, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let active   = Self::state( &serial.active  , theme, 0 )?;
        let hovered  = Self::state( &serial.hovered , theme, 1 )?;
        let pressed  = Self::state( &serial.pressed , theme, 2 )?;
        let disabled = Self::state( &serial.disabled, theme, 3 )?;
        
        // Find the first state theme that is not None.
        let default = match (active, hovered, pressed, disabled) {
            (Some(d), _, _, _) => d,
            (_, Some(d), _, _) => d,
            (_, _, Some(d), _) => d,
            (_, _, _, Some(d)) => d,

            _ => return Err(()),
        };

        Ok(Button {
            state: [
                if active.is_some()   { active.unwrap()   } else { default },
                if hovered.is_some()  { hovered.unwrap()  } else { default },
                if pressed.is_some()  { pressed.unwrap()  } else { default },
                if disabled.is_some() { disabled.unwrap() } else { default },
            ],
        })
    }

    fn state(serial: &Component, theme: &Theme, index: usize) -> Result<Option<State>, ()> {
        match serial {
            Component::Defined( state ) => Ok( Some( State::from(&state, &theme)? ) ),

            Component::Inherited( name ) => match theme.button.get( name ) {
                Some( button ) => Ok( Some( button.state[index].clone() ) ),
                _ => Err(()),
            },

            Component::None => Ok( None ),
        }
    }
}

impl StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some( self.state[0].background.into() ),
            border_radius: self.state[0].border.radius,
            border_width: self.state[0].border.width,
            border_color: self.state[0].border.color.into(),
            text_color: self.state[0].text.into()
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some( self.state[1].background.into() ),
            border_radius: self.state[1].border.radius,
            border_width: self.state[1].border.width,
            border_color: self.state[1].border.color.into(),
            text_color: self.state[1].text.into()
        }
    }

    fn pressed(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some( self.state[2].background.into() ),
            border_radius: self.state[2].border.radius,
            border_width: self.state[2].border.width,
            border_color: self.state[2].border.color.into(),
            text_color: self.state[2].text.into()
        }
    }

    fn disabled(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some( self.state[3].background.into() ),
            border_radius: self.state[3].border.radius,
            border_width: self.state[3].border.width,
            border_color: self.state[3].border.color.into(),
            text_color: self.state[3].text.into()
        }
    }
}



#[derive(Clone, Copy, Debug)]
pub struct State {
    /// Background color.
    pub(crate) background: Color, 

    /// Text color.
    pub(crate) text: Color,

    /// Border theme.
    pub(crate) border: Border,
}

impl State {
    /// Attempts to create a theme from its serialized version.
    fn from(serial: &serial::State, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let background = match theme.color.get(&serial.background) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the text color.
        let text = match theme.color.get(&serial.text) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(&serial.border) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok( State { background, text, border } )
    }
}
