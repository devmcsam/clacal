use crate::Expr;

pub enum NumLiteral {
    /// A number that can be exactly represented without a decimal.
    Integer(i64),
    /// Any real number.
    Float(f64),
}