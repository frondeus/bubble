use std::marker::PhantomData;

use thiserror::Error;

/// This trait is used to convert a type `T` to a `Self` if possible.
/// If the conversion is not possible, the original type `T` is returned.
/// Used to chain different conversions together instead of relying on `match` inside `from` implementations.
///
/// Automatically implemented for types that implement `From<T>`.
pub trait Bubble<T>: Sized {
    fn bubble(t: T) -> Result<Self, T> {
        Err(t)
    }
}

pub trait SBubble<T, S> {
    fn sbubble(&self, t: T) -> Result<S, T> {
        Err(t)
    }
}

impl<T, S, Marker> SBubble<T, S> for &Marker { }


impl<T, U> Bubble<T> for U
where
    U: From<T>,
{
    fn bubble(t: T) -> Result<U, T> {
        Ok(From::from(t))
    }
}

//--------------------------------

#[derive(PartialEq, Debug, Error)]
enum Top {
    #[error("A")]
    A(#[from] A),
    #[error("B")]
    B(#[source] Bottom),

    #[error("C")]
    C(#[from] C),
}

struct Marker<T>(PhantomData<T>);
impl<T> Marker<T> {
    fn new() -> Self { Self(PhantomData) }
}

impl From<Bottom> for Top {
    fn from(bot: Bottom) -> Top {
        Err(bot)
            .or_else(|bot: Bottom| (&mut &Marker::<A>::new()).sbubble(bot)   .map(Top::A))
            .or_else(|bot: Bottom| (&mut &Marker::<B>::new()).sbubble(bot)   .map(Top::B))
            .or_else(|bot: Bottom| (&mut &Marker::<C>::new()).sbubble(bot)   .map(Top::C))
            .expect("Bottom should be A or B or C")
    }
}

impl SBubble<Bottom, A> for &mut &Marker<A> {
    fn sbubble(&self, t: Bottom) -> Result<A, Bottom> {
        A::bubble(t)
    }
}

impl SBubble<Bottom, B> for &mut &Marker<B> {
    fn sbubble(&self, t: Bottom) -> Result<B, Bottom> {
        B::bubble(t)
    }
}
impl SBubble<Bottom, Bottom> for &mut &Marker<B> {
    fn sbubble(&self, t: Bottom) -> Result<Bottom, Bottom> {
        Ok(t)
    }
}

// impl From<Bottom> for Top {
//     fn from(bot: Bottom) -> Top {
//         Err(bot)
//             .or_else(|bot: Bottom| A::bubble(bot).map(Top::A))
//             .or_else(|bot: Bottom| Bottom::bubble(bot).map(Top::B))
//             .or_else(|bot: Bottom| C::bubble(bot).map(Top::C))
//             .expect("Bottom should be A or B or C")
//     }
// }
impl Bubble<Bottom> for A {
    fn bubble(t: Bottom) -> Result<Self, Bottom> {
        match t {
            Bottom::A(a) => Ok(a),
            _ => Err(t),
        }
    }
}
impl Bubble<Bottom> for B {
    fn bubble(t: Bottom) -> Result<Self, Bottom> {
        match t {
            Bottom::B(b) => Ok(b),
            _ => Err(t),
        }
    }
}

#[derive(PartialEq, Debug, Error)]
enum Bottom {
    #[error("A")]
    A(#[from] A),
    #[error("B")]
    B(#[from] B),
}


#[derive(PartialEq, Debug, Error)]
#[error("A")]
struct A;
#[derive(PartialEq, Debug, Error)]
#[error("B")]
struct B;

// impl Bubble<Bottom, DoesntImplementMarker> for C {
//     fn bubble(t: Bottom) -> Result<Self, Bottom> {
//         Err(t)
//     }
// }

#[derive(PartialEq, Debug, Error)]
#[error("C")]
struct C;

fn top_a() -> Result<(), Top> {
    bottom_a()?;
    Ok(())
}

fn bottom_a() -> Result<(), Bottom> {
    Err(A.into())
}

fn bottom_b() -> Result<(), Bottom> {
    Err(B.into())
}

fn top_b() -> Result<(), Top> {
    bottom_b()?;
    Ok(())
}

fn top_c() -> Result<(), Top> {
    Err(C.into())
}

#[test]
fn test_a() {
    let res = top_a().unwrap_err();
    assert_eq!(res, Top::A(A));
}

#[test]
fn test_b() {
    let res = top_b().unwrap_err();
    assert_eq!(res, Top::B(Bottom::B(B)));
}

#[test]
fn test_c() {
    let res = top_c().unwrap_err();
    assert_eq!(res, Top::C(C));
}
