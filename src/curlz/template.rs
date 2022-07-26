use minijinja::value::Value;

pub fn render<'source>(env: &mut minijinja::Environment<'source>, ctx: &Value, str: &'source str, name: &'source str) -> crate::Result<String> {
    env.add_template(name, str)?;
    let template = env.get_template(name)?;

    template.render(&ctx).map_err(|e| e.into())
}
