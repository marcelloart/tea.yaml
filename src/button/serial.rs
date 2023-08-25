//! Serial Button theme.



use serde_derive::{
    Deserialize, Serialize,
};



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Button {
    /// Active state.
    pub(super) active: Component,

    /// Hovered state.
    pub(super) hovered: Component,

    /// Pressed state.
    pub(super) pressed: Component,

    /// Disabled state.
    pub(super) disabled: Component,
}



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    /// Key to the background color.
    pub(super) background: String,

    /// Key to the text color.
    pub(super) text: String,

    /// Key to the border theme.
    pub(super) border: String,
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Component {
    /// The theme is defined.
    Defined( State ),

    /// The button state is inherited from another theme.
    Inherited( String ),

    /// The theme is not defined.
    None,
}
