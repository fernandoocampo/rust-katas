fn derive(coefficient: u32, exponent: u32) -> String {
    format!("{}x^{}", coefficient*exponent, exponent-1)
}

#[cfg(test)]
mod product_tests {
    use crate::integers::product;
    #[test]
    fn test_derive() {
        assert_eq!(product::derive(7, 8), "56x^7");
        assert_eq!(product::derive(5, 9), "45x^8");
    }
}