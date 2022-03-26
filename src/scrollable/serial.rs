//! `serde` compatible version of the scrollbar theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scrollbar {
    /// Active state.
    pub active: Component,

    /// Hovered state.
    pub hovered: Component,

    /// Dragging state.
    pub dragging: Component,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScrollbarState {
    /// Key to the background color.
    pub color: String,

    /// Key to the border theme.
    pub border: String,

    /// Key to the scroller color.
    pub scolor: String,

    /// Key to the scroller border.
    pub sborder: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The button state is defined.
    Defined(ScrollbarState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}
