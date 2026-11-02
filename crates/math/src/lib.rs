//! A custom math library written for ldbg.
//! 
//! Features:
//! - Vectors (vec2, vec3, vec4)
//! - Quaternions (+ Euler rotation)
//! - Matrices (3x3,4x4)
//! - Utility
//! 
//! More details following soon...

#![allow(unused)]
#![forbid(unreachable_code)]

pub mod vec2;
pub mod traits;

/// 
pub mod prelude {
    pub use super::vec2::Vec2;
}

/*
TODO:
- vec2
- vec2i (i32 variant)
- vec2d (f64 variant)
- [vec2g (generic variant)]?
*/
