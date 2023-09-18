fn balance(value: &str) -> bool {
    let mut result = true;
    let mut char_stack = Vec::<char>::new();

    for c in value.chars() {
        if c == '(' || c == '[' || c == '{' {
            char_stack.push(c);
            continue;
        }

        if char_stack.is_empty() {
            result = false;
            break;
        }

        let item = char_stack.pop();

        if item == Some('(') && c == ')' {
            continue;
        }

        if item == Some('[') && c == ']' {
            continue;
        }

        if item == Some('{') && c == '}' {
            continue;
        }

        result = false;
        break;
    }

    result
}

#[cfg(test)]
mod parentheses_tests {
    use crate::o_balance::parentheses::balance;
    use std::collections::HashMap;

    #[test]
    fn test_balance() {
        // Given
        let test_cases = HashMap::from([
            ("{{()}}", true),
            ("{{()]", false),
            ("(((())))", true),
            ("((((}})))", false),
            ("(((())))", true),
            ("[((()))]", true),
            ("(()))", false),
            (")(", false),
        ]);

        // When
        for (value, want) in test_cases.into_iter() {
            let got = balance(value);
            // Then
            assert_eq!(want, got, "{value} should be {want}");
        }
    }
}
