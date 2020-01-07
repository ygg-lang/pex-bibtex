use core::fmt::Write;

mod for_3rd;
mod for_std;

pub trait Latexify {
    fn latexify<W: Write>(&self, f: &mut W) -> core::fmt::Result;
    fn to_latex(&self) -> String {
        let mut s = String::new();
        self.latexify(&mut s).unwrap();
        s
    }
}
