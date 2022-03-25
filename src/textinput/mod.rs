//! text input theme.



mod serial;
mod style;



pub use self::serial::{
    TextInput as Serial,
    TextInputState as StateSerial,
    Component,
};
pub use self::style::Theme;
