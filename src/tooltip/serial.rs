//! `serde` compatible version of the tooltip theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tooltip {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub textcolor: String,

    /// Key to the border definition.
    pub border: String,
}
