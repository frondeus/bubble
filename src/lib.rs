#![allow(dead_code)]

#[derive(PartialEq, Debug)]
enum Top {
    A(A),
    B(Bottom),
}

impl From<Bottom> for Top {
    fn from(bot: Bottom) -> Top {
        match bot {
            Bottom::A(a) => Top::A(a),
            bot => Top::B(bot),
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
