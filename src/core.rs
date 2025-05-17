/// This trait is used to convert a type `T` to a `Self` if possible.
/// If the conversion is not possible, the original type `T` is returned.
/// Used to chain different conversions together instead of relying on `match` inside `from` implementations.
///
/// Automatically implemented for types that implement `From<T>`.
pub trait Bubble<T>: Sized {
    fn bubble(t: T) -> Result<Self, T>;
}

pub trait SpecializedBubble<T, S> {
    fn sbubble(&self, t: T) -> Result<S, T>;
}

// pub trait HasBubble{}

impl<T, U> Bubble<T> for U
where
    U: From<T>,
{
    fn bubble(t: T) -> Result<U, T> {
        Ok(From::from(t))
    }
}

// pub struct BubbleMark;

impl<T, Mark> SpecializedBubble<T, T> for &Mark {
    fn sbubble(&self, t: T) -> Result<T, T> {
        Ok(t)
    }
}

impl<T, S, Mark> SpecializedBubble<T, S> for &mut &Mark {
    fn sbubble(&self, t: T) -> Result<S, T> {
        Err(t)
    }
}

// impl<T, S> SpecializedBubble<T, S> for &mut &BubbleMark 
// where S: Sized + Bubble<T> {
//     fn sbubble(&self, t: T) -> Result<S, T> {
//         S::bubble(t)
//     }
// }
