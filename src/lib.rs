#![allow(dead_code)]

#[derive(PartialEq, Debug)]
enum Top {
    A(A),
    B(Bottom),
}

trait MaybeFrom<T>: Sized {
    fn maybe_from(t: T) -> Result<Self, T>;
}

impl<T, U> MaybeFrom<T> for U
where
    U: From<T>,
{
    fn maybe_from(t: T) -> Result<Self, T> {
        Ok(From::from(t))
    }
}

impl From<Bottom> for Top {
    fn from(bot: Bottom) -> Top {
        A::maybe_from(bot)
            .map(Top::A)
            .or_else(|bot| Bottom::maybe_from(bot).map(Top::B))
            .expect("Bottom should be A or B")
    }
}
impl MaybeFrom<Bottom> for A {
    fn maybe_from(t: Bottom) -> Result<Self, Bottom> {
        match t {
            Bottom::A(a) => Ok(a),
            _ => Err(t),
        }
    }
}

impl From<A> for Bottom {
    fn from(a: A) -> Bottom {
        Bottom::A(a)
    }
}

#[derive(PartialEq, Debug)]
enum Bottom {
    A(A),
    B(B),
}

#[derive(PartialEq, Debug)]
struct A;
#[derive(PartialEq, Debug)]
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
