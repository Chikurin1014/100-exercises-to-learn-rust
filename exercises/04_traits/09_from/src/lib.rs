// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[allow(dead_code)]
pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

#[allow(dead_code, unused_variables)]
fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
