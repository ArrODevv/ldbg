//! The module containing the vec2 implementation.

use serde::{Deserialize, Serialize};

use crate::traits::{GetLength, GetLengthSquared};

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

impl GetLength for Vec2 {
    type Output = f32;
    fn len(&self) -> Self::Output {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl GetLengthSquared for Vec2 {
    type Output = f32;
    fn len_sqr(&self) -> Self::Output {
        self.x * self.x + self.y * self.y
    }
}
