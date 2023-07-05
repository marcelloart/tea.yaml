//! Border theme.



pub(crate) mod serial;



use crate::{ Color, Theme };

use std::sync::Arc;



#[derive(Clone, Debug)]
pub struct Border {
    /// Border color.
    pub color: Arc<Color>,

    /// Radius of the border.
    pub radius: f32,

    /// Width of the border.
    pub width: f32,
}

impl Border {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::Border, theme: &Theme) -> Result<Self, ()> {
        match theme.color.get(&serial.color) {
            Some(color) => Ok( Self { color: color.clone(), radius: serial.radius, width: serial.width } ),
            _ => Err(()),
        }
    }
}
