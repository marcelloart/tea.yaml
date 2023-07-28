//! Pick list theme.



pub(crate) mod serial;



use crate::{ Border, Color, Theme, };

use iced::widget::pane_grid::{ Appearance, Line, StyleSheet, };

use serial::{ HoveredComponent, LineComponent, };

use std::sync::Arc;



#[derive(Clone, Debug)]
pub struct PaneGrid {
    /// Pane grid hovered region.
    pub region: Arc<Hovered>,

    /// Pane Grid states.
    pub state: [Arc<State>; 2],
}

impl PaneGrid {
    /// Attempts to create a theme from its serialized version.
    pub(crate) fn create(serial: &serial::PaneGrid, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let picked  = Self::state( &serial.picked , theme, 0 )?;
        let hovered = Self::state( &serial.hovered, theme, 1 )?;
        let region = Self::hovered(&serial.region, theme)?;
        
        // Find the first state theme that is not None.
        let default = match (&picked, &hovered) {
            (Some(d), _) => d.clone(),
            (_, Some(d)) => d.clone(),

            _ => return Err(()),
        };

        // Hovered region must be defined.
        let region = match region {
            None => return Err(()),
            Some(r) => r,
        };

        Ok(PaneGrid {
            region,
            state: [
                picked.unwrap_or(default.clone()),
                hovered.unwrap_or(default.clone()),
            ],
        })
    }

    fn hovered(serial: &serial::HoveredComponent, theme: &Theme) -> Result<Option<Arc<Hovered>>, ()> {
        match serial {
            HoveredComponent::Defined( hovered ) => Ok( Some( Arc::new( Hovered::from(&hovered, &theme)? ) ) ),

            HoveredComponent::Inherited( name ) => match theme.panegrid.get( name ) {
                Some( panegrid ) => Ok( Some( panegrid.region.clone() ) ),
                _ => Err(()),
            },

            HoveredComponent::None => Ok( None ),
        }
    }

    fn state(serial: &serial::LineComponent, theme: &Theme, index: usize) -> Result<Option<Arc<State>>, ()> {
        match serial {
            LineComponent::Defined( state ) => Ok( Some( Arc::new( State::from(&state, &theme)? ) ) ),

            LineComponent::Inherited( name ) => match theme.panegrid.get( name ) {
                Some( panegrid ) => Ok( Some( panegrid.state[index].clone() ) ),
                _ => Err(()),
            },

            LineComponent::None => Ok( None ),
        }
    }
}

impl StyleSheet for PaneGrid {
    type Style = iced::Theme;

    fn hovered_region(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: (*self.region.background).into(),
            border_width: self.region.border.width,
            border_radius: self.region.border.radius,
            border_color: (*self.region.border.color).into(),
        }
    }

    fn picked_split(&self, _: &Self::Style) -> Option<Line> {
        Some( Line {
            color: (*self.state[0].color).into(),
            width: self.state[0].width.into(),
        } )
    }

    fn hovered_split(&self, _: &Self::Style) -> Option<Line> {
        Some( Line {
            color: (*self.state[1].color).into(),
            width: self.state[1].width.into(),
        } )
    }
}



#[derive(Clone, Debug)]
pub struct Hovered {
    /// Background of the hovered region.
    background: Arc<Color>,

    /// Border of the hovered region.
    border: Arc<Border>,
}

impl Hovered {
    /// Attempts to create a theme from its serialized version.
    fn from(serial: &serial::Hovered, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let background = match theme.color.get(&serial.background) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        // Get the border.
        let border = match theme.border.get(&serial.border) {
            Some(border) => border.clone(),
            _ => return Err(()),
        };

        Ok( Hovered { background, border, } )
    }
}



#[derive(Clone, Debug)]
pub struct State {
    /// Line color.
    pub color: Arc<Color>,

    /// Line width.
    pub width: f32,
}

impl State {
    /// Attempts to create a theme from its serialized version.
    fn from(serial: &serial::State, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let color = match theme.color.get(&serial.color) {
            Some(color) => color.clone(),
            _ => return Err(()),
        };

        Ok( State { color, width: serial.width } )
    }
}
