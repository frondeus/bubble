use std::marker::PhantomData;

/// This trait is used to convert a type `T` to a `Self` if possible.
/// If the conversion is not possible, the original type `T` is returned.
/// Used to chain different conversions together instead of relying on `match` inside `from` implementations.
///
/// Automatically implemented for types that implement `From<T>`.
pub trait Bubble<T, M>: Sized {
    fn bubble(t: T) -> Result<Self, T>;
}

pub struct SelfBubble;
pub struct IntermediateBubble;
pub struct DeriveBubble;

impl<T, U> Bubble<T, SelfBubble> for U
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
    S: Bubble<T, DeriveBubble>,
{
    fn sbubble(&self, t: T) -> Result<S, T> {
        S::bubble(t)
    }
}

impl<T, S> SBubble<T, S> for &mut &mut &Marker<T, S>
where
    S: Bubble<T, (IntermediateBubble, DeriveBubble)>,
{
    fn sbubble(&self, t: T) -> Result<S, T> {
        S::bubble(t)
    }
}

impl<T, S> SBubble<T, S> for &mut &mut &mut &Marker<T, S>
where
    S: Bubble<T, (IntermediateBubble, SelfBubble)>,
{
    fn sbubble(&self, t: T) -> Result<S, T> {
        S::bubble(t)
    }
}

impl<T, S> SBubble<T, S> for &mut &mut &mut &mut &Marker<T, S>
where
    S: Bubble<T, SelfBubble>,
{
    fn sbubble(&self, t: T) -> Result<S, T> {
        S::bubble(t)
    }
}
