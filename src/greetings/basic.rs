fn hello(name: &str) -> String {
    format!("Hello {}", name)
}

fn hello_world() -> String {
    "Hello World".to_string()
}

#[cfg(test)]
mod hello_world_tests {
    use crate::greetings::basic;

    #[test]
    fn test_hello_world() {
        // Given
        let want = "Hello World";
        // When
        let got = basic::hello_world();
        // Then
        assert_eq!(want, got);
    }

    #[test]
    fn test_hello_someone() {
        // Given
        let name = "Fernando";
        let want = "Hello Fernando";
        // When
        let got = basic::hello(name);
        // Then
        assert_eq!(want, got);
    }
}