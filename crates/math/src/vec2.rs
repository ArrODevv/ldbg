//! The module containing the vec2 implementation.

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use crate::traits::Vector;

/*
TODO:
- ops (+, -, *, /, +=, -=, *=, /=, -, %, %=, [], Display)
- Other traits:
- - Hash
- Conversions:
- - From f32
- - From f64
- - From i32
*/

/// A 2-component Vector using f32 as the data type.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Vec2 {
    /// The `x` component.
    x: f32,
    /// The `y` component.
    y: f32,
}

impl Vec2 {
    /// Creates a new `Vec2` with the specified `x` and `y` components.
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    /// Creates a new `Vec2` with both components set to `0.0`.
    pub fn zero() -> Self {
        Vec2 { x: 0.0f32, y: 0.0f32 }
    }
}

pub mod consts {
    //! Useful constants for use with `Vec2`.
    use super::Vec2;
    pub const ZERO: Vec2 = Vec2 { x: 0.0f32, y: 0.0f32 };
    pub const ONE: Vec2 = Vec2 { x: 1.0f32, y: 1.0f32 };

    pub const UP: Vec2 = Vec2 { x: 0.0f32, y: 1.0f32 };
    pub const DOWN: Vec2 = Vec2 { x: 0.0f32, y: -1.0f32 };
    pub const LEFT: Vec2 = Vec2 { x: -1.0f32, y: 0.0f32 };
    pub const RIGHT: Vec2 = Vec2 { x: 1.0f32, y: 0.0f32 };
}

impl Vector for Vec2 {
    type ComponentType = f32;

    fn len(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    fn len_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn normal(&self) -> Self {
        let len = self.len();
        if len.is_normal() {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self {
                x: 0.0f32,
                y: 0.0f32,
            }
        }
    }

    fn normalize(&mut self) {
        let len = self.len();
        if len.is_normal() {
            self.x /= len;
            self.y /= len;
        } else {
            self.x = 0.0f32;
            self.y = 0.0f32;
        }
    }

    fn get_component_0(&self) -> Option<Self::ComponentType> {
        Some(self.x)
    }

    fn get_component_1(&self) -> Option<Self::ComponentType> {
        Some(self.y)
    }

    fn get_component_2(&self) -> Option<Self::ComponentType> {
        None
    }

    fn get_component_3(&self) -> Option<Self::ComponentType> {
        None
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2 {{ {:.3}, {:.3} }}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use num_traits::FloatConst;
    use super::*;

    const COMMON_V_1: Vec2 = Vec2 { x: 1.23f32, y: 4.56f32 };
    const COMMON_V_2: Vec2 = Vec2 { x: 1f32, y: 1f32 };
    const COMMON_V_Z: Vec2 = consts::ZERO;

    #[test]
    fn display() {
        assert_eq!(format!("{}", COMMON_V_1), "Vec2 { 1.23, 4.56 }");
    }

    #[test]
    fn len() {
        assert_eq!(COMMON_V_1.len(), f32::sqrt(22.3065f32));
        assert_eq!(COMMON_V_2.len(), f32::SQRT_2());
        assert_eq!(COMMON_V_Z.len(), 0f32);
    }

    #[test]
    fn len_sqr() {
        assert_eq!(COMMON_V_1.len(), 22.3065f32);
        assert_eq!(COMMON_V_2.len(), 2f32);
        assert_eq!(COMMON_V_Z.len(), 0f32);
    }
}
