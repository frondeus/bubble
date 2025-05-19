// use std::marker::PhantomData;

mod build_from;
mod cast_into;
mod strukt;

// pub trait BuildFrom<From> {
//     fn build_from(t: From) -> Result<Self, From>
//     where
//         Self: Sized;
// }

// struct MarkSelf;
// struct MarkDerive;

// pub trait CastInto<To> {
//     fn cast_into(self) -> Result<To, Self>
//     where
//         Self: Sized;
// }

// // impl<T> CastInto<T> for T {
// //     fn cast_into(self) -> Result<T, T> {
// //         Ok(self)
// //     }
// // }

// pub trait SBuildFrom<From, To> {
//     fn sbuild_from(&self, t: From) -> Result<To, From>;
// }

// pub trait SCastInto<From, To> {
//     fn scast_into(&self, t: From) -> Result<To, From>;
// }

// struct Marker<From, To>(PhantomData<(From, To)>);

// impl<From, To> Default for Marker<From, To> {
//     fn default() -> Self {
//         Self(PhantomData)
//     }
// }

// macro_rules! marker {
//     ($from: ty , $to: ty) => {
//         (&mut &mut &mut &Marker::<$from, $to>::default())
//     };
// }

// impl<From, To> SBuildFrom<From, To> for &Marker<From, To> {
//     fn sbuild_from(&self, t: From) -> Result<To, From> {
//         eprintln!(
//             "Dispatching FROM {} TO {}: Err",
//             std::any::type_name::<From>(),
//             std::any::type_name::<To>()
//         );
//         Err(t)
//     }
// }

// impl<From, To> SBuildFrom<From, To> for &mut &Marker<From, To>
// where
//     From: CastInto<To>,
// {
//     fn sbuild_from(&self, t: From) -> Result<To, From> {
//         eprintln!(
//             "Dispatching FROM {} TO {}: CastInto",
//             std::any::type_name::<From>(),
//             std::any::type_name::<To>()
//         );
//         marker!(From, To).scast_into(t)
//         // Err(t)
//     }
// }

// impl<From> SBuildFrom<From, From> for &mut &mut &Marker<From, From> {
//     fn sbuild_from(&self, t: From) -> Result<From, From> {
//         eprintln!("Dispatching SELF {}", std::any::type_name::<From>(),);
//         Ok(t)
//     }
// }

// impl<From, To> SBuildFrom<From, To> for &mut &mut &mut &Marker<From, To>
// where
//     To: BuildFrom<From>,
// {
//     fn sbuild_from(&self, t: From) -> Result<To, From> {
//         eprintln!(
//             "Dispatching FROM {} TO {}: BuildFrom",
//             std::any::type_name::<From>(),
//             std::any::type_name::<To>()
//         );
//         To::build_from(t)
//     }
// }

// //--

// impl<From, To> SCastInto<From, To> for &Marker<From, To> {
//     fn scast_into(&self, t: From) -> Result<To, From> {
//         eprintln!(
//             "CAST FROM {} TO {}: Err",
//             std::any::type_name::<From>(),
//             std::any::type_name::<To>()
//         );
//         // marker!(From, To).scast_into(t)
//         Err(t)
//     }
// }

// impl<From> SCastInto<From, From> for &mut &Marker<From, From> {
//     fn scast_into(&self, t: From) -> Result<From, From> {
//         eprintln!("CAST SELF {}", std::any::type_name::<From>(),);
//         Ok(t)
//     }
// }

// impl<From, To> SCastInto<From, To> for &mut &mut &Marker<From, To>
// where
//     From: CastInto<To>,
// {
//     fn scast_into(&self, t: From) -> Result<To, From> {
//         eprintln!(
//             "CAST FROM {} TO {}: CastInto",
//             std::any::type_name::<From>(),
//             std::any::type_name::<To>()
//         );
//         t.cast_into()
//     }
// }
// ///
// /// -----------------------------

// #[derive(Debug, PartialEq)]
// enum Top {
//     A(A),
//     B(B),
//     Middle(Middle),
// }

// impl<T> CastInto<T> for Top
// // where Middle: CastInto<T>
// {
//     fn cast_into(self) -> Result<T, Top> {
//         println!("CastInto<T> for Top");

//         match self {
//             Top::A(a) => marker!(A, T).scast_into(a).map_err(Top::A),
//             Top::B(b) => marker!(B, T).scast_into(b).map_err(Top::B),
//             Top::Middle(m) => marker!(Middle, T).scast_into(m).map_err(Top::Middle),
//         }
//     }
// }

// #[test]
// fn cast_into_top() {
//     let top_a = Top::A(A);
//     let a = marker!(Top, A).scast_into(top_a).unwrap();
//     assert_eq!(a, A);
// }
// // impl CastInto<A> for Top {
// //     fn cast_into(self) -> Result<A, Top> {
// //         println!("CastInto<A> for Top");
// //         match self {
// //             Top::A(a) => Ok(a),
// //             t => Err(t),
// //         }
// //     }
// // }
// // impl CastInto<Middle> for Top {
// //     fn cast_into(self) -> Result<Middle, Top> {
// //         println!("CastInto<Middle> for Top");
// //         match self {
// //             Top::Middle(m) => Ok(m),
// //             t => Err(t),
// //         }
// //     }
// // }

