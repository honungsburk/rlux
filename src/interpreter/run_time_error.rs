use super::LuxValue;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    TypeError(String),
    DivideByZero(String),
    UndefinedVariable(String),
    UnsupportedType(String),
    Return(LuxValue)
}