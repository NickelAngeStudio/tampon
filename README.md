# tampon[^1]
Specialized crate that contains [`SAFE`](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html) Rust [`functions`](https://doc.rust-lang.org/rust-by-example/fn.html), [`macros`](https://doc.rust-lang.org/rust-by-example/macros.html) and [`trait`](https://doc.rust-lang.org/rust-by-example/trait.html) to [`serialize / deserialize`](https://en.wikipedia.org/wiki/Serialization) data structure and/or object in packed [`bytes`](https://en.wikipedia.org/wiki/Byte).

### Version

#### 1.1.4.
- All macros works without tokens.

#### 1.1.3.  
- Updated [`deserialize_size!(...)`] to include optional max_size to prevent exploit.
- Updated [`deserialize_size!(...)`] and [`deserialize!(...)`] to accept no tokens.

#### 1.1.0. 
- Refactored faster macro code.
- Remove Tampon`<T>` from Trait implementation.
- Added function [`deserialize_size()`](https://docs.rs/tampon/latest/tampon/trait.Tampon.html#tymethod.deserialize_size) to trait Tampon.
- Added [`deserialize_size!(...)`](https://docs.rs/tampon/latest/tampon/macro.deserialize_size.html) to get the real size of a buffer.
- Added optional argument `optional_len_type` for maximum array size.
- Updated rand crate.

#### 1.0.0. 
- Base version

#### Macros
1. [`buffer!(...) -> Vec<u8>`](https://docs.rs/tampon/latest/tampon/macro.buffer.html) - Variadic macro used to create a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) and [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.buffer.html#compatible-variabless).
2. [`serialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.serialize.html) - Variadic macro used to [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.serialize.html#compatible-variabless) into a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer).
3. [`deserialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.deserialize.html) - Variadic macro used to [`deserialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.deserialize.html#compatible-variabless) from a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer).
4. [`bytes_size!(...) -> usize`](https://docs.rs/tampon/latest/tampon/macro.bytes_size.html) - Variadic macro used to get the size in [`bytes`](https://en.wikipedia.org/wiki/Byte) of [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.bytes_size.html#compatible-variabless) to [`serialize`](https://en.wikipedia.org/wiki/Serialization).
5. [`deserialize_size!(...)`](https://docs.rs/tampon/latest/tampon/macro.deserialize_size.html) Variadic macro used to get the size in [`bytes`](https://en.wikipedia.org/wiki/Byte) of [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.bytes_size.html#compatible-variabless) from buffer that return an error if buffer is too small.

#### Functions
1. [`generate_buffer(...) -> Vec<u8>`](https://docs.rs/tampon/latest/tampon/fn.generate_buffer.html) - Generate a random buffer with specific size and [`charset`](https://docs.rs/tampon/latest/tampon/buffer_generator_charset/index.html).
2. [`wipe_buffer(...)`](https://docs.rs/tampon/latest/tampon/fn.wipe_buffer.html) - Wipe a sensible buffer to prevent [`cold boot attack`](https://en.wikipedia.org/wiki/Cold_boot_attack) for greater security.
3. [`compare_buffer(...)`](https://docs.rs/tampon/latest/tampon/fn.compare_buffers.html) - Compare 2 buffers and return the [`absolute difference`](https://en.wikipedia.org/wiki/Absolute_difference).

#### Trait
1. [`Tampon`](https://docs.rs/tampon/latest/tampon/trait.Tampon.html) - Trait used to [`serialize / deserialize`](https://en.wikipedia.org/wiki/Serialization) object.

[^1]: [`Tampon`](https://www.google.com/search?q=memory+buffer+in+french) means `buffer` in french.
