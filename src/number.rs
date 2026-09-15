use crate::Expr;


/// Any real number that can be represented by either an `i64` or an `f64`. We use an `i64` for cases
/// where the input to something is an integer, this can make computation much faster and more accurate.
pub enum Number {
    /// A number that can be exactly represented without a decimal.
    Integer(i64),
    /// Any real number.
    Float(f64),
}