/// parses pairs like `"key=value"` strings into tuples of `Option<(key, value)>`
/// spaces around the [`separator`] are being removed
#[inline]
pub fn parse_pairs(pairs: &str, separator: char) -> Option<(&str, &str)> {
    pairs
        .split_once(separator)
        .map(|(key, value)| (key.trim(), value.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_defines_by_equal_happy_path() {
        assert_eq!(parse_pairs("foo=bar", '='), Some(("foo", "bar")));
        assert_eq!(parse_pairs("foo =bar", '='), Some(("foo", "bar")));
        assert_eq!(parse_pairs("foo = bar", '='), Some(("foo", "bar")));
        assert_eq!(parse_pairs("foo= bar", '='), Some(("foo", "bar")));
        assert_eq!(parse_pairs("foo: bar", ':'), Some(("foo", "bar")));
    }

    #[test]
    fn should_split_defines_by_equal_unhappy_path() {
        assert_eq!(
            parse_pairs("baz=123324+adf+=vasdf", '='),
            Some(("baz", "123324+adf+=vasdf"))
        );
    }

    #[test]
    fn should_not_split_defines_if_no_equal_is_contained() {
        assert_eq!(parse_pairs("foo", '='), None);
        assert_eq!(parse_pairs("baz=", '='), Some(("baz", "")));
    }
}
