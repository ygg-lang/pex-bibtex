#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use core::fmt::Write;

mod for_3rd;
mod for_std;

// noinspection SpellCheckingInspection
/// A trait for LaTeX representation.
pub trait Latexify {
    /// Write LaTeX representation to the formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use latexify::Latexify;
    /// # fn main() -> std::fmt::Result {
    /// let mut buffer = String::new();
    /// Latexify::fmt(&0, &mut buffer)?;
    /// println!("{}", buffer); // 0
    /// //
    /// # Ok(())
    /// # }
    /// ```
    fn fmt<W: Write>(&self, f: &mut W) -> core::fmt::Result;
    /// Get the LaTeX string of raw object.
    ///
    /// # Examples
    ///
    /// ```
    /// use latexify::Latexify;
    /// 0.to_latex(); // "0"
    /// ```
    fn to_latex(&self) -> String {
        let mut s = String::new();
        Latexify::fmt(self, &mut s).unwrap();
        s
    }
}
