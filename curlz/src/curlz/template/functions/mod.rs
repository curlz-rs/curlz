use minijinja::value::Value;
use minijinja::Environment;

mod auth;
mod chrono;
mod process_env;
mod prompt;

pub(super) fn register_functions(env: &mut Environment) {
    env.add_function("processEnv", process_env::process_env);
    env.add_function("process_env", process_env::process_env);
    // this provides lazy env var lookup
    env.add_global("env", Value::from_struct_object(process_env::ProcessEnv));

    prompt::register_functions(env);
    auth::register_functions(env);
    chrono::register_functions(env);
}
