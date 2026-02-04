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
/// This trait must be implemented by object that needs to be [`serialize / deserialize`](https://en.wikipedia.org/wiki/Serialization).
/// 
/// Functions can easily be implemented using macros of the same name (see example below).
/// 
/// # Example(s)
/// ```
/// // Import trait Tampon, macro bytes_size, deserialize and serialize
/// use tampon::{ Tampon, bytes_size, deserialize, serialize };
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
///     fn serialize(&self, buffer : &mut [u8]) -> usize {
///         // Use serialize! macro to serialize object, get the size with optional parameter and return it
///         serialize!(buffer, bytes_copied, (self.f1):u8, (self.f2):u32, (self.f3):f64, [self.v1]:u8, [self.v2]:f64);
///         bytes_copied
///     }
///
///     fn deserialize(buffer : &[u8]) -> (TamponExample, usize) {
///         // Use deserialize! macro to deserialize data, get the size with optional parameter
///         deserialize!(buffer, bytes_read, (f1):u8, (f2):u32, (f3):f64, [v1]:u8, [v2]:f64);
///         // From buffer must return a pair of object + bytes read
///         (TamponExample {f1,f2,f3,v1,v2 }, bytes_read)
///     }
/// }
/// ```
pub trait Tampon {
    /// Size of the trait implementation in [`bytes`](https://en.wikipedia.org/wiki/Byte).
    /// 
    /// Use macro [`bytes_size!`] to easily return the size in bytes.
    /// 
    /// # Example(s)
    /// ```ignore
    /// fn bytes_size(&self) -> usize {
    ///     bytes_size!((self.f1):u8, (self.f1):u32, (self.f3):f64, [self.v1]:u8, [self.v2]:f64)
    /// }
    /// ```
    fn bytes_size(&self) -> usize;

    /// Serialize object variable into buffer. 
    /// 
    /// Use macro [`serialize!`] to easily serialize and get size in bytes.
    /// 
    /// # Argument(s)
    /// * `buffer` - Mutable buffer slice reference to serialize into. 
    /// 
    /// # Example(s)
    /// ```ignore
    /// fn serialize(&self, buffer : &mut [u8]) -> usize {
    ///     serialize!(buffer, bytes_copied, (self.f1):u8, (self.f2):u32, (self.f3):f64, [self.v1]:u8, [self.v2]:f64);
    ///     bytes_copied
    /// }
    /// ```
    /// 
    /// # Return
    /// Bytes count written into buffer.
    fn serialize(&self, buffer : &mut [u8]) -> usize;

    /// Deserialize a new variable instance from buffer and return it with bytes read.
    /// 
    /// Use macro [`deserialize!`] to easily deserialize and get size in bytes.
    /// # Argument(s)
    /// * `buffer` - Non-mutable buffer slice reference to deserialize from. 
    /// 
    /// # Example(s)
    /// ```ignore
    /// fn deserialize(buffer : &[u8]) -> (TamponExample, usize) {
    ///     deserialize!(buffer, bytes_read, (f1):u8, (f2):u32, (f3):f64, [v1]:u8, [v2]:f64);
    ///     (TamponExample{f1,f2,f3,v1,v2}, bytes_read)
    /// }
    /// ```
    /// 
    /// # Return
    /// Tuple of new object and bytes read from buffer.
    fn deserialize(buffer : &[u8]) -> (Self, usize) where Self: Sized;
}