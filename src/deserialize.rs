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

/// ##### Variadic macro used to [`deserialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](macro.deserialize.html#compatible-variabless) from a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer). 
/// 
/// # Description
/// Variadic macro used to [`deserialize`](https://en.wikipedia.org/wiki/Serialization) [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
/// 
/// <b>deserialize! automatically creates variables when retrieving data.</b>
/// # Usage
/// `deserialize!(buffer, [bytes_read,] [0..n](v1, ..., vn):type, [0..n][optional_len_type :  s1, ..., sn]:type);`
/// * `buffer` - Unmutable reference to [`slice`] of [`u8`] to copy bytes from.
/// * bytes_read - (Optional) Identifier here can be used to get the count of bytes read from buffer.
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be read from buffer.
/// * One-to-many `[optional_len_type : s1, ..., sn]:type` where elements in `brackets[]` are the slices to be read from buffer.
///     * `optional_len_type` u8, u16, u32 default, u64 or u128 for the size of bytes used to encode length. 
/// 
/// # Example(s)
/// ```
/// // Import macro deserialize
/// use tampon::deserialize;
/// 
/// // Any buffer made with to_buffer! with (4554):u16, (65598.48896):f64, (c=12545566,d=456878):u32, ("Example string"):String, [vec![i32::MAX; 5]]:i32
/// let buffer: Vec<u8> = vec![202, 17, 145, 184, 199, 210, 231, 3, 240, 64, 30, 110, 191, 0, 174, 248, 6, 0, 14, 0, 0, 0, 69, 120, 97, 109, 112, 108, 101, 32, 115, 116, 114, 105, 110, 103, 5, 0, 0, 0, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127];
///
/// // Deserialize data from buffer. (variable are created during deserialization process)
/// deserialize!(buffer, bytes_read, (a):u16, (b):f64, (c,d):u32, (e):String, [f]:i32);
///
/// // Print result
/// println!("Bytes read={} | a={}, b={}, c={}, d={}, e={}, f={:?}", bytes_read, a,b,c,d,e,f);
/// ```
/// ##### Buffer smaller than content to retrieve will cause a panic! :
/// ``` should_panic
/// // Import macro deserialize
/// use tampon::deserialize;
/// 
/// // Any buffer made too small for data to retrieve.
/// let buffer: Vec<u8> = vec![202, 17, 145, 184, 199, 210, 231, 3, 240, 64, 30, 110, 191, 0, 174, 248, 6, 0, 14, 0, 0, 0, 69, 120, 97, 109, 112, 108, 101, 32, 115, 116, 114, 105, 110];
///
/// // Deserialize data from buffer (will panic!)
/// deserialize!(buffer, bytes_read, (_a):u16, (_b):f64, (_c,_d):u32, (_e):String, [_f]:i32);
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
/// 
/// # Panic(s)
/// * Will panic! if `buffer` length is smaller than all target length combined. Use [`deserialize_size`](crate::deserialize_size) to prevent it.
#[macro_export]
macro_rules! deserialize {
    ($buffer:expr, $( $tokens:tt : $tokens_type:ident ),+ ) => {    // Without optional parameter
        $crate::deserialize!($buffer, tampon_bytes_read, $( $tokens : $tokens_type ),+);
    };
    ($buffer:expr, $bytes_read : ident, $( $tokens:tt : $tokens_type:ident ),+ ) => {
        let mut $bytes_read : usize = 0;
        $(
            $crate::deserialize_parser! ( $buffer[$bytes_read..], $bytes_read, $tokens : $tokens_type);
        )+
    };
}

/// Hidden extension of the to_buffer! macro. Parse tokens. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! deserialize_parser {
    ($buffer:expr, $bytes_read : ident, ($($name:ident),+ ) : $tok_type : ident) => {  // Simple
        $(
            $crate::deserialize_parser!(@PARSE $bytes_read, $buffer, $name => $tok_type);
        )+


    };
    ($buffer:expr, $bytes_read : ident, [$($name:ident),+ ] : $tok_type : ident) => {  // Vector
         $(
            $crate::deserialize_parser!(@PARSE $bytes_read, $buffer, $name => [u32, $tok_type]);
        )+
    };
    ($buffer:expr, $bytes_read : ident, [$len_type:ty : $($name:ident),+ ] : $tok_type : ident) => {  // Vector with length type
         $(
            $crate::deserialize_parser!(@PARSE $bytes_read, $buffer, $name => [$len_type:ty, $tok_type]);
        )+
    };

    // Slice affectator with len type
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => [$len_type:ty, $type:ident]) => {

        // Keep bytes size of u32
        let len_bs = size_of::<$len_type>();

        // Get size of slice
        let slice_size = <$len_type>::from_le_bytes($buffer[0..len_bs].try_into().expect("Incorrect length!"));

        // Increase $bytes_read by u32 size
        $bytes_read += len_bs;

        // Init vector
        let mut $name:Vec<$type> = Vec::new();

        // Retrieve each slice
        for i in 0..slice_size {

            $crate::deserialize_parser!(@PARSE $bytes_read, $buffer, item_name => $type);
            $name.push(item_name);   // Push temporary variable into vector
        }       

    };


    /**********
    * BOOLEAN *
    **********/
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => bool) => {
        // Translate byte into u8
        let u8val = <u8>::from_le_bytes($buffer[0..core::mem::size_of::<u8>()].try_into().expect("Incorrect length!"));

        // Set bool value according to u8 value
        let $name = if u8val == 0 {
            false
        } else {
            true
        };

        // 1 byte was consumed for boolean
        $bytes_read += core::mem::size_of::<u8>();
    };


    /***********
    * NUMERICS * 
    ***********/
    (@NUM $bytes_read:expr, $buffer:expr, $name:ident, $type:ty) => {
        let $name = <$type>::from_le_bytes($buffer[0..size_of::<$type>()].try_into().expect("Incorrect length!"));
        $bytes_read += size_of::<$type>();
    };

    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => u8) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, u8 ); };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => u16) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, u16 ); };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => u32) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, u32 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => u64) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, u64 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => u128) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, u128 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => f32) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, f32 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => f64) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, f64 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => i8) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, i8 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => i16) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, i16 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => i32) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, i32 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => i64) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, i64 ) };
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => i128) => { $crate::deserialize_parser! (@NUM $bytes_read, $buffer, $name, i128 ) };

    /*********
    * STRING * 
    *********/
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => String) => {

        // Keep bytes size of u32
        let u32_bs = core::mem::size_of::<u32>();
        
        // Get size of string to retrieve
        let string_size = (<u32>::from_le_bytes($buffer[0..u32_bs].try_into().unwrap())) as usize;

        // Use String::from_utf8 which is SAFE https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8
        let $name = String::from_utf8($buffer[u32_bs..u32_bs + string_size].to_vec()).expect("UTF8 String incorrect!"); 

        // Return size used 
        $bytes_read += u32_bs + string_size;

    };

    /***************
    * TAMPON TRAIT * 
    ***************/
    (@PARSE $bytes_read:expr, $buffer:expr, $name:ident => $tampon:ident) => {

        let temp = $tampon::deserialize(&$buffer);
        let $name = temp.0;
        $bytes_read += temp.1;

    };
}