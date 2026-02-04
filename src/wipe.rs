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

 /// ##### Wipe a buffer, overwriting content with zeroes.
 /// Wipe a sensible buffer to prevent [`cold boot attack`](https://en.wikipedia.org/wiki/Cold_boot_attack) for greater security.
 /// 
 /// # Argument(s)
 /// * `buffer` - Mutable reference to vector of [`u8`] to wipe.
 /// 
 /// # Warning(s)
 /// <b>It goes without saying that it can't be reversed.</b>
 /// 
 /// # Example(s)
 /// ```
 /// // Import tampon crate.
 /// use tampon::wipe_buffer;
 /// 
 /// // Create a u8 array
 /// let mut buffer : &mut Vec<u8> = &mut vec![80, 76, 90, 87, 73, 80, 69, 77, 69];
 /// 
 /// // Print current buffer
 /// println!("Buffer = {:?}", buffer);
 /// 
 /// // Wipe buffer
 /// tampon::wipe_buffer(&mut buffer);
 /// 
 /// // Print wiped buffer
 /// println!("Buffer = {:?}", buffer);
 /// ```
 pub fn wipe_buffer(buffer : &mut Vec<u8>){

    for elem in buffer.iter_mut() {
        *elem = 0;
    }

 }