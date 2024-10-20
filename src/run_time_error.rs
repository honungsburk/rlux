#[derive(Debug, Clone)]
pub enum RunTimeError {
    TypeError(String),
    DivideByZero(String),
    UndefinedVariable(String),
}