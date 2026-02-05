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

/// ##### Variadic macro used to create a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) and [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](macro.buffer.html#compatible-variabless). 
/// 
/// # Description
/// Variadic macro used to create a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) and [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
/// 
/// # Usage
/// `let buffer = buffer!([0..n](v1, ..., vn):type, [0..n][optional_len_type : s1, ..., sn]:type);`
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be copied into created buffer.
/// * One-to-many `[s1, ..., sn]:type` where elements in `brackets[]` are the slices to be copied into created buffer.
///     * `optional_len_type` u8, u16, u32 default, u64 or u128 for the size of bytes used to encode length. 
/// 
/// # Return
/// New buffer created with argument(s) serialized with the size needed to contain them all.
/// 
/// # Example(s)
/// ```
/// // Import macro buffer
/// use tampon::buffer;
/// 
/// // Declare multiple variables (numerics don't need to be same type)
/// let a:u8 = 55;
/// let b:u8 = 255;
/// let c:u32 = 12545566;
/// let d:String = String::from("Example string");
/// let e:Vec<i32> = vec![i32::MAX; 50];
/// let f:Vec<f64> = vec![f64::MAX; 50];
/// let g:Vec<f64> = vec![f64::MAX; 50];
/// 
/// // Create a buffer and serialize data with buffer! macro.
/// let buffer:Vec<u8> = buffer!((a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // Print result
/// println!("Resulted buffer={:?}", buffer);
/// ```
/// 
/// # Compatible variables(s)
/// * [`bool`]
/// * All [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) except [`usize`] and [`isize`]
/// * [`String`] 
/// * Implementors of trait [`Tampon`](trait.Tampon.html)
/// * [`slice`] of the above types
/// 
/// # Endianness
/// * [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) bytes are written as [`little endian`](https://en.wikipedia.org/wiki/Endianness).
#[macro_export]
macro_rules! buffer {

    ($( $tokens:tt : $tokens_type:ident ),+ ) => {{
        // Get size needed for variable serialization
        let buffer_size = $crate::bytes_size!( $($tokens : $tokens_type),+);

        // Create mutable buffer of needed size
        let mut buffer:Vec<u8> = vec![0;buffer_size];

        // Serialize variable into vector
        $crate::serialize!(buffer, $($tokens : $tokens_type),+);


        // Return buffer
        buffer

    } as Vec<u8> };
}