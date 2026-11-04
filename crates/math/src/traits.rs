/// A simple trait implementing common functions for all vectors.
/// (len, len_sqr, normal, normalize).
/// Also includes functions making working with generics easier.
pub trait Vector {
    type ComponentType;

    /// Returns the length of this vector.
    fn len(&self) -> f32;
    /// Returns the length of this vector without sqrt.
    fn len_sqr(&self) -> f32;

    /// Returns the normal representation of this vector (unit vector).
    fn normal(&self) -> Self;
    /// Modifies this vector in place to the normal representation (unit vector).
    fn normalize(&mut self);

    fn get_component_0(&self) -> Option<Self::ComponentType>;
    fn get_component_1(&self) -> Option<Self::ComponentType>;
    fn get_component_2(&self) -> Option<Self::ComponentType>;
    fn get_component_3(&self) -> Option<Self::ComponentType>;
}
