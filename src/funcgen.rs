//use pest::Parser;
//use pest_derive::Parser;
use std::ops::*;

//Takes a number `num` and a binary operator `op` and returns `num op x` (e.g. `2 * x`)
fn apply_binary<A,B,C>(num: A, op: impl Fn(A,B) -> C) -> impl Fn(B) -> C 
where A: Copy {
    move |x| op(num,x)
}

//Takes a unary operator `op` and returns `op x` (e.g. `~x`)
fn apply_unary<A,B>(op: impl Fn(A) -> B) -> impl Fn(A) -> B {
    move |x| op(x)
}

//Credit to https://stackoverflow.com/users/5903309/jan-hohenheim on StackOverflow
fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}


/*
    2*x + 3 - 20
    2 * x
    + (expr) 3
    - (expr) 20
*/

//#[derive(Parser)]
//#[grammar = "math.pest"]
//struct FunctionParser;