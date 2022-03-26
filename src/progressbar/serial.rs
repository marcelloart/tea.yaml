//! `serde` compatible version of the container theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProgressBar {
    /// Key to the background color.
    pub background: String,

    /// Key to the bar color.
    pub bar: String,

    /// Border radius.
    pub radius: f32,
}
