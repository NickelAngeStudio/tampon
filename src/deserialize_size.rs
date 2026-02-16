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


/// Variadic macro used to determine the buffer size in [`bytes`](https://en.wikipedia.org/wiki/Byte) needed to [`deserialize`](https://en.wikipedia.org/wiki/Serialization).
/// 
/// # Usage
/// `let size = deserialize_size!(buffer, [max_size,] [0..n](v1, ..., vn):type, [0..n][optional_len_type :  s1, ..., sn]:type);`
/// * `buffer` - Unmutable reference to [`slice`] of [`u8`] to determine size from.
/// * max_size - (Optional) Maximum size for deserialize_size to prevent deserialize bomb or malformed binaries. Default : 0 = no limit.
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be sized.
/// * One-to-many `[s1, ..., sn]:type` where elements in `brackets[]` are the slices to be sized.
///     * `optional_len_type` u8, u16, u32 default, u64 or u128 for the size of bytes used to encode length. 
/// 
/// # Return
/// A `Result` which is 
/// - `Ok` :  Size in bytes of all arguments as [`usize`].
/// - `Err` : [`DeserializeSizeBufferIncomplete`](crate::TamponError::DeserializeSizeBufferIncomplete) if buffer is too small and/or corrupted.
/// - `Err` : [`DeserializeSizeBufferIncomplete`](crate::TamponError::DeserializeSizeGreaterThanMax) if size is bigger than optional max_size.
///
/// # Example(s)
/// ```
/// // Import macro
/// use tampon::deserialize_size;
/// 
/// // Any buffer made with to_buffer! with (4554):u16, (65598.48896):f64, (c=12545566,d=456878):u32, ("Example string"):String, [vec![i32::MAX; 5]]:i32
/// let buffer: Vec<u8> = vec![202, 17, 145, 184, 199, 210, 231, 3, 240, 64, 30, 110, 191, 0, 174, 248, 6, 0, 14, 0, 0, 0, 69, 120, 97, 109, 112, 108, 101, 32, 115, 116, 114, 105, 110, 103, 5, 0, 0, 0, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127];
///
/// // Get the size in bytes of all those elements in one macro call
/// match deserialize_size!(buffer, (a):u16, (b):f64, (c,d):u32, (e):String, [f]:i32) {
///     Ok(size) => assert_eq!(size, buffer.len()),
///     Err(err) => panic!("corrupted buffer")
/// }
/// ```
/// 
/// # Compatible variables(s)
/// * [`bool`]
/// * All [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) except [`usize`] and [`isize`]
/// * [`String`] 
/// * Implementors of trait [`Tampon`](trait.Tampon.html)
/// * [`slice`] of the above types
/// 
#[macro_export]
macro_rules! deserialize_size {

    ($buffer:expr, $( $tokens:tt : $tokens_type:ident ),+ ) => {{

        $crate::deserialize_size!($buffer, usize::MAX, $( $tokens : $tokens_type ),+)

    } as Result<usize, $crate::TamponError>};

    ($buffer:expr, $max_size : expr, $( $tokens:tt : $tokens_type:ident ),+ ) => {{
    
        let mut deserialize_status : u8 = 0;    // 0 is okay, 1 is incomplete, 2 is size > max
        let mut tampon_bytes_read = 0;
        $(
            if deserialize_status == 0 {
                $crate::deserialize_size_parser! ( $buffer[tampon_bytes_read..], deserialize_status, $max_size, tampon_bytes_read, $tokens : $tokens_type);
            }
        )+

        match deserialize_status {
            1 => Err($crate::TamponError::DeserializeSizeBufferIncomplete),
            2 => Err($crate::TamponError::DeserializeSizeGreaterThanMax),
            _ =>  Ok(tampon_bytes_read),
        }

    } as Result<usize, $crate::TamponError>};

    ($buffer:expr) => {{

        Ok(0 as usize)

    } as Result<usize, $crate::TamponError>};

    ($buffer:expr, $max_size : expr) => {{

        Ok(0 as usize)

    } as Result<usize, $crate::TamponError>};

}


