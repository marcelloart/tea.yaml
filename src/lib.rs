//! `painter` is a dynamic theme library for the `iced` GUI framework.
//! It contains a collection of `Style` conertable structures that can 
//! be serialized using `serde`.



pub mod border;
pub mod button;
pub mod color;
pub mod container;

mod theme;



pub use self::theme::Theme;
