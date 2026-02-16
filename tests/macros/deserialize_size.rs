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


//! Contains deserialize_size! validations tests
//! 
//! # Tests
//! V1 : deserialize_size returns Ok(size) for bool
//! V2 : deserialize_size returns Ok(size) for numeric
//! V3 : deserialize_size returns Ok(size) for string
//! V4 : deserialize_size returns Ok(size) for tampon
//! V5 : deserialize_size returns Ok(size) for all types
//! V6 : deserialize_size `optional_len_type` for slices
//! V7 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer bool
//! V8 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer numeric
//! V9 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer string
//! V10 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer tampon
//! V11 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer all types
//! V12 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for bool
//! V13 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for numeric
//! V14 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for string
//! V15 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for tampon
//! V16 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for all types
//! V17 : deserialize_size returns Ok(size) for size == optional max_size
//! V18 : deserialize_size returns Ok(size) for size < optional max_size
//! V19 : deserialize_size returns Ok(size) for optional max_size == 0

use core::panic;
use tampon::{Tampon, TamponError, serialize};

use tampon::{buffer, deserialize_size};

use crate::data::OLT_SIZE_DIFF;
use crate::{boolean_slice, numeric_slice, string_slice, tampon_slice};
use crate::{boolean_var, data::STRINGS, implementation::{TamponS1, TamponS2}, numeric_var, string_var, tampon_var};

