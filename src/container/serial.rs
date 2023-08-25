//! `serde` compatible version of the container theme.



use serde_derive::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Container {
    /// Key to the background color.
    pub color: String,

    /// Key to the border definition.
    pub border: String,
}
