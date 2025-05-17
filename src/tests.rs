use super::*;
use super::derive::Bubble;
use super::core::{SpecializedBubble};

use thiserror::Error;

#[derive(PartialEq, Debug, Error, Bubble)]
enum Top {
    #[error("A")]
    A(#[source] A),
    #[error("B")]
    B(#[source] Bottom),

    #[error("C")]
    C(#[source] C),
}

// impl From<A> for Top {
//     fn from(bot: A) -> Top {
//         A::bubble(bot)
//         .map(Top::A)
//         .or_else(|bot| (&mut &BottomMark).sbubble(bot).map(Top::B))
//         // .or_else(|bot| (&mut &CMark)bubble(bot).map(Top::C))
//         .expect("Bottom should be A or B")
//         // Top::A(a)
//     }
// }

impl From<Bottom> for Top {
    fn from(bot: Bottom) -> Top {
        A::bubble(bot)
            .map(Top::A)
            .or_else(|bot: Bottom| (&mut &mut &BottomMark).sbubble(bot).map(Top::B))
            .or_else(
                |bot: Bottom| {
                    (&mut &mut &CMark).sbubble(bot).map(Top::C)
                }
            )
            // .or_else(|bot| C::bubble(bot).map(Top::C))
            .expect("Bottom should be A or B")
    }
}

struct BottomMark;
// impl SpecializedBubble<Bottom, Bottom> for &mut &BottomMark {
//     fn sbubble(&self, t: Bottom) -> Result<Bottom, Bottom> {
//         Bottom::bubble(t)
//     }
// }

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

struct CMark;

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
