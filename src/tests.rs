use crate::bubble; // Simulate that it is a separate crate
use bubble::Bubble;

use thiserror::Error;

#[derive(PartialEq, Debug, Error, Bubble)]
enum Top {
    #[error("A")]
    A(#[from] A),

    #[error("B")]
    B(#[bubble(from)] Bottom),

    #[error("C")]
    C(#[from] C),

    #[error("Intermediate")]
    Intermediate(#[bubble(from)] Intermediate),
}

#[derive(PartialEq, Debug, Error, Bubble)]
enum Intermediate {
    #[error("Bottom")]
    Bottom(#[bubble(from)] Bottom),
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

#[derive(PartialEq, Debug, Error)]
#[error("C")]
struct C;

fn top_inner_a() -> Result<(), Top> {
    bottom_a()?;
    Ok(())
}

fn top_outer_a() -> Result<(), Top> {
    Err(A.into())
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

fn top_c() -> Result<(), Top> {
    Err(C.into())
}

fn top_intermediate() -> Result<(), Top> {
    intermediate()?;
    Ok(())
}

fn intermediate() -> Result<(), Intermediate> {
    bottom_a()?;
    Ok(())
}

#[test]
fn test_inner_a() {
    let res = top_inner_a().unwrap_err();
    assert_eq!(res, Top::A(A));
}

#[test]
fn test_b() {
    let res = top_b().unwrap_err();
    assert_eq!(res, Top::B(Bottom::B(B)));
}

#[test]

fn test_c() {
    let res = top_c().unwrap_err();
    assert_eq!(res, Top::C(C));
}

#[test]
fn test_outer_a() {
    let res = top_outer_a().unwrap_err();
    assert_eq!(res, Top::A(A));
}

#[test]
fn test_intermediate() {
    let res = top_intermediate().unwrap_err();
    assert_eq!(res, Top::A(A));
}
