//! `serde` compatible version of the slider theme.



use serde_derive::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Slider {
    /// Active state.
    pub active: Component,

    /// Hovered state.
    pub hovered: Component,

    /// Dragging state.
    pub dragging: Component,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SliderState {
    /// Key to the rail colors.
    pub colors: (String, String),

    /// Border of the handle.
    pub hborder: String,

    /// Color of the handle.
    pub hcolor: String,

    /// Shape of the handle.
    pub shape: HandleShape,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component {
    /// The slider state is defined.
    Defined(SliderState),

    /// The slider state is inherited from another slider theme.
    Inherited(String),

    /// The slider state is not defined.
    None,
}

/// Copied from `iced` for serialization.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HandleShape {
    Circle(f32),
    Rectangle(u16, f32),
}

impl HandleShape {
    pub fn convert(&self) -> iced::widget::slider::HandleShape {
        match *self {
            HandleShape::Circle(radius) => iced::widget::slider::HandleShape::Circle { radius },
            HandleShape::Rectangle(width, border_radius) => iced::widget::slider::HandleShape::Rectangle { width, border_radius },
        }
    }
}
