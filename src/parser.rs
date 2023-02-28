use pest::{
    iterators::Pairs,
    pratt_parser::{PrattParser, Op, Assoc}

};
use pest_derive::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[grammar = "math.pest"]
pub struct FunctionParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
    };
}

pub fn eval(expr: Pairs<Rule>) -> i64 {
    PRATT_PARSER.
        map_primary(|primary| match primary.as_rule() {
            Rule::int => primary.as_str().parse::<i64>().unwrap(),
            Rule::expr => eval(primary.into_inner()),
            _ => unreachable!("eval() found unexpected expr: {:?}", primary)
        })
        .map_infix(|lhs, op, rhs| { match op.as_rule() {
                Rule::add => println!("{lhs} + {rhs}"),
                Rule::sub => println!("{lhs} - {rhs}"),
                Rule::mul => println!("{lhs} * {rhs}"),
                Rule::div => println!("{lhs} / {rhs}"),
                _ => unreachable!("eval() found unexpected infix operator: {:?}", op)
            };

            0

        })
        .parse(expr)
}
