//! `serde` compatible version of the pane grid theme.



use serde_derive::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaneGrid {
    /// Hovered state.
    pub region: HoveredComponent,

    /// Picked state.
    pub picked: LineComponent,

    /// Hovered state.
    pub hovered: LineComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hovered {
    /// Backgroud color of the region.
    pub background: String, 

    /// Border of the region.
    pub border: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HoveredComponent {
    /// The line state is defined.
    Defined( Hovered ),

    /// The line state is inherited from another pane grid theme.
    Inherited( String ),

    /// The line state is not defined.
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    /// The color of the line.
    pub color: String, 

    /// Width of the line.
    pub width: f32,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LineComponent {
    /// The line state is defined.
    Defined( State ),

    /// The line state is inherited from another pane grid theme.
    Inherited( String ),

    /// The line state is not defined.
    None,
}
