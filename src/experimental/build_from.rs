use std::marker::PhantomData;

use super::cast_into::CastInto;

pub trait BuildFrom<From> {
    fn build_from(from: From) -> Result<Self, From>
    where
        Self: Sized;
}

impl<T> BuildFrom<T> for T {
    fn build_from(from: T) -> Result<Self, T> {
        Ok(from)
    }
}

pub struct Marker<From, To>(PhantomData<(From, To)>);

impl<From, To> Default for Marker<From, To> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

macro_rules! marker {
    ($from: ty, $to: ty) => {
        (&mut &mut &Marker::<$from, $to>::default())
    };
}

pub trait SBuildFrom<From, To> {
    fn sbuild_from(&self, from: From) -> Result<To, From>;
}

impl<From, To> SBuildFrom<From, To> for &Marker<From, To> {
    fn sbuild_from(&self, from: From) -> Result<To, From> {
        Err(from)
    }
}

impl<From, To> SBuildFrom<From, To> for &mut &Marker<From, To>
where
    From: CastInto + 'static,
    To: 'static,
{
    fn sbuild_from(&self, from: From) -> Result<To, From> {
        if from.has::<To>() {
            Ok(from.cast::<To>())
        } else {
            Err(from)
        }
    }
}

impl<From, To> SBuildFrom<From, To> for &mut &mut &Marker<From, To>
where
    To: BuildFrom<From>,
{
    fn sbuild_from(&self, from: From) -> Result<To, From> {
        To::build_from(from)
    }
}

//----
use super::cast_into::*;

impl BuildFrom<A> for Top {
    fn build_from(from: A) -> Result<Self, A> {
        Ok(Top::A(from))
    }
}
impl BuildFrom<Middle> for Top {
    fn build_from(from: Middle) -> Result<Self, Middle> {
        Err(from)
            .or_else(|from| marker!(Middle, A).sbuild_from(from).map(Top::A))
            .or_else(|from| marker!(Middle, Middle).sbuild_from(from).map(Top::Middle))
    }
}

#[test]
fn test_build_from() {
    let top = Top::build_from(A).unwrap();
    assert_eq!(top, Top::A(A));

    let top = Top::build_from(Middle::Bottom(Bottom::A(A))).unwrap();
    assert_eq!(top, Top::A(A));

    let top = Top::build_from(Middle::Bottom(Bottom::B(B))).unwrap();
    assert_eq!(top, Top::Middle(Middle::Bottom(Bottom::B(B))));
}

impl BuildFrom<Bottom> for Middle {
    fn build_from(from: Bottom) -> Result<Self, Bottom> {
        Err(from).or_else(|from| {
            marker!(Bottom, Bottom)
                .sbuild_from(from)
                .map(Middle::Bottom)
        })
    }
}

impl BuildFrom<A> for Bottom {
    fn build_from(from: A) -> Result<Self, A> {
        Ok(Bottom::A(from))
    }
}
