//! `serde` compatible version of the checkbox theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Checkbox {
    /// Active state.
    pub active: Component,

    /// Hovered state.
    pub hovered: Component,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckboxState {
    /// Key to the background color.
    pub background: String,

    /// Key to the checkmark color.
    pub checkmark: String,

    /// Key to the text color.
    pub textcolor: Option<String>,

    /// Key to the border theme.
    pub border: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The button state is defined.
    Defined(CheckboxState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}
