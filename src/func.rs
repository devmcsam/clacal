use crate::Expr;
pub enum Function {
    /// A function natively supported by this crate.
    BuiltInFunction(BuiltInFunction),
    /// A function that has been defined either buy users of this crate or created at runtime.
    UserFunction,
}

/// Functions that are built into the calculator framework. These use a fast path via enum resolution
/// rather than key lookup for user created functions.
pub enum BuiltInFunction {
    Abs(Box<Expr>),
    Signum(Box<Expr>),
    Reciprocal(Box<Expr>),
    Floor(Box<Expr>),
    Ceil(Box<Expr>),
    Round(Box<Expr>),
    Trunc(Box<Expr>),
    Fract(Box<Expr>),
    Sqrt(Box<Expr>),
    Cbrt(Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>),
    Exp2(Box<Expr>),
    ExpM1(Box<Expr>),
    Ln(Box<Expr>),
    Ln1P(Box<Expr>),
    Log(Box<Expr>, Box<Expr>),
    Log2(Box<Expr>),
    Log10(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Asin(Box<Expr>),
    Acos(Box<Expr>),
    Atan(Box<Expr>),
    Atan2(Box<Expr>, Box<Expr>),
    Sinh(Box<Expr>),
    Cosh(Box<Expr>),
    Tanh(Box<Expr>),
    Asinh(Box<Expr>),
    Acosh(Box<Expr>),
    Atanh(Box<Expr>),
    Hypot(Box<Expr>, Box<Expr>),
    ToDegrees(Box<Expr>),
    ToRadians(Box<Expr>),
    Min(Box<Expr>, Box<Expr>),
    Max(Box<Expr>, Box<Expr>),
    Copysign(Box<Expr>, Box<Expr>),
    Clamp(Box<Expr>, Box<Expr>, Box<Expr>),
}