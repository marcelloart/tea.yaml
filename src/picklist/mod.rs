//! Pick list theme.



pub(crate) mod serial;



use crate::{ Border, Color, Theme };

use iced_native::{
    widget::{
        pick_list::{
            Appearance, StyleSheet,
        },
    },
};

use serial::{ MenuComponent, StateComponent };



#[derive(Clone, Copy, Debug)]
pub struct Picklist {
    /// Active state.
    pub state: [State; 2],

    /// Menu style.
    pub menu: Menu,
}

impl Picklist {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::Picklist, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let active   = Self::state( &serial.active  , theme, 0 )?;
        let hovered  = Self::state( &serial.hovered , theme, 1 )?;

        // Get the menu style.
        let menu = Self::menu( &serial.menu, theme )?;
        
        // Find the first state theme that is not None.
        let default = match (active, hovered) {
            (Some(d), _) => d,
            (_, Some(d)) => d,

            _ => return Err(()),
        };

        Ok(Picklist {
            state: [
                if active.is_some()   { active.unwrap()   } else { default },
                if hovered.is_some()  { hovered.unwrap()  } else { default },
            ],

            menu,
        })
    }

    fn state(serial: &serial::StateComponent, theme: &Theme, index: usize) -> Result<Option<State>, ()> {
        match serial {
            StateComponent::Defined( state ) => Ok( Some( State::from(&state, &theme)? ) ),

            StateComponent::Inherited( name ) => match theme.picklist.get( name ) {
                Some( picklist ) => Ok( Some( picklist.state[index].clone() ) ),
                _ => Err(()),
            },

            StateComponent::None => Ok( None ),
        }
    }

    fn menu(serial: &serial::MenuComponent, theme: &Theme) -> Result<Menu, ()> {
        match serial {
            MenuComponent::Defined( state ) => Ok( Menu::from(&state, &theme)? ),

            MenuComponent::Inherited( name ) => match theme.picklist.get( name ) {
                Some( picklist ) => Ok( picklist.menu.clone() ),
                _ => Err(()),
            },
        }
    }
}

impl StyleSheet for Picklist {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: self.state[0].text.into(),
            placeholder_color: self.state[0].placeholder.into(),
            background: self.state[0].background.into(),
            border_radius: self.state[0].border.radius,
            border_width: self.state[0].border.width,
            border_color: self.state[0].border.color.into(),
            icon_size: self.state[0].iconsize,
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: self.state[1].text.into(),
            placeholder_color: self.state[1].placeholder.into(),
            background: self.state[1].background.into(),
            border_radius: self.state[1].border.radius,
            border_width: self.state[1].border.width,
            border_color: self.state[1].border.color.into(),
            icon_size: self.state[1].iconsize,
        }
    }
}



#[derive(Clone, Copy, Debug)]
pub struct State {
    /// Background color.
    pub background: Color, 

    /// Text color.
    pub text: Color,

    /// Placeholder color.
    pub placeholder: Color,

    /// Border theme.
    pub border: Border,

    /// Icon size.
    pub iconsize: f32,
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

        // Get the placeholder color.
        let placeholder = match theme.color.get(&serial.placeholder) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(&serial.border) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok( State { background, text, placeholder, border, iconsize: serial.iconsize } )
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Menu {
    /// Key to the background color.
    pub background: [Color; 2],

    /// Key to the text color.
    pub text: [Color; 2],

    /// Key to the border theme.
    pub border: Border,
}

impl Menu {
    /// Attempts to create a theme from its serialized version.
    fn from(serial: &serial::Menu, theme: &Theme) -> Result<Self, ()> {
        // Get the background colors.
        let background = match theme.color.get(&serial.background) {
            Some(color) => *color,
            _ => return Err(()),
        };

        let sbackground = match theme.color.get(&serial.sbackground) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the text color.
        let text = match theme.color.get(&serial.text) {
            Some(color) => *color,
            _ => return Err(()),
        };

        let stext = match theme.color.get(&serial.stext) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(&serial.border) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok( Menu { background: [background, sbackground], text: [text, stext], border } )
    }
}
