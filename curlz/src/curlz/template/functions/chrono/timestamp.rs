use chrono::Utc;
use minijinja::value::Value;
use minijinja::State;

pub(super) fn timestamp(_: &State) -> Value {
    Value::from(Utc::now().timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::RenderBuilder;
    use chrono::Utc;

    #[test]
    fn test_timestamp() {
        let now = Utc::now();
        let timestamp_result = RenderBuilder::new()
            .with_function("timestamp", timestamp)
            .render(r#"now = {{ timestamp() }}"#);

        assert_eq!(format!("now = {}", now.timestamp()), timestamp_result);
    }
}
