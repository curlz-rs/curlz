use minijinja::Environment;

mod timestamp;

pub(super) fn register_functions(env: &mut Environment) {
    env.add_function("timestamp", timestamp::timestamp);
}
