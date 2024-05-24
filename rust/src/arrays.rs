use crate::addressing::AddressingMode;

/// Enum that represents all the array types
#[derive(Debug, Copy, Clone)]
pub enum Array {
    /// Ranges are selected using an Array2
    Range(Array2<AddressingMode>),
    /// Colors are specified using HSL components in an Array3
    Color(Array3<AddressingMode>),
}

/// Prism Assembly Language representation of an array with two elements (range selection)
#[derive(Debug, Copy, Clone)]
pub struct Array2<T>(pub T, pub T);

impl<T> TryFrom<&Vec<T>> for Array2<T>
where
    T: Clone,
{
    type Error = ();

    // Copy the contents of a vector into an Array
    fn try_from(vec: &Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() == 2 {
            Ok(Self(vec[0].clone(), vec[1].clone()))
        } else {
            Err(())
        }
    }
}

impl<T> Into<Vec<T>> for Array2<T> {
    /// Transform an [Array2] back into a [Vec] or size 2
    fn into(self) -> Vec<T> {
        vec![self.0, self.1]
    }
}

/// Prism Assembly Language representation of an array with HSL components
#[derive(Debug, Copy, Clone)]
pub struct Array3<T>(pub T, pub T, pub T);

impl<T> TryFrom<&Vec<T>> for Array3<T>
where
    T: Clone,
{
    type Error = ();

    // Copy the contents of a vector into an Array
    fn try_from(vec: &Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() == 3 {
            Ok(Self(vec[0].clone(), vec[1].clone(), vec[2].clone()))
        } else {
            Err(())
        }
    }
}

impl<T> Into<Vec<T>> for Array3<T> {
    /// Transform an [Array3] back into a [Vec] or size 3
    fn into(self) -> Vec<T> {
        vec![self.0, self.1, self.2]
    }
}
