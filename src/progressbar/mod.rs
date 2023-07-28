//! Progress bar theme.



pub(crate) mod serial;



use crate::{ Color, Theme, };

use iced::{
    BorderRadius,

    widget::progress_bar::{
        Appearance, StyleSheet,
    }
};

use std::sync::Arc;



#[derive(Clone, Debug)]
pub struct ProgressBar {
    /// Background color.
    pub background: Arc<Color>,

    /// Bar color.
    pub bar: Arc<Color>,

    /// Border radius.
    pub radius: BorderRadius,
}

impl ProgressBar {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::ProgressBar, theme: &Theme) -> Result<Self, ()> {
        // Get the color of the progress bar background.
        let background = match theme.color.get(&serial.background) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the border of the progress bar bar.
        let bar = match theme.color.get(&serial.bar) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        Ok( ProgressBar { background, bar, radius: BorderRadius::from( serial.radius ) } )
    }
}

impl StyleSheet for ProgressBar {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: (*self.background).into(),
            bar: (*self.bar).into(),
            border_radius: self.radius,
        }
    }
}
