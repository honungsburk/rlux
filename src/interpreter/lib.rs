//! Standard library for lux. All definitions are available in the global scope.
//!
//!

use super::{Environment, LuxValue, RuntimeError};
use std::time::{SystemTime, UNIX_EPOCH};


/// Read the current time in milliseconds
fn clock(_: &[LuxValue]) -> Result<LuxValue, RuntimeError> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    Ok(super::LuxValue::Number(in_ms as f64))
}

/// Load the standard library into an environment.
pub fn load(env: &mut Environment) {
    let clock = LuxValue::native_function("clock", 0, clock);
    env.define("clock".to_string(), clock);
}