#[test]
// V1 : deserialize_size returns Ok(size) for bool
fn deserialize_size_returns_ok_bool() {
    let mut bytes_size = 0;
    boolean_var!(bytes_size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    let buffer = buffer!((_b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9):bool);

    match deserialize_size!(buffer, (_b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9):bool) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_bool should not return Err!"),
    }

    let mut bytes_size = 0;
    boolean_slice!(bytes_size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    let buffer = buffer!([bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool);

    match deserialize_size!(buffer, [bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_slices_bool should not return Err!"),
    }

}

#[test]
// V2 : deserialize_size returns Ok(size) for numeric
fn deserialize_size_returns_ok_numeric() {

    let mut bytes_size = 0;
    numeric_var!(bytes_size, n0:u8, n1:u16, n2:u32, n3:u64, n4:u128, n5:f32, n6:f64, n7:i8, n8:i16, n9:i32, n10:i64, n11:i128);
    let buffer = buffer!((n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128);

    match deserialize_size!(buffer, (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_numeric should not return Err!"),
    }

    let mut bytes_size = 0;
    numeric_slice!(bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    let buffer = buffer!([ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128);

    match deserialize_size!(buffer, [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_slices_numeric should not return Err!"),
    }
}

#[test]
// V3 : deserialize_size returns Ok(size) for string
fn deserialize_size_returns_ok_string() {

    let mut bytes_size = 0;
    string_var!(bytes_size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);
    let buffer = buffer!((s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String);

    match deserialize_size!(buffer, (s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_string should not return Err!"),
    }

    let mut bytes_size = 0;
    string_slice!(bytes_size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);
    let buffer = buffer!([ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String);

    match deserialize_size!(buffer, [ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_slices_string should not return Err!"),
    }

}

#[test]
// V4 : deserialize_size returns Ok(size) for Tampon
fn deserialize_size_returns_ok_tampon() {

    let mut bytes_size = 0;
    tampon_var!(bytes_size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);
    let buffer = buffer!((t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2);

    match deserialize_size!(buffer, (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_tampon should not return Err!"),
    }

    let mut bytes_size = 0;
    tampon_slice!(bytes_size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);

    let buffer = buffer!([ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2);

    match deserialize_size!(buffer, [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_slices_tampon should not return Err!"),
    }
}

#[test]
// V5 : deserialize_size returns Ok(size) for all type mixed
fn deserialize_size_returns_ok_mixed() {

    let mut bytes_size = 0;
    boolean_var!(bytes_size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    boolean_slice!(bytes_size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    numeric_var!(bytes_size, n0:u8, n1:u16, n2:u32, n3:u64, n4:u128, n5:f32, n6:f64, n7:i8, n8:i16, n9:i32, n10:i64, n11:i128);
    numeric_slice!(bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    string_var!(bytes_size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);
    string_slice!(bytes_size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);
    tampon_var!(bytes_size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);
    tampon_slice!(bytes_size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);
    let buffer = buffer!(
        (_b0, _b1, _b2):bool, 
        [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1,
        [ss0,ss1]:String, [ss2,ss3,ss4]:String,
        (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128,
        [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool,
        [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
        (s0,s1):String, (s2,s3,s4):String, (s5):String,
        (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1,
        [bs0,bs1]:bool, [bs2]:bool,
        [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128,
        (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2,
        [ss5]:String, [ss6,ss7,ss8,ss9]:String,
        (n5):f32, (n6):f64,
        (s6,s7,s8,s9):String,
        [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2,
        (_b3, _b4, _b5, _b6, _b7, _b8, _b9):bool,
        (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128);

    match deserialize_size!(buffer, 
        (_b0, _b1, _b2):bool, 
        [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1,
        [ss0,ss1]:String, [ss2,ss3,ss4]:String,
        (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128,
        [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool,
        [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
        (s0,s1):String, (s2,s3,s4):String, (s5):String,
        (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1,
        [bs0,bs1]:bool, [bs2]:bool,
        [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128,
        (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2,
        [ss5]:String, [ss6,ss7,ss8,ss9]:String,
        (n5):f32, (n6):f64,
        (s6,s7,s8,s9):String,
        [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2,
        (_b3, _b4, _b5, _b6, _b7, _b8, _b9):bool,
        (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128) {
        Ok(size) => assert_eq!(bytes_size, size, "deserialize_size! wrong size given!"),
        Err(_) => panic!("Test deserialize_size_returns_ok_mixed should not return Err!"),
    }

}

#[test]
// V6 : deserialize_size `optional_len_type` for slices
fn deserialize_size_optional_len_type() {
    
    // Create variables and get their size
    let mut var_size = 0;
    boolean_slice!(var_size, 0, to_bs0, to_bs1, to_bs2, to_bs3, to_bs4, to_bs5, to_bs6, to_bs7, to_bs8, to_bs9);

    // Create buffer with serialize! and different len
    let mut buffer:Vec<u8> = vec![0;var_size*2];
    serialize!(buffer, to_size, [u8 : to_bs0,to_bs1]:bool, [u16 : to_bs2,to_bs3]:bool, [u32 : to_bs4]:bool, [u64 : to_bs5]:bool, [u128 : to_bs6,to_bs7]:bool, 
        [to_bs8,to_bs9]:bool);


    // Get back data with deserialize!
    match deserialize_size!(buffer, [u8 : from_bs0,from_bs1]:bool, [u16 : from_bs2,from_bs3]:bool, [u32 : from_bs4]:bool, [u64 : from_bs5]:bool, [u128 : from_bs6,from_bs7]:bool, 
        [from_bs8,from_bs9]:bool) {
            Ok(from_size) => assert!(var_size + OLT_SIZE_DIFF == to_size && to_size == from_size),
            Err(_) => panic!("Test deserialize_size_returns_ok_mixed should not return Err!"),
    }

}

#[test]
// V7 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer bool
fn deserialize_size_returns_err_bool() {
    let mut _bytes_size = 0;
    boolean_var!(_bytes_size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    let buffer = buffer!((_b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9):bool);

    match deserialize_size!(buffer[0..buffer.len()/2], (_b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9):bool) {
        Ok(_) => panic!("Test deserialize_size_returns_err_bool should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }

    let mut _bytes_size = 0;
    boolean_slice!(_bytes_size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    let buffer = buffer!([bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool);

    match deserialize_size!(buffer[0..buffer.len()-1], [bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool) {
        Ok(_) => panic!("Test deserialize_size_returns_err_bool should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }

}

#[test]
// V8 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer numeric
fn deserialize_size_returns_err_numeric() {
     let mut _bytes_size = 0;
    numeric_var!(_bytes_size, n0:u8, n1:u16, n2:u32, n3:u64, n4:u128, n5:f32, n6:f64, n7:i8, n8:i16, n9:i32, n10:i64, n11:i128);
    let buffer = buffer!((n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128);

    match deserialize_size!(buffer[0..buffer.len()/2], (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128) {
        Ok(_) => panic!("Test deserialize_size_returns_err_numeric should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }

    let mut _bytes_size = 0;
    numeric_slice!(_bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    let buffer = buffer!([ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128);

    match deserialize_size!(buffer[0..buffer.len()-1], [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128) {
        Ok(_) => panic!("Test deserialize_size_returns_err_numeric should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }
}

#[test]
// V9 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer string
fn deserialize_size_returns_err_string() {
    let mut _bytes_size = 0;
    string_var!(_bytes_size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);
    let buffer = buffer!((s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String);

    match deserialize_size!(buffer[0..buffer.len()/2], (s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String) {
        Ok(_) => panic!("Test deserialize_size_returns_err_string should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }

    let mut _bytes_size = 0;
    string_slice!(_bytes_size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);
    let buffer = buffer!([ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String);

    match deserialize_size!(buffer[0..buffer.len()-1], [ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String) {
        Ok(_) => panic!("Test deserialize_size_returns_err_string should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }
}

#[test]
// V10 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer tampon
fn deserialize_size_returns_err_tampon() {
    let mut _bytes_size = 0;
    tampon_var!(_bytes_size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);
    let buffer = buffer!((t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2);

    match deserialize_size!(buffer[0..buffer.len()/2], (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2) {
        Ok(_) => panic!("Test deserialize_size_returns_err_tampon should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }

    let mut _bytes_size = 0;
    tampon_slice!(_bytes_size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);

    let buffer = buffer!([ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2);

    match deserialize_size!(buffer[0..buffer.len()-1], [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2) {
        Ok(_) => panic!("Test deserialize_size_returns_err_tampon should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
    }
}

#[test]
// V11 : deserialize_size returns Err(DeserializeSizeBufferIncomplete) for incomplete buffer all types
 fn deserialize_size_returns_err_mixed() {
    
    let mut _bytes_size = 0;
    boolean_var!(_bytes_size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    boolean_slice!(_bytes_size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    numeric_var!(_bytes_size, n0:u8, n1:u16, n2:u32, n3:u64, n4:u128, n5:f32, n6:f64, n7:i8, n8:i16, n9:i32, n10:i64, n11:i128);
    numeric_slice!(_bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    string_var!(_bytes_size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);
    string_slice!(_bytes_size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);
    tampon_var!(_bytes_size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);
    tampon_slice!(_bytes_size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);
    let buffer = buffer!(
        (_b0, _b1, _b2):bool, 
        [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1,
        [ss0,ss1]:String, [ss2,ss3,ss4]:String,
        (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128,
        [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool,
        [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
        (s0,s1):String, (s2,s3,s4):String, (s5):String,
        (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1,
        [bs0,bs1]:bool, [bs2]:bool,
        [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128,
        (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2,
        [ss5]:String, [ss6,ss7,ss8,ss9]:String,
        (n5):f32, (n6):f64,
        (s6,s7,s8,s9):String,
        [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2,
        (_b3, _b4, _b5, _b6, _b7, _b8, _b9):bool,
        (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128);

    match deserialize_size!(buffer[0..buffer.len()-1], 
        (_b0, _b1, _b2):bool, 
        [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1,
        [ss0,ss1]:String, [ss2,ss3,ss4]:String,
        (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128,
        [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool,
        [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
        (s0,s1):String, (s2,s3,s4):String, (s5):String,
        (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1,
        [bs0,bs1]:bool, [bs2]:bool,
        [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128,
        (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2,
        [ss5]:String, [ss6,ss7,ss8,ss9]:String,
        (n5):f32, (n6):f64,
        (s6,s7,s8,s9):String,
        [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2,
        (_b3, _b4, _b5, _b6, _b7, _b8, _b9):bool,
        (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128) {
            Ok(_) => panic!("Test deserialize_size_returns_err_mixed should return err!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeBufferIncomplete),
        }
}


#[test]
// V12 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size
 fn deserialize_size_returns_err_greater_max_bool() {

    let mut _bytes_size = 0;
    boolean_var!(_bytes_size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    let buffer = buffer!((_b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9):bool);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;

    match deserialize_size!(buffer, _bytes_size, (_b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9):bool) {
        Ok(_) => panic!("0. deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }

    let mut _bytes_size = 0;
    boolean_slice!(_bytes_size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    let buffer = buffer!([bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool);

    // Reduce _bytes_size by 1 to make max lower than size
    _bytes_size -= 1;

    match deserialize_size!(buffer, _bytes_size, [bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool) {
        Ok(_) => panic!("1. deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }

 }

 #[test]
// V13 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for numeric
fn deserialize_size_returns_err_greater_max_numeric() {
    
     let mut _bytes_size = 0;
    numeric_var!(_bytes_size, n0:u8, n1:u16, n2:u32, n3:u64, n4:u128, n5:f32, n6:f64, n7:i8, n8:i16, n9:i32, n10:i64, n11:i128);
    let buffer = buffer!((n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;

    match deserialize_size!(buffer, _bytes_size, (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128) {
        Ok(_) => panic!("deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }

    let mut _bytes_size = 0;
    numeric_slice!(_bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    let buffer = buffer!([ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;

    match deserialize_size!(buffer, _bytes_size, [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128) {
        Ok(_) => panic!("deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }
     
}

#[test]
// V14 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for string
fn deserialize_size_returns_err_greater_max_string() {
    
    let mut _bytes_size = 0;
    string_var!(_bytes_size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);
    let buffer = buffer!((s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;

    match deserialize_size!(buffer, _bytes_size, (s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String) {
        Ok(_) => panic!("deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }

    let mut _bytes_size = 0;
    string_slice!(_bytes_size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);
    let buffer = buffer!([ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;


    match deserialize_size!(buffer, _bytes_size, [ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String) {
        Ok(_) => panic!("deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }
    
}


#[test]
// V15 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for tampon
fn deserialize_size_returns_err_greater_max_tampon() {
    
    let mut _bytes_size = 0;
    tampon_var!(_bytes_size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);
    let buffer = buffer!((t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;


    match deserialize_size!(buffer, _bytes_size, (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2) {
        Ok(_) => panic!("deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }

    let mut _bytes_size = 0;
    tampon_slice!(_bytes_size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);

    let buffer = buffer!([ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;

    match deserialize_size!(buffer, _bytes_size, [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2) {
        Ok(_) => panic!("deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }
    
}

#[test]
// V16 : deserialize_size returns Err(DeserializeSizeGreaterThanMax) for size > optional max_size for all types
fn deserialize_size_returns_err_greater_max_mixed() {
    
    let mut _bytes_size = 0;
    boolean_var!(_bytes_size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    boolean_slice!(_bytes_size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    numeric_var!(_bytes_size, n0:u8, n1:u16, n2:u32, n3:u64, n4:u128, n5:f32, n6:f64, n7:i8, n8:i16, n9:i32, n10:i64, n11:i128);
    numeric_slice!(_bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    string_var!(_bytes_size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);
    string_slice!(_bytes_size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);
    tampon_var!(_bytes_size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);
    tampon_slice!(_bytes_size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);
    let buffer = buffer!(
        (_b0, _b1, _b2):bool, 
        [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1,
        [ss0,ss1]:String, [ss2,ss3,ss4]:String,
        (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128,
        [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool,
        [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
        (s0,s1):String, (s2,s3,s4):String, (s5):String,
        (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1,
        [bs0,bs1]:bool, [bs2]:bool,
        [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128,
        (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2,
        [ss5]:String, [ss6,ss7,ss8,ss9]:String,
        (n5):f32, (n6):f64,
        (s6,s7,s8,s9):String,
        [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2,
        (_b3, _b4, _b5, _b6, _b7, _b8, _b9):bool,
        (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128);

    // Reduce bytes_size by 1 to make max lower than size
    _bytes_size -= 1;

    match deserialize_size!(buffer, _bytes_size, 
        (_b0, _b1, _b2):bool, 
        [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1,
        [ss0,ss1]:String, [ss2,ss3,ss4]:String,
        (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128,
        [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool,
        [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
        (s0,s1):String, (s2,s3,s4):String, (s5):String,
        (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1,
        [bs0,bs1]:bool, [bs2]:bool,
        [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128,
        (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2,
        [ss5]:String, [ss6,ss7,ss8,ss9]:String,
        (n5):f32, (n6):f64,
        (s6,s7,s8,s9):String,
        [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2,
        (_b3, _b4, _b5, _b6, _b7, _b8, _b9):bool,
        (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128) {
        Ok(_) => panic!("deserialize_size! should returns Err(DeserializeSizeGreaterThanMax)!"),
        Err(err) => assert_eq!(err, TamponError::DeserializeSizeGreaterThanMax),
    }
     
}

#[test]
// V17 : deserialize_size returns Ok(size) for size == optional max_size
 fn deserialize_size_equal_max() {
    let mut _bytes_size = 0;
    numeric_slice!(_bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    let buffer = buffer!([ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128);

    match deserialize_size!(buffer, _bytes_size, [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128) {
        Ok(size) => assert!(size == _bytes_size),
        Err(_) => panic!("deserialize_size! should returns Ok(size)!"),
    }
 }

#[test]
// V18 : deserialize_size returns Ok(size) for size < optional max_size
 fn deserialize_size_less_than_max() {

    let mut _bytes_size = 0;
    numeric_slice!(_bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    let buffer = buffer!([ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128);

    // Increase bytes_size by 1 to make bigger than size
    _bytes_size += 1;

    match deserialize_size!(buffer, _bytes_size, [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128) {
        Ok(size) => assert!(size < _bytes_size),
        Err(_) => panic!("deserialize_size! should returns Ok(size)!"),
    }

 }

 #[test]
 // V19 : deserialize_size returns Ok(size) for optional max_size == 0
fn deserialize_size_zero_max() {


    let mut _bytes_size = 0;
    numeric_slice!(_bytes_size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    let buffer = buffer!([ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128);

    match deserialize_size!(buffer, 0, [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128) {
        Ok(size) => assert!(size == _bytes_size),
        Err(_) => panic!("deserialize_size! should returns Ok(size)!"),
    }


}