/// Hidden extension of the to_buffer! macro. Parse tokens. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! deserialize_size_parser {
    ($buffer:expr, $deserialize_status: expr, $max_size : expr, $bytes_read : ident, ($($name:ident),+ ) : $tok_type : ident) => {  // Simple
        
        $(
            if $deserialize_status == 0 {
                $crate::deserialize_size_parser!(@PARSE $deserialize_status, $max_size, $bytes_read, $buffer, $name => $tok_type);
            }
        )+
       

    };

    ($buffer:expr,  $deserialize_status: expr, $max_size : expr, $bytes_read : ident, [$($name:ident),+ ] : $tok_type : ident) => {  // Vector
         $(
            if $deserialize_status == 0 {
                $crate::deserialize_size_parser!(@PARSE $deserialize_status, $max_size, $bytes_read, $buffer, $name => [u32, $tok_type]);
            }
        )+
    };

    ($buffer:expr, $deserialize_status : expr, $max_size : expr, $bytes_read : ident, [$len_type:ty : $($name:ident),+ ] : $tok_type : ident) => { // Vector with length type
        $(
            if $deserialize_status == 0 {
                $crate::deserialize_size_parser!(@PARSE $deserialize_status, $max_size, $bytes_read, $buffer, $name => [$len_type, $tok_type]);
            }
        )+

    };

    // Slice affectator with len type
    (@PARSE $deserialize_status : expr, $max_size : expr,  $bytes_read:expr, $buffer:expr, $name:ident => [$len_type:ty, $type:ident]) => {


        // Keep bytes size $len_type
        let len_bs = size_of::<$len_type>();

        if len_bs > $buffer.len() {
            $deserialize_status = 1;
        } else {
            let slice_size = <$len_type>::from_le_bytes($buffer[0..len_bs].try_into().expect(""));
            $bytes_read += len_bs;

            if $max_size > 0 && $bytes_read > $max_size {   // Test max size
                $deserialize_status = 2;
            } 
            
            // Retrieve each slice
            for i in 0..slice_size {
                if $deserialize_status == 0 {
                    $crate::deserialize_size_parser!(@PARSE  $deserialize_status, $max_size,  $bytes_read, $buffer, item_name => $type);
                }
            }       
        }


    };


    /**********
    * BOOLEAN *
    **********/
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => bool) => {
        if $buffer.len() < size_of::<u8>() {
            $deserialize_status = 1;
        } else {
            $bytes_read += 1;
            if $max_size > 0 && $bytes_read > $max_size {   // Test max size
                $deserialize_status = 2;
            } 
        }
    };


    /***********
    * NUMERICS * 
    ***********/
    (@NUM $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident, $type:ty) => {
        if $buffer.len() < size_of::<$type>() {
            $deserialize_status = 1;
        } else {
            $bytes_read += size_of::<$type>();
            if $max_size > 0 && $bytes_read > $max_size {   // Test max size
                $deserialize_status = 2;
            } 
        }
    };


    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => u8) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size, $bytes_read, $buffer, $name, u8 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => u16) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, u16 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => u32) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, u32 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => u64) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, u64 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => u128) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, u128 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => f32) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, f32 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => f64) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, f64 )};
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => i8) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, i8 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => i16) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, i16 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => i32) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, i32 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => i64) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, i64 ) };
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => i128) => 
        { $crate::deserialize_size_parser! (@NUM $deserialize_status, $max_size,  $bytes_read, $buffer, $name, i128 ) };

    /*********
    * STRING * 
    *********/
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => String) => {

        // Keep bytes size of u32
        let u32_bs = size_of::<u32>();

        if u32_bs > $buffer.len() {
            $deserialize_status = 1;
        } else {
             // Get size of string to retrieve
            let string_size = <u32>::from_le_bytes($buffer[0..u32_bs].try_into().expect("")) as usize;
            if $buffer.len() < string_size + u32_bs {
                $deserialize_status = 1;
            } else {
                $bytes_read += u32_bs + string_size;
                if $max_size > 0 && $bytes_read > $max_size {   // Test max size
                    $deserialize_status = 2;
                } 
            }
        }     

    };

    /***************
    * TAMPON TRAIT * 
    ***************/
    (@PARSE $deserialize_status : expr, $max_size : expr, $bytes_read:expr, $buffer:expr, $name:ident => $tampon:ident) => {
        match $tampon::deserialize_size(&$buffer, $max_size - $bytes_read) {
            Ok(size) => {
                $bytes_read += size;
                if $max_size > 0 && $bytes_read > $max_size {   // Test max size
                    $deserialize_status = 2;
                } 
            },
            Err(err) => match err {
                $crate::TamponError::DeserializeSizeBufferIncomplete => $deserialize_status = 1,
                $crate::TamponError::DeserializeSizeGreaterThanMax => $deserialize_status = 2,

            }
        }
        
    };
}