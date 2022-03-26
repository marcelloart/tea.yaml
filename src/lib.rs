//! `painter` is a dynamic theme library for the `iced` GUI framework.
//! It contains a collection of `Style` conertable structures that can 
//! be serialized using `serde`.



pub mod border;
pub mod button;
pub mod checkbox;
pub mod color;
pub mod container;
pub mod panegrid;
pub mod progressbar;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod textinput;

mod theme;



pub use self::theme::Theme;
