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


/// ##### Trait used to [`serialize / deserialize`](https://en.wikipedia.org/wiki/Serialization) object.
/// 
/// This trait must be implemented if used by object in macros.
/// 
/// Functions can easily be implemented using macros (see example below).
/// 
/// # Example(s)
/// ```
/// // Import trait Tampon, macro bytes_size, deserialize and serialize
/// use tampon::{ Tampon, TamponError, bytes_size, deserialize, deserialize_size, serialize };
/// 
/// // Create a struct with variables
/// pub struct TamponExample {
///     f1:u8,
///     f2:u32,
///     f3:f64,
///     v1:Vec<u8>,
///     v2:Vec<f64>,
/// }
/// 
/// // Implement Tampon for struct
/// impl Tampon for TamponExample {
///     fn bytes_size(&self) -> usize {
///         // Use bytes_size! macro to easily return the size in bytes
///         bytes_size!((self.f1):u8, (self.f1):u32, (self.f3):f64, [self.v1]:u8, [self.v2]:f64)
///     }
/// 
/// 
///     fn deserialize_size(buffer : &[u8]) -> Result<usize, TamponError>{
///         // Use deserialize_size! macro to easily return the size in bytes
///         deserialize_size!(buffer, (f1):u8, (f1):u32, (f3):f64, [v1]:u8, [v2]:f64)
///     }
/// 
///     fn serialize(&self, buffer : &mut [u8]) -> usize {
///         // Use serialize! macro to serialize object, get the size with optional parameter and return it
///         serialize!(buffer, bytes_copied, (self.f1):u8, (self.f2):u32, (self.f3):f64, [self.v1]:u8, [self.v2]:f64);
///         bytes_copied
///     }
///
///     fn deserialize(buffer : &[u8]) ->  (Self, usize) {
///         // Use deserialize! macro to deserialize data
///         deserialize!(buffer, bytes_read, (f1):u8, (f2):u32, (f3):f64, [v1]:u8, [v2]:f64);
///         (TamponExample {f1,f2,f3,v1,v2 }, bytes_read)     
///     }
/// }
/// ```
pub trait Tampon {
    /// Size of the trait implementation in [`bytes`](https://en.wikipedia.org/wiki/Byte).
    /// 
    /// Use macro [`bytes_size!`](crate::bytes_size) to easily return the size in bytes.
    /// 
    /// # Example(s)
    /// ```ignore
    /// fn bytes_size(&self) -> usize {
    ///     bytes_size!((self.f1):u8, (self.f1):u32, (self.f3):f64, [self.v1]:u8, [self.v2]:f64)
    /// }
    /// ```
    fn bytes_size(&self) -> usize;


    /// Size of the trait implementation from [`bytes`](https://en.wikipedia.org/wiki/Byte).
    /// 
    /// Use macro [`deserialize_size!`](crate::deserialize_size) to easily return the size in bytes.
    /// 
    /// # Example(s)
    /// ```ignore
    /// fn deserialize_size(&self) -> -> Result<usize, TamponError> {
    ///     deserialize_size!((f1):u8, (f1):u32, (f3):f64, [v1]:u8, [v2]:f64)
    /// }
    /// ```
    fn deserialize_size(buffer : &[u8]) -> Result<usize, crate::TamponError>;

    /// Serialize object variable into buffer. 
    /// 
    /// Use macro [`serialize!`](crate::serialize) to easily serialize and get size in bytes.
    /// 
    /// # Argument(s)
    /// * `buffer` - Mutable buffer slice reference to serialize into. 
    /// 
    /// # Returns
    /// - Size of bytes written as [`usize`].  
    /// 
    /// # Example(s)
    /// ```ignore
    /// fn serialize(&self, buffer : &mut [u8]) -> usize {
    ///     serialize!(buffer, bytes_copied, (self.f1):u8, (self.f2):u32, (self.f3):f64, [self.v1]:u8, [self.v2]:f64)
    ///     bytes_copied
    /// }
    /// ```
    fn serialize(&self, buffer : &mut [u8]) -> usize;

    /// Deserialize a new variable instance from buffer and return it with bytes read.
    /// 
    /// Use macro [`deserialize!`](crate::deserialize) to easily deserialize and get size in bytes.
    /// # Argument(s)
    /// * `buffer` - Non-mutable buffer slice reference to deserialize from. 
    ///
    /// # Returns
    /// - Tuple of new object and bytes read from buffer.
    /// 
    /// # Example(s)
    /// ```ignore
    /// fn deserialize(buffer : &[u8]) -> (Self, usize) {
    ///     deserialize!(buffer, bytes_read, (f1):u8, (f2):u32, (f3):f64, [v1]:u8, [v2]:f64);
    ///     (TamponExample {f1,f2,f3,v1,v2 }, bytes_read)
    /// }
    /// ```
    fn deserialize(buffer : &[u8]) -> (Self, usize) where Self: Sized;
}