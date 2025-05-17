use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

pub trait CastInto {
    fn has_ty(&self, ty: TypeId) -> bool;

    fn has<T: 'static>(&self) -> bool {
        self.has_ty(TypeId::of::<T>())
    }

    fn cast_into(self) -> Box<dyn Any>;

    fn cast<T: 'static>(self) -> T
    where
        Self: Sized,
    {
        *self.cast_into().downcast::<T>().unwrap()
    }
}

// impl<T> CastInto<T, CastSelf> for T {
//     fn cast_into(self) -> Result<T, T> {
//         Ok(self)
//     }
// }

#[derive(Debug, PartialEq)]
pub enum Top {
    A(A),
    Middle(Middle),
}

impl CastInto for Top {
    fn has_ty(&self, ty: TypeId) -> bool {
        match self {
            Top::A(a) => a.has_ty(ty),
            Top::Middle(middle) => middle.has_ty(ty),
        }
    }
    fn cast_into(self) -> Box<dyn Any> {
        match self {
            Top::A(a) => a.cast_into(),
            Top::Middle(middle) => middle.cast_into(),
        }
    }
}

// impl<T, AM, MM> CastInto<T, (CastOther, AM, MM)> for Top
// where A: CastInto<T, AM> ,
//     Middle: CastInto<T, MM>
// {
//     fn cast_into(self) -> Result<T, Top> {
//         match self {
//             Top::A(a) => a.cast_into().map_err(Top::A),
//             Top::Middle(middle) => middle.cast_into().map_err(Top::Middle)
//             // Top::A(a) => marker!((A, T)).scast_into(a).map_err(Top::A),
//             // Top::Middle(middle) => marker!((Middle, T)).scast_into(middle).map_err(Top::Middle),
//         }
//     }
// }

#[test]
fn test_top() {
    let top = Top::A(A);
    let a: A = top.cast::<A>();

    let top = Top::Middle(Middle::Bottom(Bottom::A(A)));
    let a: A = top.cast::<A>();
}

#[derive(Debug, PartialEq)]
pub enum Middle {
    Bottom(Bottom),
}

impl CastInto for Middle {
    fn has_ty(&self, ty: TypeId) -> bool {
        match self {
            Middle::Bottom(bottom) => bottom.has_ty(ty),
        }
    }
    fn cast_into(self) -> Box<dyn Any> {
        match self {
            Middle::Bottom(bottom) => bottom.cast_into(),
        }
    }
}

// impl<T, BM> CastInto<T, (CastOther, BM)> for Middle
// where
//     Bottom: CastInto<T, BM>
// {
//     fn cast_into(self) -> Result<T, Self> {
//         match self {
//             Middle::Bottom(bottom) =>
//             bottom.cast_into().map_err(Middle::Bottom),
//         }
//     }
// }

// Commented out version compiles and runs but its not what we want. Its not generic

// impl CastInto<Bottom, MarkDerive> for Middle {
//     fn cast_into(self) -> Result<Bottom, Self> {
//         match self {
//             Middle::Bottom(bottom) => marker!((Bottom, Bottom)).scast_into(bottom).map_err(Middle::Bottom),
//         }
//     }
// }

// // This should not exist
// impl CastInto<A, MarkDerive> for Middle {
//     fn cast_into(self) -> Result<A, Self> {
//         match self {
//             Middle::Bottom(bottom) => marker!((Bottom, A)).scast_into(bottom).map_err(Middle::Bottom),
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub enum Bottom {
    A(A),
    B(B),
}

impl CastInto for Bottom {
    fn has_ty(&self, ty: TypeId) -> bool {
        match self {
            Bottom::A(a) => a.has_ty(ty),
            Bottom::B(b) => b.has_ty(ty),
        }
    }
    fn cast_into(self) -> Box<dyn Any> {
        match self {
            Bottom::A(a) => a.cast_into(),
            Bottom::B(b) => b.cast_into(),
        }
    }
}
// impl<T, AM, BM> CastInto<T, (CastOther, AM, BM)> for Bottom
// where A: CastInto<T, AM>,
// B: CastInto<T, BM>
// {
//     fn cast_into(self) -> Result<T, Self> {
//         match self {
//             Bottom::A(a) => a.cast_into().map_err(Bottom::A),
//             Bottom::B(b) => b.cast_into().map_err(Bottom::B),
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub struct A;

impl CastInto for A {
    fn has_ty(&self, ty: TypeId) -> bool {
        ty == TypeId::of::<A>()
    }
    fn cast_into(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

// impl<T> CastInto<T, CastErr> for A {
//     fn cast_into(self) -> Result<T, Self> {
//         Err(self)
//     }
// }

#[derive(Debug, PartialEq)]
pub struct B;

impl CastInto for B {
    fn has_ty(&self, ty: TypeId) -> bool {
        ty == TypeId::of::<B>()
    }
    fn cast_into(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

// impl<T> CastInto<T, CastErr> for B {
//     fn cast_into(self) -> Result<T, Self> {
//         Err(self)
//     }
// }
