use super::*;
use super::derive::Bubble;

use thiserror::Error;

#[derive(PartialEq, Debug, Error)]
enum Top {
    #[error("A")]
    A(#[source] A),
    #[error("B")]
    B(#[source] Bottom),
}

impl From<Bottom> for Top {
    fn from(bot: Bottom) -> Top {
        A::bubble(bot)
            .map(Top::A)
            .or_else(|bot| Bottom::bubble(bot).map(Top::B))
            .expect("Bottom should be A or B")
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

#[derive(PartialEq, Debug, Error, Bubble)]
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

fn top() -> Result<(), Top> {
    bottom()?;
    Ok(())
}

fn bottom() -> Result<(), Bottom> {
    Err(A.into())
}

#[test]
fn test() {
    let res = top().unwrap_err();
    assert_eq!(res, Top::A(A));
}
