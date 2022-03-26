//! `serde` compatible version of the rule theme.



use serde::{ Deserialize, Serialize };



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    /// Key to the color.
    pub color: String,

    /// Fill mode of the rule.
    pub fillmode: FillMode,

    /// Radius of the rule.
    pub radius: f32,

    /// Width of the rule.
    pub width: u16,
}


/// Copied from `iced` for serialization.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FillMode {
    AsymmetricPadding(u16, u16),
    Full,
    Padded(u16),
    Percent(f32),
}

impl FillMode {
    pub fn convert(&self) -> iced::widget::rule::FillMode {
        match *self {
            FillMode::AsymmetricPadding(a, b) => iced::widget::rule::FillMode::AsymmetricPadding(a, b),
            FillMode::Full => iced::widget::rule::FillMode::Full,
            FillMode::Padded(a) => iced::widget::rule::FillMode::Padded(a),
            FillMode::Percent(a) => iced::widget::rule::FillMode::Percent(a),
        }
    }
}
