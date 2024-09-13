// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

pub use saturating_u16::SaturatingU16;

mod saturating_u16 {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct SaturatingU16(u16);

    impl From<u16> for SaturatingU16 {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<u8> for SaturatingU16 {
        fn from(value: u8) -> Self {
            Self(value.into())
        }
    }
    impl From<&u16> for SaturatingU16 {
        fn from(value: &u16) -> Self {
            Self(*value)
        }
    }
    impl From<&u8> for SaturatingU16 {
        fn from(value: &u8) -> Self {
            Self((*value).into())
        }
    }
    impl std::ops::Add for SaturatingU16 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0.saturating_add(rhs.0))
        }
    }
    impl std::ops::Add<u16> for SaturatingU16 {
        type Output = Self;
        fn add(self, rhs: u16) -> Self::Output {
            Self(self.0.saturating_add(rhs))
        }
    }
    impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
        type Output = Self;
        fn add(self, rhs: &SaturatingU16) -> Self::Output {
            Self(self.0.saturating_add(rhs.0))
        }
    }
    impl std::ops::Add<&u16> for SaturatingU16 {
        type Output = Self;
        fn add(self, rhs: &u16) -> Self::Output {
            Self(self.0.saturating_add(*rhs))
        }
    }
    impl PartialEq<u16> for SaturatingU16 {
        fn eq(&self, other: &u16) -> bool {
            self.0.eq(other)
        }
    }
    impl PartialOrd<u16> for SaturatingU16 {
        fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(other)
        }
    }
}
