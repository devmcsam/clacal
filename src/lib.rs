mod func;
mod number;

use func::*;
use number::*;

pub enum Expr {
    /// An `Operator`
    Operator(Operator),
    /// A number that is known at parse time
    NumLiteral(NumLiteral),
    /// A function that is called
    Function(Function),
}

pub enum Operator {
    /// The additive operator, typically '+'.
    Add,
    /// The subtractive operator, typically '-'.
    Sub,
    /// The multiplicative operator, typically '*'.
    Mul,
    /// The divide operator, typically '/'.
    Div,
    /// Raising one number to another's power, typically '^'.
    Pow,
    /// Negating a number (multiplying by negative 1), typically '-' directly before a number.
    Neg,
}