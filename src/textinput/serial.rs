//! `serde` compatible version of the text input theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextInput {
    /// Active state.
    pub active: Component,

    /// Hovered state.
    pub hovered: Component,

    /// Focused state.
    pub focused: Component,

    /// Disabled state.
    pub disabled: Component,

    /// Placeholder color.
    pub placeholder: String,

    /// Value color.
    pub value: String,

    /// Selection color.
    pub selection: String,

    /// Disabled color.
    pub disabledc: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    /// Key to the background color.
    pub background: String,

    /// Key to the border theme.
    pub border: String,

    /// Key to the icon color.
    pub icon: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The button state is defined.
    Defined( State ),

    /// The button state is inherited from another button theme.
    Inherited( String ),

    /// The button state is not defined.
    None,
}
