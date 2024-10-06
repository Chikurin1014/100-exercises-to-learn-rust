// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

// MY_NOTE: You can write like this where `Step` trait and `num` trait is available:
// MY_NOTE: trait Power<Exp: std::iter::Step + num::traits::Zero> {
// MY_NOTE:     fn power(self, exp: Exp) -> Self;
// MY_NOTE: }
// MY_NOTE: > You cannot (yet) implement Iterator for ranges over custom types,
// MY_NOTE: > until Step (or some replacement API) is stabilised.
// MY_NOTE: > cite: https://stackoverflow.com/questions/56986468/why-am-i-getting-an-error-about-a-missing-unstable-trait-stditerstep-whic
trait Power<Exp> {
    fn power(self, exp: Exp) -> Self;
}

impl Power<u16> for u32 {
    fn power(self, exp: u16) -> Self {
        let mut power = 1;
        for _ in 0..exp {
            power *= self;
        }
        power
    }
}

impl Power<u32> for u32 {
    fn power(self, exp: u32) -> Self {
        let mut power = 1;
        for _ in 0..exp {
            power *= self;
        }
        power
    }
}

impl Power<&u32> for u32 {
    fn power(self, exp: &u32) -> Self {
        let mut power = 1;
        for _ in 0..*exp {
            power *= self;
        }
        power
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
