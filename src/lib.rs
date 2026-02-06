#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]

/* 
Copyright (c) 2026  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/tampon

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

 // # Re-export for Public API
 #[doc(inline)]
 pub use generate::generate_buffer as generate_buffer;
 pub use generate::buffer_generator_charset as buffer_generator_charset;
 pub use wipe::wipe_buffer as wipe_buffer;
 pub use compare::compare_buffers as compare_buffers;
 pub use crate::tampon::Tampon as Tampon;
 pub use error::TamponError as TamponError;

/// Generate buffer
#[doc(hidden)]
pub mod generate;

/// Wipe buffer
#[doc(hidden)]
pub mod wipe;

/// Compare buffers
#[doc(hidden)]
pub mod compare;

/// Tampon trait
#[doc(hidden)]
pub mod tampon;

/// errors
#[doc(hidden)]
pub mod error;

/// bytes_size macro
#[doc(hidden)]
pub mod bytes_size;

/// serialize macro
#[doc(hidden)]
pub mod serialize;

/// deserialize_size macro
#[doc(hidden)]
pub mod deserialize_size;

/// deserialize macro
#[doc(hidden)]
pub mod deserialize;

/// buffer! macro
#[doc(hidden)]
pub mod buffer;

// Tests module folder
#[cfg(test)]
mod test;
