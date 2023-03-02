use minijinja::Environment;

mod basic;
mod jwt;

use basic::basic;
use jwt::jwt;

pub fn register_functions(env: &mut Environment) {
    env.add_function("jwt", jwt);
    env.add_function("basic", basic);
}
