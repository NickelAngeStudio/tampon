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

/// ##### Variadic macro used to get the size in [`bytes`](https://en.wikipedia.org/wiki/Byte) of [`compatible variables`](macro.serialize_size.html#compatible-variabless) to [`serialize`](https://en.wikipedia.org/wiki/Serialization).
/// 
/// Variadic macro used to get the size in [`bytes`](https://en.wikipedia.org/wiki/Byte) of [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
///
/// # Usage
/// `let size = serialize_size!([0..n](v1, ..., vn):type, [0..n][optional_len_type :  s1, ..., sn]:type);`
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be sized.
/// * One-to-many `[s1, ..., sn]:type` where elements in `brackets[]` are the slices to be sized.
///     * `optional_len_type` u8, u16, u32 default, u64 or u128 for the size of bytes used to encode length. 
/// 
/// # Return
/// Size in bytes of all arguments as [`usize`].
/// 
/// # Example(s)
/// ```
/// // Import macro
/// use tampon::serialize_size;
/// 
/// // Declare multiple variables
/// let a:u8 = 55;
/// let b:u8 = 255;
/// let c:u32 = 12545566;
/// let d:String = String::from("Example string");
/// let e:Vec<i32> = vec![i32::MAX; 50];
/// let f:Vec<f64> = vec![f64::MAX; 50];
/// let g:Vec<f64> = vec![f64::MAX; 50];
/// 
/// // Get the size in bytes of all those elements in one macro call
/// let size = serialize_size!((a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // Print result
/// println!("Bytes size of variables a,b,c,d,e,f,g is {}", size);
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
macro_rules! serialize_size {
    // Return 0 on empty
    () => {{ 0 } as usize };

    ($( $tokens:tt : $tokens_type:ident ),+ ) => {{

        $(
            $crate::serialize_size_parser! ($tokens : $tokens_type) +
        )+
        0

    } as usize};

}



/// Hidden extension of the serialize_size! macro. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! serialize_size_parser {
    (($($name:expr),+ ) : $tok_type : ident) => {{  // Simple
        $(
            $crate::serialize_size_parser!(@PARSE $name => $tok_type) +
        )+
        0
    } as usize };

    ([$($name:expr),+ ] : $tok_type : ident) => {{ // Vector
         $(
            $crate::serialize_size_parser!(@PARSE $name => [u32, $tok_type]) +
        )+
        0
    } as usize };
    ([$len_type:ty : $($name:expr),+ ] : $tok_type : ident) => {{  // Vector with length type
         $(
            $crate::serialize_size_parser!(@PARSE $name => [$len_type:ty, $tok_type]) +
        )+
        0
    } as usize };



    /**********
     * SLICES *
     *********/
    (@PARSE $name:expr => [$len_type:ty, $tok_type : ident]) => {{ 
        // Size padding + length of slice * size of elements        
        if $name.len() > 0 {
            let mut ret : usize = 0;
            for elem in $name.iter() {
                ret += $crate::serialize_size_parser!(@PARSE *elem => $tok_type);
            } 
            size_of::<$len_type>() + ret
            //size_of::<$len_type>() +  $name.len() * $crate::serialize_size_parser!(@PARSE $name[0] => $tok_type)
        } else {
            size_of::<$len_type>()
        }

    } as usize };


    /**********
    * BOOLEAN *
    **********/
    (@PARSE $expr:expr => bool) => {{ 
        // bool use 1 byte (even if 1 bit)
        1 
    } as usize };

    /***********
    * NUMERICS * 
    ***********/
    (@NUM $name:expr, $type:ty) => {{ size_of::<$type>() } as usize };

    (@PARSE $name:expr => u8) => {{ $crate::serialize_size_parser! (@NUM $name, u8 ) } as usize };
    (@PARSE $name:expr => u16) => {{ $crate::serialize_size_parser! (@NUM $name, u16 ) } as usize };
    (@PARSE $name:expr => u32) => {{ $crate::serialize_size_parser! (@NUM $name, u32 ) } as usize };
    (@PARSE $name:expr => u64) => {{ $crate::serialize_size_parser! (@NUM $name, u64 ) } as usize };
    (@PARSE $name:expr => u128) => {{ $crate::serialize_size_parser! (@NUM $name, u128 ) } as usize };
    (@PARSE $name:expr => f32) => {{ $crate::serialize_size_parser! (@NUM $name, f32 ) } as usize };
    (@PARSE $name:expr => f64) => {{ $crate::serialize_size_parser! (@NUM $name, f64 ) } as usize };
    (@PARSE $name:expr => i8) => {{ $crate::serialize_size_parser! (@NUM $name, i8 ) } as usize };
    (@PARSE $name:expr => i16) => {{ $crate::serialize_size_parser! (@NUM $name, i16 ) } as usize };
    (@PARSE $name:expr => i32) => {{ $crate::serialize_size_parser! (@NUM $name, i32 ) } as usize };
    (@PARSE $name:expr => i64) => {{ $crate::serialize_size_parser! (@NUM $name, i64 ) } as usize };
    (@PARSE $name:expr => i128) => {{ $crate::serialize_size_parser! (@NUM $name, i128 ) } as usize };

    /*********
    * STRING * 
    *********/
    (@PARSE $expr:expr => String) => {{ 
        // String is a slice of char and need to pad the size
        // String::len() gives size of string in bytes (https://doc.rust-lang.org/std/string/struct.String.html#method.len-1)
        size_of::<u32>() + $expr.len()
    } as usize };


    /***************
    * TAMPON TRAIT * 
    ***************/
    (@PARSE $expr:expr => $tampon:ident) => {{
        $expr.bytes_size()
    } as usize };

}