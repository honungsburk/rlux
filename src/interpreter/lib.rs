//! Standard library for lux. All definitions are available in the global scope.
//!
//!

use super::{value::NativeFunction, Environment, LuxCallable, LuxValue};
use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
struct Clock;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native clock>") // Customize as needed
    }
}

impl LuxCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        self: std::rc::Rc<Self>,
        _interpreter: &mut super::Interpreter,
        _args: &[super::LuxValue],
    ) -> Result<super::LuxValue, super::RunTimeError> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let in_ms =
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

        Ok(super::LuxValue::Number(in_ms as f64))
    }
}

pub fn load(env: &mut Environment) {
    let clock = LuxValue::native_function("clock", 0, |_| {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let in_ms =
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

        Ok(super::LuxValue::Number(in_ms as f64))
    });
    env.define("clock".to_string(), clock);
}
