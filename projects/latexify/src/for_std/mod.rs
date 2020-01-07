use core::fmt::Write;
use crate::Latexify;

macro_rules! impl_number {
    ($($t:ty),*) => {
        $(
            impl Latexify for $t {
                fn fmt<W: Write>(&self, f: &mut W) -> std::fmt::Result {
                    write!(f, "{}", self)
                }
            }
        )*
    };
}

impl_number![i8, i16, i32, i64, i128, isize];
impl_number![u8, u16, u32, u64, u128, usize];
impl_number![f32, f64];