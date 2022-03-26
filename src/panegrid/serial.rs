//! `serde` compatible version of the pane grid theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaneGrid {
    /// Picked state.
    pub picked: Option<Component>,

    /// Hovered state.
    pub hovered: Option<Component>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    /// The color of the line.
    pub color: String, 

    /// Width of the line.
    pub width: f32,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The line state is defined.
    Defined(Line),

    /// The line state is inherited from another pane grid theme.
    Inherited(String),

    /// The line state is not defined.
    None,
}
