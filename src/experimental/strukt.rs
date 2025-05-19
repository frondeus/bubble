use std::error::Error;

use crate::core::{Bubble, SourceIter};
use thiserror::Error;

fn print_error(error: &(dyn Error + 'static)) {
    let mut sources = SourceIter {
        current: Some(error),
    };
    let Some(source) = sources.next() else {
        return;
    };
    eprintln!("ERROR: {}", source);
    while let Some(source) = sources.next() {
        println!("  CAUSED BY: {}", source);
    }
}

#[derive(Debug, Error)]
pub enum Top {
    #[error("Top::A")]
    A(Bubble<A>),
    #[error("Top::Middle")]
    Middle(#[source] Middle),
}

impl From<Middle> for Top {
    fn from(value: Middle) -> Self {
        Err(value)
            .or_else(|value| Bubble::<A>::build(value).map(Top::A))
            .or_else(|value| Ok::<Top, Middle>(Top::Middle(value)))
            .unwrap()
    }
}

#[test]
fn test_top() {
    let a = Bubble::<A>::build(A).unwrap();
    let a = a.downcast_ref();
    print_error(a);

    let a = Bubble::<A>::build(Bottom::A(A)).unwrap();
    let a = a.downcast_ref();
    print_error(a);

    let a = Bubble::<A>::build(Top::Middle(Middle::Bottom(Bottom::A(A)))).unwrap();
    let a = a.downcast_ref();
    print_error(a);

    let a = Bubble::<A>::build(Top::Middle(Middle::Bottom(Bottom::A(A)))).unwrap();
    print_error(a.full_error());
}

#[derive(Debug, PartialEq, Error)]
pub enum Middle {
    #[error("Middle::Bottom")]
    Bottom(#[from] Bottom),
}

#[derive(Debug, PartialEq, Error)]
pub enum Bottom {
    #[error("Bottom::A")]
    A(#[from] A),
    #[error("Bottom::B")]
    B(B),
}

#[derive(Debug, PartialEq, Error)]
#[error("A")]
pub struct A;

#[derive(Debug, PartialEq, Error)]
#[error("B")]
pub struct B;