// impl BuildFrom<A> for Top {
//     fn build_from(t: A) -> Result<Self, A> {
//         eprintln!("BuildFrom<A> for Top");
//         Err(t)
//             .or_else(|t| marker!(A, A).sbuild_from(t).map(Top::A))
//             .or_else(|t| marker!(A, B).sbuild_from(t).map(Top::B))
//             .or_else(|t| marker!(A, Middle).sbuild_from(t).map(Top::Middle))
//     }
// }
// impl BuildFrom<B> for Top {
//     fn build_from(t: B) -> Result<Self, B> {
//         eprintln!("BuildFrom<B> for Top");
//         Err(t)
//             .or_else(|t| marker!(B, A).sbuild_from(t).map(Top::A))
//             .or_else(|t| marker!(B, B).sbuild_from(t).map(Top::B))
//             .or_else(|t| marker!(B, Middle).sbuild_from(t).map(Top::Middle))
//     }
// }
// impl BuildFrom<Middle> for Top {
//     fn build_from(t: Middle) -> Result<Self, Middle> {
//         eprintln!("BuildFrom<Middle> for Top");
//         Err(t)
//             .or_else(|t| marker!(Middle, A).sbuild_from(t).map(Top::A))
//             .or_else(|t| marker!(Middle, B).sbuild_from(t).map(Top::B))
//             .or_else(|t| marker!(Middle, Middle).sbuild_from(t).map(Top::Middle))
//     }
// }

// impl From<A> for Top {
//     fn from(t: A) -> Self {
//         marker!(A, Top).sbuild_from(t).unwrap()
//     }
// }
// impl From<B> for Top {
//     fn from(t: B) -> Self {
//         marker!(B, Top).sbuild_from(t).unwrap()
//     }
// }
// impl From<Middle> for Top {
//     fn from(t: Middle) -> Self {
//         marker!(Middle, Top).sbuild_from(t).unwrap()
//     }
// }

// impl BuildFrom<Top> for Middle {
//     fn build_from(t: Top) -> Result<Self, Top>
//     where
//         Self: Sized,
//     {
//         eprintln!("BuildFrom<Top> for Middle");
//         match t {
//             Top::A(a) => marker!(A, Middle).sbuild_from(a).map_err(Top::A),
//             Top::B(b) => marker!(B, Middle).sbuild_from(b).map_err(Top::B),
//             Top::Middle(m) => marker!(Middle, Middle).sbuild_from(m).map_err(Top::Middle),
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// enum Middle {
//     Bottom(Bottom),
//     Bottom2(Bottom2),
// }

// impl<T> CastInto<T> for Middle
// where
//     Bottom: CastInto<T>,
// {
//     fn cast_into(self) -> Result<T, Middle> {
//         eprintln!("CastInto<T> for Middle");
//         match self {
//             Middle::Bottom(b) => marker!(Bottom, T).scast_into(b).map_err(Middle::Bottom),
//             Middle::Bottom2(b2) => marker!(Bottom2, T).scast_into(b2).map_err(Middle::Bottom2),
//         }
//     }
// }

// impl BuildFrom<Bottom> for Middle {
//     fn build_from(t: Bottom) -> Result<Self, Bottom> {
//         eprintln!("BuildFrom<Bottom> for Middle");
//         Err(t).or_else(|t| marker!(Bottom, Bottom).sbuild_from(t).map(Middle::Bottom))
//     }
// }
// impl BuildFrom<Bottom2> for Middle {
//     fn build_from(t: Bottom2) -> Result<Self, Bottom2> {
//         eprintln!("BuildFrom<Bottom2> for Middle");
//         Err(t).or_else(|t| {
//             marker!(Bottom2, Bottom2)
//                 .sbuild_from(t)
//                 .map(Middle::Bottom2)
//         })
//     }
// }

// impl From<Bottom> for Middle {
//     fn from(t: Bottom) -> Self {
//         marker!(Bottom, Middle).sbuild_from(t).unwrap()
//     }
// }

// impl From<Bottom2> for Middle {
//     fn from(t: Bottom2) -> Self {
//         marker!(Bottom2, Middle).sbuild_from(t).unwrap()
//     }
// }

// impl BuildFrom<Middle> for Bottom {
//     fn build_from(t: Middle) -> Result<Self, Middle>
//     where
//         Self: Sized,
//     {
//         eprintln!("BuildFrom<Middle> for Bottom");
//         match t {
//             Middle::Bottom(b) => Ok(b),
//             b => Err(b),
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// enum Bottom {
//     A(A),
// }

// impl CastInto<A> for Bottom {
//     fn cast_into(self) -> Result<A, Bottom> {
//         eprintln!("CastInto<A> for Bottom");
//         match self {
//             Bottom::A(a) => Ok(a),
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// enum Bottom2 {
//     B(B),
// }

// impl CastInto<B> for Bottom2 {
//     fn cast_into(self) -> Result<B, Bottom2> {
//         eprintln!("CastInto<B> for Bottom2");
//         match self {
//             Bottom2::B(b) => Ok(b),
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// struct A;

// #[derive(Debug, PartialEq)]
// struct B;

// fn top() -> Result<(), Top> {
//     middle()?;
//     Ok(())
// }

// fn middle() -> Result<(), Middle> {
//     bottom()?;
//     Ok(())
// }

// fn bottom() -> Result<(), Bottom> {
//     return Err(Bottom::A(A));
// }

// fn top2() -> Result<(), Top> {
//     middle2()?;
//     Ok(())
// }

// fn middle2() -> Result<(), Middle> {
//     bottom2()?;
//     Ok(())
// }

// fn bottom2() -> Result<(), Bottom2> {
//     return Err(Bottom2::B(B));
// }

// #[test]
// fn test_middle() {
//     let top = top().unwrap_err();

//     assert_eq!(top, Top::A(A));
// }

// #[test]
// fn test_middle2() {
//     let top = top2().unwrap_err();

//     assert_eq!(top, Top::B(B));
// }
