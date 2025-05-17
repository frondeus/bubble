use std::marker::PhantomData;

/// This trait is used to convert a type `T` to a `Self` if possible.
/// If the conversion is not possible, the original type `T` is returned.
/// Used to chain different conversions together instead of relying on `match` inside `from` implementations.
///
/// Automatically implemented for types that implement `From<T>`.
pub trait Bubble<T>: Sized {
    fn bubble(t: T) -> Result<Self, T>;
}

impl<T, U> Bubble<T> for U
where
    U: From<T>,
{
    fn bubble(t: T) -> Result<U, T> {
        Ok(From::from(t))
    }
}

/// Specialized bubble used for autoref trick
///
pub trait SBubble<T, S> {
    fn sbubble(&self, t: T) -> Result<S, T> {
        Err(t)
    }
}

/// Structure that implements [`SBubble`].
///
/// Used for autoref specialization trick.
pub struct Marker<T, U>(PhantomData<T>, PhantomData<U>);

impl<T, U> Default for Marker<T, U> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<T, S> SBubble<T, S> for &Marker<T, S> {}

impl<T, S> SBubble<T, S> for &mut &Marker<T, S>
where
    S: Bubble<T>,
{
    fn sbubble(&self, t: T) -> Result<S, T> {
        S::bubble(t)
    }
}
