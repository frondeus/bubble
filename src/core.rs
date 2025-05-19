use std::{error::Error, marker::PhantomData};

/// Trait to allow "build" variant type from leaf node.
pub trait BuildFrom<From> {
    fn build_from(from: From) -> Result<Self, From>
    where
        Self: Sized;
}

/// A wrapper around an error that allows it to be created from any error that has
/// `T` in its source chain.
#[derive(Debug)]
pub struct Bubble<T: Error + 'static> {
    marker: PhantomData<T>,
    error: Box<dyn Error + 'static>,
}

impl<T: Error + 'static> Bubble<T> {
    pub fn build<U: Error + 'static>(error: U) -> Result<Self, U> {
        let original = error;
        let iter = SourceIter {
            current: Some(&original),
        };
        for source in iter {
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
        let iter = SourceIter {
            current: Some(&*self.error),
        };
        for source in iter {
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
