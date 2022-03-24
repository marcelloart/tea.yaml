//! `serde` compatible version of the button theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Button {
    /// Active state.
    pub active: Component,

    /// Hovered state.
    pub hovered: Component,

    /// Pressed state.
    pub pressed: Component,

    /// Disabled state.
    pub disabled: Component,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButtonState {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub textcolor: String,

    /// Key to the border theme.
    pub border: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The button state is defined.
    Defined(ButtonState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}
