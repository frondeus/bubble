use std::{
    any::{Any, TypeId},
    error::Error,
    marker::PhantomData,
};

/// Trait to allow "extract" variant type from an enum.
///
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

/// An error that is not composed of other errors.
pub trait AtomicError {}

impl<T> CastInto for T
where
    T: AtomicError + 'static,
{
    fn has_ty(&self, ty: std::any::TypeId) -> bool {
        ty == std::any::TypeId::of::<T>()
    }
    fn cast_into(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }
}

/// Trait to allow "build" variant type from leaf node.
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

#[macro_export]
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
        // eprintln!("Invoke ERR: {} -> {}", std::any::type_name::<From>(), std::any::type_name::<To>());
        Err(from)
    }
}

impl<From, To> SBuildFrom<From, To> for &mut &Marker<From, To>
where
    From: CastInto + 'static,
    To: 'static,
{
    fn sbuild_from(&self, from: From) -> Result<To, From> {
        // eprintln!("Invoke CAST: {} -> {}", std::any::type_name::<From>(), std::any::type_name::<To>());

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
        // eprintln!("Invoke BUILD: {} -> {}", std::any::type_name::<From>(), std::any::type_name::<To>());
        To::build_from(from)
    }
}

#[derive(Debug)]
pub struct Bubble<T: Error + 'static> {
    marker: PhantomData<T>,
    error: Box<dyn Error + 'static>,
}

impl<T: Error + 'static> Bubble<T> {
    pub fn build<U: Error + 'static>(error: U) -> Result<Self, U> {
        let original = error;
        let mut iter = SourceIter {
            current: Some(&original),
        };
        while let Some(source) = iter.next() {
            if source.is::<T>() {
                return Ok(Bubble {
                    marker: PhantomData,
                    error: Box::new(original),
                });
            }
        }
        Err(original)
    }

    pub fn full_error(&self) -> &(dyn Error + 'static) {
        &*self.error
    }

    pub fn downcast_ref(&self) -> &T {
        let mut iter = SourceIter {
            current: Some(&*self.error),
        };
        while let Some(source) = iter.next() {
            if source.is::<T>() {
                return source.downcast_ref::<T>().unwrap();
            }
        }
        panic!("No source found");
    }
}

pub(crate) struct SourceIter<'a> {
    pub(crate) current: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Iterator for SourceIter<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(Error::source);
        current
    }
}
