pub fn factorial(n: u32) -> u32 {
    let mut result = 1u32;
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // rather than overflowing and wrapping around

        // MY_NOTE: In order to call method `saturating_mul`, the type of `result` must be defined explicitly when it is declaired
        // MY_NOTE: `saturating_mul`を呼ぶためには`result`の型が宣言時に明示されている必要がある
        result = result.saturating_mul(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
