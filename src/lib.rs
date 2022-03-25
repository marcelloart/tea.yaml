//! `painter` is a dynamic theme library for the `iced` GUI framework.
//! It contains a collection of `Style` conertable structures that can 
//! be serialized using `serde`.



pub mod border;
pub mod button;
pub mod checkbox;
pub mod color;
pub mod container;
pub mod textinput;

mod theme;



pub use self::theme::Theme;
