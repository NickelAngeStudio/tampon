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

/// ##### Compare 2 buffers and return the [`absolute difference`](https://en.wikipedia.org/wiki/Absolute_difference).
/// Compare 2 buffers and return the [`absolute difference`](https://en.wikipedia.org/wiki/Absolute_difference).
/// 
/// If both buffers are identical in size AND content, this function will return 0.
/// # Example(s)
/// ```
/// // Import tampon function
/// use tampon::compare_buffers;
/// 
/// let b1:Vec<u8> = vec![5;5];
/// let b2:Vec<u8> = vec![5;5];
/// 
/// // Both buffer should be equal and return 0
/// assert!(tampon::compare_buffers(&b1,&b2)==0);
/// 
/// let b3:Vec<u8> = vec![4;4];
/// 
/// // Buffers should be different
/// assert!(tampon::compare_buffers(&b1,&b3)>0)
/// ```
/// # Argument(s)
/// * `b1` - First `Vec<u8>` buffer reference to compare.
/// * `b2` - Second `Vec<u8>` buffer reference to compare.
/// # Return
/// Absolute difference between both buffers. Identical in size and content will return 0.
pub fn compare_buffers(b1 : &Vec<u8>,  b2 : &Vec<u8>) -> usize {
        
    // Difference is initialize with the absolute difference in length
    let mut _diff: usize = if b1.len() > b2.len() {
        b1.len() - b2.len()
    } else {
        b2.len() - b1.len()
    };

    // Size will be the lowest of both sizes
    let size = std::cmp::min(b1.len(), b2.len());

    // Calculate the difference in size and take the lowest size of both buffers.
    if b1.len() > b2.len() {
        _diff = b1.len() - b2.len();
    } else {
        _diff = b2.len() - b1.len();
    }
    
    // Calculate the absolute difference of b1, b2
    for i in 0..size {

        if b1[i] > b2[i] {
            _diff = _diff + (b1[i] - b2[i]) as usize;
        } else {
            _diff = _diff + (b2[i] - b1[i]) as usize;
        }
    }
     
    // Return the difference
    _diff
}