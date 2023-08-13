#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_package_name() {
        // You can directly access functions/types from the module
        let result = format_package_name();
        assert_eq!(result, expected_value);
    }
}
