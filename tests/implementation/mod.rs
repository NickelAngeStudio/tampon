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

use std::vec;

use tampon::{TamponError, deserialize, deserialize_size};
pub use tampon::{Tampon, bytes_size, serialize};

use crate::data::{do_vecs_match, do_vecs_eq_match};


// Struct used to test Tampon traits in macros
 pub struct TamponS1 {
    _f1:u8,
    _f2:u32,
    _f3:f64,
    f4:TamponS2,
    v1:Vec<u8>,
    v2:Vec<f64>,
    v3:Vec<TamponS2>
 }

 impl TamponS1 {
    pub fn new(f1:u8, f2:u32, f3:f64, vsize:usize) -> TamponS1 {

        let f4 = TamponS2::new(0,1234567892);

        let v1:Vec<u8> = vec![f1;vsize]; 
        let v2:Vec<f64> = vec![f3;vsize];

        let mut v3:Vec<TamponS2> = Vec::new();

        for i in 0..f1 {
            v3.push(TamponS2::new(i, (i as i128 * i as i128) as i128));
        }

        TamponS1 {
            _f1: f1,_f2: f2,_f3: f3,f4,v1,v2,v3
        }
    }
 }


impl Tampon for TamponS1 {
    fn bytes_size(&self) -> usize {
        bytes_size!((self._f1):u8, (self._f2):u32, (self._f3):f64, (&self.f4):TamponS2, [self.v1]:u8, [self.v2]:f64, [self.v3]:TamponS2)
    }

    fn serialize(&self, buffer : &mut [u8]) -> usize {
        serialize!(buffer, size, (self._f1):u8, (self._f2):u32, (self._f3):f64, (self.f4):TamponS2, [self.v1]:u8, [self.v2]:f64, [self.v3]:TamponS2);
        size
    }

    fn deserialize(buffer : &[u8]) -> (Self, usize) {

        deserialize!(buffer, size, (_f1):u8, (_f2):u32, (_f3):f64, (f4):TamponS2, [v1]:u8, [v2]:f64, [v3]:TamponS2);
        (TamponS1 { _f1,_f2,_f3,f4,v1,v2,v3 }, size)       

    }
    
    fn deserialize_size(buffer : &[u8], max_size : usize) -> Result<usize, TamponError> {
        deserialize_size!(buffer, max_size, (_f1):u8, (_f2):u32, (_f3):f64, (f4):TamponS2, [v1]:u8, [v2]:f64, [v3]:TamponS2)
    }
}

impl PartialEq for TamponS1 {
    fn eq(&self, other: &Self) -> bool {
        self._f1 == other._f1 && self._f2 == other._f2 && self._f3 == other._f3 && self.f4.eq(&other.f4)
        && do_vecs_match(&self.v1,&other.v1) && do_vecs_match(&self.v2,&other.v2) && do_vecs_eq_match(&self.v3,&other.v3)
    }
}


 // Struct used as inner struct of test struct
 pub struct TamponS2 {
    _f1:u8,
    _f2:i128
 }

 impl TamponS2 {
    pub fn new(f1:u8, f2:i128) -> TamponS2 {
        TamponS2 {
            _f1: f1,_f2: f2
        }
    }
 }

 impl Tampon for TamponS2 {
    fn bytes_size(&self) -> usize {
        bytes_size!((self._f1):u8, (self._f2):i128)
    }

    fn serialize(&self, buffer : &mut [u8]) -> usize {
        serialize!(buffer, size, (self._f1):u8, (self._f2):i128);
        size
    }

    fn deserialize(buffer : &[u8]) -> (Self, usize) {
        deserialize!(buffer, size, (_f1):u8, (_f2):i128);
        (TamponS2 {_f1,_f2 }, size)
    }
        
    fn deserialize_size(buffer : &[u8], max_size : usize) -> Result<usize, TamponError> {
        deserialize_size!(buffer, max_size, (_f1):u8, (_f2):i128)
    }

        
}    


impl PartialEq for TamponS2 {
    fn eq(&self, other: &Self) -> bool {
        self._f1 == other._f1 && self._f2 == other._f2
    }
}