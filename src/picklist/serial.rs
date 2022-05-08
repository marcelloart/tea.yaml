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
pub struct PicklistState {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub textcolor: String,

    /// Key to the placeholder color.
    pub placeholdercolor: String,

    /// Key to the border theme.
    pub border: String,

    /// Icon size.
    pub iconsize: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Menu {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub textcolor: String,

    /// Key to the border theme.
    pub border: String,

    /// Key to the selected background color.
    pub sbackground: String,

    /// Key to the selected text color.
    pub stextcolor: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StateComponent {
    /// The button state is defined.
    Defined(PicklistState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MenuComponent {
    /// The button state is defined.
    Defined(Menu),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}
