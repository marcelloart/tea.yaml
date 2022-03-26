//! Slider theme.



mod serial;
mod style;



pub use self::serial::{
    Slider as Serial,
    SliderState as StateSerial,
    Component,
};

pub use self::style::Theme;
