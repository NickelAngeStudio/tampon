# tampon[^1]
Crate that contains [`SAFE`](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html) Rust [`functions`](https://doc.rust-lang.org/rust-by-example/fn.html), [`macro`](https://doc.rust-lang.org/rust-by-example/macros.html) and [`trait`](https://doc.rust-lang.org/rust-by-example/trait.html) to [`serialize / deserialize`](https://en.wikipedia.org/wiki/Serialization) data structure and/or object and generate [`buffer`](https://en.wikipedia.org/wiki/Data_buffer).

### Version

#### 1.1.0. 
- Refactored faster macro code.
- Remove Tampon<T> from Trait implementation.
- Added optional argument for maximum array size.
- Macro `bytes_size!` renamed to [`serialize_size!(...)`](https://docs.rs/tampon/latest/tampon/macro.serialize_size.html).
- Added 3 new macros [`try_serialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.try_serialize.html),  [`try_deserialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.try_deserialize.html) that returns a [Result](https://doc.rust-lang.org/std/result/) and [`deserialize_size!(...)`](https://docs.rs/tampon/latest/tampon/macro.deserialize_size.html).
- Updated rand crate.

#### 1.0.0. 
- Base version

#### Functions
1. [`generate_buffer(...) -> Vec<u8>`](https://docs.rs/tampon/latest/tampon/fn.generate_buffer.html) - Generate a random buffer with specific size and [`charset`](https://docs.rs/tampon/latest/tampon/buffer_generator_charset/index.html).
2. [`wipe_buffer(...)`](https://docs.rs/tampon/latest/tampon/fn.wipe_buffer.html) - Wipe a sensible buffer to prevent [`cold boot attack`](https://en.wikipedia.org/wiki/Cold_boot_attack) for greater security.
3. [`compare_buffer(...)`](https://docs.rs/tampon/latest/tampon/fn.compare_buffers.html) - Compare 2 buffers and return the [`absolute difference`](https://en.wikipedia.org/wiki/Absolute_difference).

#### Macros
1. [`buffer!(...) -> Vec<u8>`](https://docs.rs/tampon/latest/tampon/macro.buffer.html) - Variadic macro used to create a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) and [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.buffer.html#compatible-variabless).
3. [`serialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.serialize.html) - Variadic macro used to [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.serialize.html#compatible-variabless) into a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer).
4. [`deserialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.deserialize.html) - Variadic macro used to [`deserialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.deserialize.html#compatible-variabless) from a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer).
5. [`try_serialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.try_serialize.html) - Variadic macro used to [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.serialize.html#compatible-variabless) into a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) that returns a [Result](https://doc.rust-lang.org/std/result/) and can't panic.
6. [`try_deserialize!(...)`](https://docs.rs/tampon/latest/tampon/macro.try_deserialize.html) - Variadic macro used to [`deserialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.deserialize.html#compatible-variabless) from a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) that returns a [Result](https://doc.rust-lang.org/std/result/) and can't panic.
2. [`serialize_size!(...) -> usize`](https://docs.rs/tampon/latest/tampon/macro.serialize_size.html) - Variadic macro used to get the size in [`bytes`](https://en.wikipedia.org/wiki/Byte) of [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.bytes_size.html#compatible-variabless) to [`serialize`](https://en.wikipedia.org/wiki/Serialization).
7. [`deserialize_size!(...)`](https://docs.rs/tampon/latest/tampon/macro.deserialize_size.html) Variadic macro used to get the size in [`bytes`](https://en.wikipedia.org/wiki/Byte) of [`compatible variables`](https://docs.rs/tampon/latest/tampon/macro.bytes_size.html#compatible-variabless) from buffer that return an error if buffer is too small.

#### Trait
1. [`Tampon`](https://docs.rs/tampon/latest/tampon/trait.Tampon.html) - Trait used to [`serialize / deserialize`](https://en.wikipedia.org/wiki/Serialization) object.

[^1]: [`Tampon`](https://www.google.com/search?q=memory+buffer+in+french) means `buffer` in french.
