use std::ops::{AddAssign, Deref, Mul};

use crate::{Error, Result};

#[derive(Debug)]
pub struct Vector<T> {
    data: Vec<T>,
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Vector { data: data.into() }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// # Errors doctest
#[expect(clippy::needless_pass_by_value)]
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(Error::DotProductLengthMismatch {
            a: a.len(),
            b: b.len(),
        });
    }
    let mut sum = T::default();
    for (&x, &y) in a.iter().zip(b.iter()) {
        sum += x * y;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product_success() -> Result<()> {
        let a = Vector::new(Vec::from([1, 2, 3]));
        let b = Vector::new(Vec::from([4, 5, 6]));
        assert_eq!(dot_product(a, b)?, 32);
        Ok(())
    }

    #[test]
    fn test_dot_product_len_error() {
        let a = Vector::new(Vec::from([1, 2, 3]));
        let b = Vector::new(Vec::from([4, 5]));
        match dot_product(a, b) {
            Err(Error::DotProductLengthMismatch { a, b }) => {
                assert_eq!(a, 3);
                assert_eq!(b, 2); // ← 能拿到具体数字
            }
            other => panic!("期望长度不匹配，实际是 {other:?}"),
        }
    }
}
