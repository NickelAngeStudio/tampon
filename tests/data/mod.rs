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

use crate::implementation::{ TamponS1, TamponS2 } ;

 // Size of slices
static SLICESIZE: usize = 255;

// Size of diff for optional_len_type
pub const OLT_SIZE_DIFF : usize = size_of::<u8>() * 2 + size_of::<u16>() * 2 + size_of::<u32>() + size_of::<u64>() + size_of::<u128>() * 2 - size_of::<u32>() * 8;

// String constants
pub const STRINGS: &'static [&'static str] = &["I saw your text.", "Écrits avec des charactères spéciaux tel que é ç à î ì ï.",
    "",     // Empty string
    "तुजो मजकूर पळयलो", // Konkani
    "Би таны бичвэрийг харсан", // Mongolian
    "Nobis tamquam probatus ad est, nam ex singulis volutpat. Pro et dicta dictas iriure, ius tibique patrioque ea. Quo meis aeque commune ea, ius ea case vocibus iracundia. Esse consul vis at, dolore dissentias vel ei. Qui tation civibus moderatius et, no mei suas harum conclusionemque. Eum et nonumy fastidii detracto, usu magna referrentur ea.",
    "我看到了你的文字", // Chinese simplified
    "رأيت النص الخاص بك", // Arabic
    "私はあなたのテキストを見ました", // Japanese
    "Я бачив ваш текст"]; // Ukrainian


// Create boolean variables and calculate size
#[macro_export]
macro_rules! boolean_var {
    ($size:expr, $name:ident) => {
        let $name:bool = $size % 2 == 0;
        $size += 1;
    };
    ($size:expr, $name:ident, $($tail:tt)*) => {
        let $name:bool = $size % 2 == 0;
        $size += 1;
        boolean_var!($size, $($tail)*);
    };
}


// Create boolean slices and calculate size
#[macro_export]
macro_rules! boolean_slice {
    ($size:expr, $index:expr, $name:ident) => {
        let mut $name:Vec<bool> = Vec::new();
        for i in 0..($index + 1) * 100{
            $name.push(i % 2 == 0);
        }
        $size += size_of::<u32>() + $name.len();
    };
    ($size:expr, $index:expr, $name:ident, $($tail:tt)*) => {
        let mut $name:Vec<bool> = Vec::new();
        for i in 0..($index + 1) * 100{
            $name.push(i % 2 == 0);
        }
        $size += size_of::<u32>() + $name.len();
        boolean_slice!($size, $index + 1, $($tail)*);
    };
}


// Create numeric variables and calculate size
#[macro_export]
macro_rules! numeric_var {
    ($size:expr, $name:ident:$type:ty) => {
        let $name:$type = <$type>::MAX;
        $size += core::mem::size_of::<$type>();
    };
    ($size:expr, $name:ident:$type:ty, $($tail:tt)*) => {
        let $name:$type = <$type>::MAX;
        $size += core::mem::size_of::<$type>();
        numeric_var!($size, $($tail)*);
    };
}


// Create numeric slices and calculate size
#[macro_export]
macro_rules! numeric_slice {
    ($size:expr, $index:expr, $name:ident:$type:ty) => {
        let $name:Vec<$type> = vec![<$type>::MAX; ($index + 1) * 100];
        $size += size_of::<u32>() + core::mem::size_of::<$type>() *  ($index + 1) * 100;
    };
    ($size:expr, $index:expr, $name:ident:$type:ty, $($tail:tt)*) => {
        let $name:Vec<$type> = vec![<$type>::MAX;   ($index + 1) * 100];
        $size += size_of::<u32>() + core::mem::size_of::<$type>() *  ($index + 1) * 100;
        numeric_slice!($size, $index + 1, $($tail)*);
    };
}


// Create string variables and calculate size
#[macro_export]
macro_rules! string_var {
    ($size:expr, $strings:expr, $index:expr, $name:ident) => {
        let $name:String = String::from($strings[$index]);
        $size += size_of::<u32>()+$name.len();
    };
    ($size:expr, $strings:expr, $index:expr, $name:ident, $($tail:tt)*) => {
        let $name:String = String::from($strings[$index]);
        $size += size_of::<u32>()+$name.len();
        string_var!($size, $strings, $index + 1, $($tail)*);
    };
}

