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


/// ##### Variadic macro used to [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](macro.serialize.html#compatible-variabless) into a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer). 
/// 
/// # Description
/// Variadic macro used to [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
/// 
/// # Usage
/// `serialize!(buffer, [bytes_copied,] [0..n](v1, ..., vn):type, [0..n][optional_len_type : s1, ..., sn]:type);`
/// * `buffer` - Mutable reference to [`slice`] of [`u8`] to copy bytes into.
/// * `bytes_copied` - (Optional) Identifier here can be used to get the count of bytes copied into buffer.
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be copied into buffer.
/// * One-to-many `[optional_len_type : s1, ..., sn]:type` where elements in `brackets[]` are the slices to be copied into buffer.
///      * `optional_len_type` u8, u16, u32 default, u64 or u128 for the size of bytes used to encode length. 
/// 
/// # Example(s)
/// ```
/// // Import macro bytes_size and serialize
/// use tampon::bytes_size;
/// use tampon::serialize;
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
/// // Get the size in bytes of all those elements in one macro call
/// let size = bytes_size!((a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // Create a mutable buffer long enough to store variables
/// let mut buffer:Vec<u8> = vec![0;size];
/// 
/// // Serialize variables content into buffer
/// serialize!(buffer, bytes_copied, (a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // (optional) Make sure bytes copied == bytes_size!
/// assert!(size == bytes_copied);
/// 
/// // Print result
/// println!("Bytes size of variables a,b,c,d,e,f is {}\nCopied size is {}\nResulted buffer={:?}", size, bytes_copied, buffer);
/// ```
/// 
/// ##### Buffer smaller than content will cause a panic! :
/// ```should_panic
/// // Import macro bytes_size and serialize
/// use tampon::bytes_size;
/// use tampon::serialize;
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
/// // Mutable buffer TOO SMALL to contains variable
/// let mut buffer:Vec<u8> = vec![0;50];
/// 
/// // Copy variables content into buffer (will panic!)
/// serialize!(buffer, (a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
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
///
/// Will panic! if buffer length is smaller than all sources length combined. Use [`bytes_size!`](crate::bytes_size) to validate buffer size.


#[macro_export]
macro_rules! serialize {
    ($buffer:expr, $( $tokens:tt : $tokens_type:ident ),+ ) => {    // Without optional parameter
        $crate::serialize! ( $buffer, tampon_bytes_written, $($tokens : $tokens_type),+);        
    };
    ($buffer:expr, $bytes_written:ident, $( $tokens:tt : $tokens_type:ident ),+ ) => {
        let mut $bytes_written : usize = 0;   
        $(
            $crate::serialize_parser! ( $buffer[$bytes_written..], $bytes_written, $tokens : $tokens_type);
        )+        
    };
    ($buffer:expr) => { };
    ($buffer:expr, $bytes_written:ident ) => {
        let $bytes_written = 0;
    }
}

/// Hidden extension of the serialize! macro. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! serialize_parser {
    ($buffer:expr, $bytes_written : ident, ($($name:expr),+ ) : $tok_type : ident) => {  // Simple
        $(
            $crate::serialize_parser!(@PARSE $bytes_written, $buffer, $name => $tok_type);
        )+


    };
    ($buffer:expr, $bytes_written : ident, [$($name:expr),+ ] : $tok_type : ident) => {  // Vector
         $(
            $crate::serialize_parser!(@PARSE $bytes_written, $buffer, $name => [u32, $tok_type]);
        )+
    };
    ($buffer:expr, $bytes_written : ident, [$len_type:ty : $($name:expr),+ ] : $tok_type : ident) => {  // Vector with length type
         $(
            $crate::serialize_parser!(@PARSE $bytes_written, $buffer, $name => [$len_type, $tok_type]);
        )+
    };


    // Slice affectator with len type
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => [$len_type:ty, $type:ident]) => {

        // Write size of slice according to $len_type
        let bytes_len = ($name.len() as $len_type).to_le_bytes();
        $buffer[0..bytes_len.len()].copy_from_slice(&bytes_len);

        // Init bytes_copied at bytes_len.len() since we will loop slice
        $bytes_written += bytes_len.len();

    
        // Loop and accumulate element of slice
        for elem in $name.iter() {
            $crate::serialize_parser!(@PARSE $bytes_written, $buffer[0..], *elem => $type);
        } 

    };


    /**********
    * BOOLEAN *
    **********/
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => bool) => {
        $bytes_written += {
        if($name) {
                //$crate::serialize_retriever!($buffer, 1 => u8)
                let bytes = (1 as u8).to_le_bytes();
                $buffer[0..bytes.len()].copy_from_slice(&bytes);
                bytes.len()
            } else {
                let bytes = (0 as u8).to_le_bytes();
                $buffer[0..bytes.len()].copy_from_slice(&bytes);
                bytes.len()
            }
        };
    };


    /***********
    * NUMERICS * 
    ***********/
    (@NUM $bytes_written:expr, $buffer:expr, $name:expr, $type:ty) => {
        // Transform the expression into bytes
        let bytes = <$type>::to_le_bytes($name);

        // Copy to buffer via copy from slice
        $buffer[0..bytes.len()].copy_from_slice(&bytes);

        // Return size used
        $bytes_written += bytes.len()
    };

    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => u8) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, u8 ); };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => u16) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, u16 ); };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => u32) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, u32 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => u64) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, u64 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => u128) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, u128 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => f32) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, f32 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => f64) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, f64 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => i8) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, i8 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => i16) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, i16 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => i32) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, i32 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => i64) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, i64 ) };
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => i128) => { $crate::serialize_parser! (@NUM $bytes_written, $buffer, $name, i128 ) };

    /*********
    * STRING * 
    *********/
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => String) => {
         // Write size of String
        let bytes_size = ($name.len() as u32).to_le_bytes();
        $buffer[0..bytes_size.len()].copy_from_slice(&bytes_size);
        // Transform String as bytes slice
        let bytes = $name.as_bytes();
        // Copy to buffer via copy from slice
        $buffer[bytes_size.len()..(bytes_size.len() + bytes.len())].copy_from_slice(&bytes);
        // Return size used
        $bytes_written += bytes_size.len() + bytes.len()
       
    };

    /***************
    * TAMPON TRAIT * 
    ***************/
    (@PARSE $bytes_written:expr, $buffer:expr, $name:expr => $tampon:ident) => {
        $bytes_written += $name.serialize(&mut $buffer);
    };

}
