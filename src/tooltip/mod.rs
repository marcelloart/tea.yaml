//! Tooltip theme.



pub(crate) mod serial;



use crate::{ Border, Color, Theme };

use iced_native::{
    widget::{
        container::{
            Appearance, StyleSheet,
        },
    },
};

use std::sync::Arc;



#[derive(Clone, Debug)]
pub struct Tooltip {
    /// Background color.
    pub background: Arc<Color>,

    /// Text color.
    pub text: Arc<Color>,

    /// Border theme.
    pub border: Arc<Border>,
}

impl Tooltip {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::Tooltip, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let background = match theme.color.get(&serial.background) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the text color.
        let text = match theme.color.get(&serial.text) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(&serial.border) {
            Some(border) => border.clone(),
            _ => return Err(()),
        };

        Ok( Tooltip { background, text, border } )
    }
}

impl StyleSheet for Tooltip {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: None,
            background: Some( (*self.background).into() ),
            border_radius: self.border.radius,
            border_width: self.border.width,
            border_color: (*self.border.color).into(),
        }
    }
}
