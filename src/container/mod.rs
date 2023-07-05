//! Container theme.



pub(crate) mod serial;



use crate::{ Border, Color, Theme };

use iced::{
    widget::{
        container::{
            Appearance, StyleSheet
        },
    },
};

use std::sync::Arc;



#[derive(Clone, Debug)]
pub struct Container {
    /// Background of the container.
    pub color: Arc<Color>,

    /// Border of the container.
    pub border: Arc<Border>,
}

impl Container {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::Container, theme: &Theme) -> Result<Self, ()> {
        // Get the color of the container.
        let color = match theme.color.get(&serial.color) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the border of the container.
        let border = match theme.border.get(&serial.border) {
            Some(border) => border.clone(),
            _ => return Err(()),
        };

        Ok( Container { color, border } )
    }
}

impl Into<iced::theme::Container> for Container {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom( Box::new(self) )
    }
}

impl StyleSheet for Container {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: None,
            background: Some( (*self.color).into() ),
            border_radius: self.border.radius,
            border_width: self.border.width,
            border_color: (*self.border.color).into(),
        }
    }
}
