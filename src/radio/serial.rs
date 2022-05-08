//! `serde` compatible version of the radio theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Radio {
    /// Active state.
    pub active: Component,

    /// Hovered state.
    pub hovered: Component,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RadioState {
    /// Key to the background color.
    pub background: String,

    /// Key to the dot color.
    pub dotcolor: String,

    /// Key to the text color.
    pub textcolor: Option<String>,

    /// Key to the border theme.
    pub border: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The radio state is defined.
    Defined(RadioState),

    /// The button state is inherited from another radio theme.
    Inherited(String),

    /// The radio state is not defined.
    None,
}
