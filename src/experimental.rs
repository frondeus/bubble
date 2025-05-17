// use std::marker::PhantomData;

// use thiserror::Error;

// /// This trait is used to convert a type `T` to a `Self` if possible.
// /// If the conversion is not possible, the original type `T` is returned.
// /// Used to chain different conversions together instead of relying on `match` inside `from` implementations.
// ///
// /// Automatically implemented for types that implement `From<T>`.
// pub trait Bubble<T>: Sized {
//     fn bubble(t: T) -> Result<Self, T> {
//         Err(t)
//     }
// }

// pub trait SBubble<T, S> {
//     fn sbubble(&self, t: T) -> Result<S, T> {
//         Err(t)
//     }
// }

// impl<T, S, Marker> SBubble<T, S> for &Marker {}

// impl<T, U> Bubble<T> for U
// where
//     U: From<T>,
// {
//     fn bubble(t: T) -> Result<U, T> {
//         Ok(From::from(t))
//     }
// }

// struct Marker<T, U>(PhantomData<T>, PhantomData<U>);
// impl<T, U> Marker<T, U> {
//     fn new() -> Self {
//         Self(PhantomData, PhantomData)
//     }
// }

// //--------------------------------

// #[derive(PartialEq, Debug, Error)]
// enum Top {
//     #[error("A")]
//     A(#[from] A),
//     #[error("B")]
//     B(#[source] Bottom),

//     #[error("C")]
//     C(#[from] C),
// }
// // variant impls
// impl Bubble<Top> for A {
//     fn bubble(t: Top) -> Result<Self, Top> {
//         match t {
//             Top::A(a) => Ok(a),
//             _ => Err(t),
//         }
//     }
// }
// impl SBubble<Top, A> for &mut &Marker<Top, A> {
//     fn sbubble(&self, t: Top) -> Result<A, Top> {
//         A::bubble(t)
//     }
// }

// impl Bubble<Top> for Bottom {
//     fn bubble(t: Top) -> Result<Self, Top> {
//         match t {
//             Top::B(b) => Ok(b),
//             _ => Err(t),
//         }
//     }
// }
// impl SBubble<Top, Bottom> for &mut &Marker<Top, Bottom> {
//     fn sbubble(&self, t: Top) -> Result<Bottom, Top> {
//         Bottom::bubble(t)
//     }
// }
// impl Bubble<Top> for C {
//     fn bubble(t: Top) -> Result<Self, Top> {
//         match t {
//             Top::C(c) => Ok(c),
//             _ => Err(t),
//         }
//     }
// }
// impl SBubble<Top, C> for &mut &Marker<Top, C> {
//     fn sbubble(&self, t: Top) -> Result<C, Top> {
//         C::bubble(t)
//     }
// }
// // structimpl
// impl SBubble<Top, Top> for &mut &Marker<Top, Top> {
//     fn sbubble(&self, t: Top) -> Result<Top, Top> {
//         Ok(t)
//     }
// }

// impl From<Bottom> for Top {
//     fn from(bot: Bottom) -> Top {
//         Err(bot)
//             .or_else(|bot: Bottom| (&mut &Marker::<Bottom, A>::new()).sbubble(bot).map(Top::A))
//             .or_else(|bot: Bottom| {
//                 (&mut &Marker::<Bottom, Bottom>::new())
//                     .sbubble(bot)
//                     .map(Top::B)
//             })
//             .or_else(|bot: Bottom| (&mut &Marker::<Bottom, C>::new()).sbubble(bot).map(Top::C))
//             .expect("Bottom should be A or B or C")
//     }
// }

// // impl From<Bottom> for Top {
// //     fn from(bot: Bottom) -> Top {
// //         Err(bot)
// //             .or_else(|bot: Bottom| A::bubble(bot).map(Top::A))
// //             .or_else(|bot: Bottom| Bottom::bubble(bot).map(Top::B))
// //             .or_else(|bot: Bottom| C::bubble(bot).map(Top::C))
// //             .expect("Bottom should be A or B or C")
// //     }
// // }
// // variant impls
// impl Bubble<Bottom> for A {
//     fn bubble(t: Bottom) -> Result<Self, Bottom> {
//         match t {
//             Bottom::A(a) => Ok(a),
//             _ => Err(t),
//         }
//     }
// }
// impl SBubble<Bottom, A> for &mut &Marker<Bottom, A> {
//     fn sbubble(&self, t: Bottom) -> Result<A, Bottom> {
//         A::bubble(t)
//     }
// }

// impl Bubble<Bottom> for B {
//     fn bubble(t: Bottom) -> Result<Self, Bottom> {
//         match t {
//             Bottom::B(b) => Ok(b),
//             _ => Err(t),
//         }
//     }
// }
// impl SBubble<Bottom, B> for &mut &Marker<Bottom, B> {
//     fn sbubble(&self, t: Bottom) -> Result<B, Bottom> {
//         B::bubble(t)
//     }
// }
// // structimpl
// impl SBubble<Bottom, Bottom> for &mut &Marker<Bottom, Bottom> {
//     fn sbubble(&self, t: Bottom) -> Result<Bottom, Bottom> {
//         Ok(t)
//     }
// }

// #[derive(PartialEq, Debug, Error)]
// enum Bottom {
//     #[error("A")]
//     A(#[from] A),
//     #[error("B")]
//     B(#[from] B),
// }

// #[derive(PartialEq, Debug, Error)]
// #[error("A")]
// struct A;
// #[derive(PartialEq, Debug, Error)]
// #[error("B")]
// struct B;

// // impl Bubble<Bottom, DoesntImplementMarker> for C {
// //     fn bubble(t: Bottom) -> Result<Self, Bottom> {
// //         Err(t)
// //     }
// // }

// #[derive(PartialEq, Debug, Error)]
// #[error("C")]
// struct C;

// fn top_a() -> Result<(), Top> {
//     bottom_a()?;
//     Ok(())
// }

// fn bottom_a() -> Result<(), Bottom> {
//     Err(A.into())
// }

// fn bottom_b() -> Result<(), Bottom> {
//     Err(B.into())
// }

// fn top_b() -> Result<(), Top> {
//     bottom_b()?;
//     Ok(())
// }

// fn top_c() -> Result<(), Top> {
//     Err(C.into())
// }

// #[test]
// fn ex_a() {
//     let res = top_a().unwrap_err();
//     assert_eq!(res, Top::A(A));
// }

// #[test]
// fn ex_b() {
//     let res = top_b().unwrap_err();
//     assert_eq!(res, Top::B(Bottom::B(B)));
// }

// #[test]
// fn ex_c() {
//     let res = top_c().unwrap_err();
//     assert_eq!(res, Top::C(C));
// }
