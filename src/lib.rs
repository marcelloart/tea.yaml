//! `painter` is a dynamic theme library for the `iced` GUI framework.
//! It contains a collection of `Style` conertable structures that can be
//! serialized using `serde`.



pub mod border;
pub mod button;
//pub mod checkbox;
pub mod color;
pub mod container;
pub mod panegrid;
pub mod picklist;
pub mod progressbar;
//pub mod radio;
//pub mod rule;
pub mod scrollable;
//pub mod slider;
pub mod textinput;
pub mod tooltip;

pub mod serial;

pub mod theme;



pub use border::Border;
pub use button::Button;
pub use color::Color;
pub use container::Container;
pub use panegrid::PaneGrid;
pub use picklist::Picklist;
pub use progressbar::ProgressBar;
pub use scrollable::Scrollable;
pub use textinput::TextInput;
pub use tooltip::Tooltip;

pub use theme::Theme;



#[cfg(all(test, feature = "dev"))]
mod tests;