// Create string slices and calculate size
#[macro_export]
macro_rules! string_slice {
    ($size:expr, $strings:expr, $index:expr, $name:ident) => {
        let mut $name:Vec<String> = Vec::new();
        for elem in $strings {
            $name.push(String::from(*elem));
            $size += size_of::<u32>()+elem.len();
        }
        $size += size_of::<u32>();
    };
    ($size:expr, $strings:expr, $index:expr, $name:ident, $($tail:tt)*) => {
        let mut $name:Vec<String> = Vec::new();
        for elem in $strings {
            $name.push(String::from(*elem));
            $size += size_of::<u32>()+elem.len();
        }
        $size += size_of::<u32>();

        string_slice!($size, $strings, $index + 1, $($tail)*);
    };
}


// Create tampon variables and calculate size
#[macro_export]
macro_rules! tampon_var {
    ($size:expr, $name:ident:TamponS1) => {
        let $name:TamponS1 = TamponS1::new(u8::MAX, $size as u32, $size as f64, $size % 255);
        $size += $name.bytes_size();
    };
    ($size:expr, $name:ident:TamponS1, $($tail:tt)*) => {
        let $name:TamponS1 = TamponS1::new(u8::MAX, $size as u32, $size as f64, $size % 255);
        $size += $name.bytes_size();
        tampon_var!($size, $($tail)*);
    };
    ($size:expr, $name:ident:TamponS2) => {
        let $name:TamponS2 = TamponS2::new(u8::MAX, $size as i128);
        $size += $name.bytes_size();
    };
    ($size:expr, $name:ident:TamponS2, $($tail:tt)*) => {
        let $name:TamponS2 = TamponS2::new(u8::MAX, $size as i128);
        $size += $name.bytes_size();
        tampon_var!($size, $($tail)*);
    };
}


// Create tampon slices and calculate size
#[macro_export]
macro_rules! tampon_slice {
    ($size:expr, $index:expr, $name:ident:TamponS1) => {
        let mut $name:Vec<TamponS1> = Vec::new();
        for _ in 0..(($size + 1) * 2) % 255 {
            let t = TamponS1::new(u8::MAX, $size as u32, $size as f64, $size % 255);
            $size += t.bytes_size();
            $name.push(t);
        }
        $size += size_of::<u32>();
    };
    ($size:expr, $index:expr, $name:ident:TamponS1, $($tail:tt)*) => {
        let mut $name:Vec<TamponS1> = Vec::new();
        for _ in 0..(($size + 1) * 2) % 255 {
            let t = TamponS1::new(u8::MAX, $size as u32, $size as f64, $size % 255);
            $size += t.bytes_size();
            $name.push(t);
        }
        $size += size_of::<u32>();
        tampon_slice!($size, $index + 1, $($tail)*);
    };

    ($size:expr, $index:expr, $name:ident:TamponS2) => {
        let mut $name:Vec<TamponS2> = Vec::new();
        for _ in 0..(($size + 1) * 2) % 255 {
            let t = TamponS2::new(u8::MAX, $size as i128);
            $size += t.bytes_size();
            $name.push(t);
        }
        $size += size_of::<u32>();
    };
    ($size:expr, $index:expr, $name:ident:TamponS2, $($tail:tt)*) => {
        let mut $name:Vec<TamponS2> = Vec::new();
        for _ in 0..(($size + 1) * 2) % 255 {
            let t = TamponS2::new(u8::MAX, $size as i128);
            $size += t.bytes_size();
            $name.push(t);
        }
        $size += size_of::<u32>();
        tampon_slice!($size, $index + 1, $($tail)*);
    };
}


// Print macro test result and assert.
pub fn macro_test_validation(expected:usize, result:usize) -> bool {

    println!("Bytes size | Expected={}, Result={}, Diff={}", expected, result, if expected > result {
        expected - result
    } else {
        result - expected
    });

    expected == result
}

// To see if 2 vectors matches
// https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element
pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn do_vecs_eq_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a.eq(b)).count();
    matching == a.len() && matching == b.len()
}


pub fn vec_of_tampon1(size:usize) -> Vec<TamponS1>{

    let mut vects: Vec<TamponS1> = Vec::new();

    for i in 0..size {
        vects.push(TamponS1::new(i as u8, i as u32, i as f64, SLICESIZE));
    }

    vects
}

pub fn vec_of_tampon2(size:usize) -> Vec<TamponS2>{

    let mut vects: Vec<TamponS2> = Vec::new();

    for i in 0..size {
        vects.push(TamponS2::new(5, i as i128));
    }

    vects
}