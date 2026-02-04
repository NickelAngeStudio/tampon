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

 // Values into buffers
 static BUFFER_VALUE_MAX: u8 = 100;
 static BUFFER_VALUE_MIN: u8 = 5;

 // Size of buffer for tests
static BUFFER_SIZE_MAX: usize = 255;
static BUFFER_SIZE_MIN: usize = 100;

#[test]
// Compare identical buffers
fn compare_identical() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];

    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be equal and return 0
    assert!(diff==0);    
}

#[test]
// Compare identical buffers with no values
fn compare_identical_no_size() {
    let b1:Vec<u8> = Vec::new();
    let b2:Vec<u8> = Vec::new();
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be equal and return 0
    assert!(diff==0);    
}

#[test]
// Compare buffers with different values but same size
fn compare_different_value() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MIN;BUFFER_SIZE_MAX];
    
    // Both buffer should be bigger than 0.
    assert!(crate::compare_buffers(&b1,&b2)>0);    
}

#[test]
// Compare buffers with same value but different sizes
fn compare_different_size() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MIN];
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
   // Both buffer should be bigger than 0.
    assert!(diff==(BUFFER_SIZE_MAX - BUFFER_SIZE_MIN));    
}

#[test]
// Compare buffer vs an empty buffer
fn compare_different_size_no_size() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = Vec::new();
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be bigger than 0.
    assert!(diff==(BUFFER_SIZE_MAX - 0));    
}

// Compare a buffer that has different size and values
#[test]
fn compare_different_value_and_size() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MIN;BUFFER_SIZE_MIN];
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be bigger than 0.
    assert!(diff>0);    
}