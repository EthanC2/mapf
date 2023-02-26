//use pest::Parser;
//use pest_derive::Parser;
use std::ops::*;

/*
    Any multi-operator expression can be composed by binary and unary functions.
    For example, `2 * -x + 3` can be broken down into `-x`, `2 * expr` and `expr + 3`
    where `expr` denotes the previous expression.

    This means that we only need three functions to be able to dynamically create
    any function:
    1. A function to apply a given binary operator to `x` and some other number
    2. A function to apply a given unary operator to `x`
    3. A function to compose two functions
    
    Note to self: the `apply` functions can just be inlined since they're just closures ://
*/

//Takes a number `num` and a binary operator `op` and returns `num op x` (e.g. `2 * x`)
#[inline(always)]
fn apply_binary<A,B,C>(num: A, op: impl Fn(A,B) -> C) -> impl Fn(B) -> C 
where A: Copy {
    move |x| op(num,x)
}

//Takes a unary operator `op` and returns `op x` (e.g. `~x`)
#[inline(always)]
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

//#[derive(Parser)]
//#[grammar = "math.pest"]
//struct FunctionParser;
