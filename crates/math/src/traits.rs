pub trait GetLength {
    type Output;
    fn len(&self) -> Self::Output;
}

pub trait GetLengthSquared {
    type Output;
    fn len_sqr(&self) -> Self::Output;
}
