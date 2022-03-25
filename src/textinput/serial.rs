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

    /// Colors of the theme.
    pub colors: [String; 3],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextInputState {
    /// Key to the background color.
    pub background: String,

    /// Key to the border theme.
    pub border: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The button state is defined.
    Defined(TextInputState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}
