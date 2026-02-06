#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_package_name() {
        let result = format_package_name();
        assert_eq!(result, expected_value);
    }
}
