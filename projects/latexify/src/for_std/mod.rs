use core::fmt::Write;
use crate::Latexify;

macro_rules! impl_number {
    ($($t:ty),*) => {
        $(
            impl Latexify for $t {
                type Context = ();
                fn fmt<W: Write>(&self, _: &Self::Context, f: &mut W) -> std::fmt::Result {
                    f.write_fmt(format_args!("{}", self))
                }
                fn to_latex(&self, _: &Self::Context) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_number![i8, i16, i32, i64, i128, isize];
impl_number![u8, u16, u32, u64, u128, usize];

