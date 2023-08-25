//! Serial version of the theme.



use crate::serial::*;

use serde_derive::{ Deserialize, Serialize };

use std::collections::HashMap;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Name of this theme.
    /// This can be used to index a set of themes inside a `Hashmap`.
    pub name: String,

    /// Brief description of this theme.
    /// Used mainly as a helper in the serialized files.
    pub description: String,

    /// Maps name keys to border themes.
    pub border: HashMap<String, Border>,

    // Maps name keys to button themes.
    pub button: HashMap<String, Button>,

    /// Maps name keys to colors.
    pub color: HashMap<String, Color>,

    /// Maps name keys to containers.
    pub container: HashMap<String, Container>,

    /// Maps name keys to pane grids.
    pub panegrid: HashMap<String, PaneGrid>,

    /// Maps name keys to picklists.
    pub picklist: HashMap<String, Picklist>,

    /// Maps name keys to progress bar.
    pub progressbar: HashMap<String, ProgressBar>,

    /// Maps name keys to scrollable.
    pub scrollable: HashMap<String, Scrollable>,

    /// Maps name keys to text input.
    pub textinput: HashMap<String, TextInput>,

    /// Maps name keys to tooltip.
    pub tooltip: HashMap<String, Tooltip>,
}
