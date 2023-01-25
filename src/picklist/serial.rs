//! `serde` compatible version of the pick list theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Picklist {
    /// Active state.
    pub active: StateComponent,

    /// Hovered state.
    pub hovered: StateComponent,

    /// Menu theme.
    pub menu: MenuComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub text: String,

    /// Key to the placeholder color.
    pub placeholder: String,

    /// Key to the border theme.
    pub border: String,

    /// Handle color.
    pub handle: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Menu {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub text: String,

    /// Key to the border theme.
    pub border: String,

    /// Key to the selected background color.
    pub sbackground: String,

    /// Key to the selected text color.
    pub stext: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StateComponent {
    /// The button state is defined.
    Defined( State ),

    /// The button state is inherited from another button theme.
    Inherited( String ),

    /// The button state is not defined.
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MenuComponent {
    /// The button state is defined.
    Defined( Menu ),

    /// The button state is inherited from another button theme.
    Inherited( String ),
}
