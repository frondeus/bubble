use thiserror::Error;

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

impl From<Bottom> for Top {
    fn from(bot: Bottom) -> Top {
        Err(bot)
            .or_else(|bot: Bottom| A::bubble(bot).map(Top::A))
            .or_else(|bot: Bottom| Bottom::bubble(bot).map(Top::B))
            .or_else(|bot: Bottom| C::bubble(bot).map(Top::C))
            .expect("Bottom should be A or B or C")
    }
}

impl Bubble<Bottom> for A {
    fn bubble(t: Bottom) -> Result<Self, Bottom> {
        match t {
            Bottom::A(a) => Ok(a),
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

impl Bubble<Bottom> for C {
    fn bubble(t: Bottom) -> Result<Self, Bottom> {
        Err(t)
    }
}

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
