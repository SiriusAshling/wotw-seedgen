use ordered_float::OrderedFloat;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Position {
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
}
impl Position {
    pub fn new<F: Into<OrderedFloat<f32>>>(x: F, y: F) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
