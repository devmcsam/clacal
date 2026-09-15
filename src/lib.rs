pub enum Expr {
    /// An `Operator`
    Operator,
    /// A number that is known at parse time
    NumLiteral,
    /// A function that is called
    Function,
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

pub enum NumLiteral {
    /// A number that can be exactly represented without a decimal.
    Integer(i64),
    /// Any real number.
    Float(f64),
}

pub enum Function {
    /// A function natively supported by this crate.
    BuiltInFunction,
    /// A function that has been defined either buy users of this crate or created at runtime.
    UserFunction,
}