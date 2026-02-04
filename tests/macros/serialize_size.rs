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

use tampon::{serialize_size, Tampon};
use crate::{data::{ macro_test_validation, STRINGS}, implementation::TamponS1, implementation::TamponS2, numeric_var, numeric_slice, string_var, string_slice, boolean_var, boolean_slice, tampon_var, tampon_slice};

#[test]
// Test bytes size of 1 bool
fn bytes_size_1_bool(){
    let mut size = 0;
    boolean_var!(size, _b0);
    assert!(macro_test_validation(size,
        serialize_size!((b0):bool)
    ));
}

#[test]
// Test bytes size of 2 bool
fn bytes_size_2_bools(){
    let mut size = 0;
    boolean_var!(size, _b0, _b1);
    assert!(macro_test_validation(size,
        serialize_size!((b0,b1):bool)
    ));
}

#[test]
// Test bytes size of 10 bool
fn bytes_size_10_bools(){
    let mut size = 0;
    boolean_var!(size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    assert!(macro_test_validation(size,
        serialize_size!((b0,b1):bool, (b2):bool, (b3,b4,b5,b6,b7):bool, (b8,b9):bool)
    ));
}

#[test]
// Test bytes size of 1 bool slice
fn bytes_size_1_bool_slice(){
    let mut size = 0;
    boolean_slice!(size, 0, bs0);
    assert!(macro_test_validation(size,
        serialize_size!([bs0]:bool)
    ));
}

#[test]
// Test bytes size of 2 bool slices
fn bytes_size_2_bool_slices(){
    let mut size = 0;
    boolean_slice!(size, 0, bs0, bs1);
    assert!(macro_test_validation(size,
        serialize_size!([bs0,bs1]:bool)
    ));
}

#[test]
// Test bytes size of 10 bool slices
fn bytes_size_10_bool_slices(){
    let mut size = 0;
    boolean_slice!(size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    assert!(macro_test_validation(size,
        serialize_size!([bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool)
    ));
}


#[test]
// Test bytes size 1 numeric type
fn bytes_size_1_numeric(){
    let mut size = 0;
    numeric_var!(size, _n0:i8);
    assert!(macro_test_validation(size,
        serialize_size!((n0):i8)
    ));
}

#[test]
// Test bytes size of 2 numeric type
fn bytes_size_2_numerics(){
    let mut size = 0;
    numeric_var!(size, _n0:f32, _n1:f32, _n2:i128);
    assert!(macro_test_validation(size,
        serialize_size!((n0, n1):f32, (n2):i128)
    ));
}

#[test]
// Test bytes size of ALL numeric type
fn bytes_size_all_numerics(){
    let mut size = 0;
    numeric_var!(size, _n0:u8, _n1:u16, _n2:u32, _n3:u64, _n4:u128, _n5:f32, _n6:f64,
        _n7:i8, _n8:i16, _n9:i32, _n10:i64, _n11:i128);
    assert!(macro_test_validation(size,
        serialize_size!((n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128)
    ));
}

#[test]
// Test bytes size of 1 empty slice of numeric. Will still be 4 bytes because need to keep 0 as length.
fn bytes_size_empty_numeric_slice(){
    let size = 4;
    let ns0:Vec<u8> = Vec::new();
    assert!(macro_test_validation(size,
        serialize_size!([ns0]:u8)
    ));
}

#[test]
// Test bytes size of 1 slice of numeric
fn bytes_size_1_numeric_slice(){
    let mut size = 0;
    numeric_slice!(size, 0, ns0:u8);

    assert!(macro_test_validation(size,
        serialize_size!([ns0]:u8)
    ));
}


#[test]
// Test bytes size of 2 slice of numeric
fn bytes_size_2_numeric_slice(){
    let mut size = 0;
    numeric_slice!(size, 0, ns0:f32, ns1:f32, ns2:i128);

    assert!(macro_test_validation(size,
        serialize_size!([ns0, ns1]:f32, [ns2]:i128)
    ));
}


#[test]
// Test bytes size of ALL slice of numeric
fn bytes_size_all_numeric_slice(){
    let mut size = 0;
    numeric_slice!(size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);

    assert!(macro_test_validation(size,
        serialize_size!([ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128)
    ));
}


#[test]
// Test bytes size 1 string
fn bytes_size_1_string(){
    let mut size = 0;
    string_var!(size, STRINGS, 0, s0);

    assert!(macro_test_validation(size,
        serialize_size!((s0):String)
    ));
}

#[test]
// Test bytes size 2 strings
fn bytes_size_2_string(){
    let mut size = 0;
    string_var!(size, STRINGS, 0, s0, s1);

    assert!(macro_test_validation(size,
        serialize_size!((s0,s1):String)
    ));
}

#[test]
// Test bytes size 10 strings
fn bytes_size_10_string(){
    let mut size = 0;
    string_var!(size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);

    assert!(macro_test_validation(size,
        serialize_size!((s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String)
    ));
}


#[test]
// Test bytes size 1 string slice
fn bytes_size_1_string_slice(){
    let mut size = 0;
    string_slice!(size, STRINGS, 0, ss0);

    assert!(macro_test_validation(size,
        serialize_size!([ss0]:String)
    ));
}

#[test]
// Test bytes size 2 strings slices
fn bytes_size_2_string_slice(){
    let mut size = 0;
    string_slice!(size, STRINGS, 0, ss0, ss1);

    assert!(macro_test_validation(size,
        serialize_size!([ss0,ss1]:String)
    ));
}

#[test]
// Test bytes size 10 strings slices
fn bytes_size_10_string_slice(){
    let mut size = 0;
    string_slice!(size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);

    assert!(macro_test_validation(size,
        serialize_size!([ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String)
    ));

}


#[test]
// Test that show that core::mem::size_of::<TamponS1>() != TamponS1::bytes_size()
fn bytes_size_struct_size_of_diff(){

    let mut size = 0;
    tampon_var!(size, t0:TamponS1);
    let sizeof_tampon_s1 = core::mem::size_of::<TamponS1>();

    println!("size_of={}, bytes_size={}",sizeof_tampon_s1, size);

    assert!(sizeof_tampon_s1 != size);

}


#[test]
// Test bytes size of 1 implementor of Tampon trait
fn bytes_size_1_tampon(){

    let mut size = 0;
    tampon_var!(size, t0:TamponS1);

    assert!(macro_test_validation(size,
        serialize_size!((t0):TamponS1)
    ));
}

#[test]
// Test bytes size of 2 implementor of Tampon trait
fn bytes_size_2_tampon(){

    let mut size = 0;
    tampon_var!(size, t0:TamponS1, t1:TamponS2);

    assert!(macro_test_validation(size,
        serialize_size!((t0):TamponS1,(t1):TamponS2)
    ));


}


#[test]
// Test bytes size of 10 implementor of Tampon trait
fn bytes_size_10_tampon(){

    let mut size = 0;
    tampon_var!(size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);

    assert!(macro_test_validation(size,
        serialize_size!((t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2)
    ));

}


#[test]
// Test bytes size of 1 slice of implementor of Tampon trait
fn bytes_size_1_tampon_slice(){

    let mut size = 0;
    tampon_slice!(size, 0, ts0:TamponS1);

    assert!(macro_test_validation(size,
        serialize_size!([ts0]:TamponS1)
    ));

}

#[test]
// Test bytes size of 2 slice of implementor of Tampon trait
fn bytes_size_2_tampon_slice(){

    let mut size = 0;
    tampon_slice!(size, 0, ts0:TamponS1, ts1:TamponS2);

    assert!(macro_test_validation(size,
        serialize_size!([ts0]:TamponS1, [ts1]:TamponS2)
    ));

}

#[test]
// Test bytes size of 10 slice of implementor of Tampon trait
fn bytes_size_10_tampon_slice(){
    

    let mut size = 0;
    tampon_slice!(size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);

    assert!(macro_test_validation(size,
        serialize_size!([ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2)
    ));

}

#[test]
// Test bytes size of 1b, 1n, 1s, 1t
fn bytes_size_1b_1n_1s_1t(){

    let mut size = 0;
    boolean_var!(size, _b0);
    numeric_var!(size, _n0:i8);
    string_var!(size, STRINGS, 0, s0);
    tampon_var!(size, t0:TamponS1);

    assert!(macro_test_validation(size,
        serialize_size!((b0):bool, (n0):i8, (s0):String, (t0):TamponS1)
    ));

}

#[test]
// Test bytes size of 1bs, 1ns, 1ss, 1ts
fn bytes_size_1bs_1ns_1ss_1ts(){

    let mut size = 0;
    boolean_slice!(size, 0, bs0);
    numeric_slice!(size, 0, ns0:i8);
    string_slice!(size, STRINGS, 0, ss0);
    tampon_slice!(size, 0, ts0:TamponS1);

    assert!(macro_test_validation(size,
        serialize_size!([bs0]:bool, [ns0]:i8, [ss0]:String, [ts0]:TamponS1)
    ));

}

#[test]
// Test bytes size of 1b, 1n, 1s, 1t, 1bs, 1ns, 1ss, 1ts
fn bytes_size_1n_1s_1t_1ns_1ss_1ts(){

    let mut size = 0;
    boolean_var!(size, _b0);
    numeric_var!(size, _n0:i8);
    string_var!(size, STRINGS, 0, s0);
    tampon_var!(size, t0:TamponS1);
    boolean_slice!(size, 0, bs0);
    numeric_slice!(size, 0, ns0:i8);
    string_slice!(size, STRINGS, 0, ss0);
    tampon_slice!(size, 0, ts0:TamponS1);

    assert!(macro_test_validation(size,
        serialize_size!((b0):bool, (n0):i8, (s0):String, (t0):TamponS1, [bs0]:bool, [ns0]:i8, [ss0]:String, [ts0]:TamponS1)
    ));

}

#[test]
// Test bytes size of 10b, ALLn, 10s, 10t, 10bs, ALLns, 10ss, 10ts
fn bytes_size_10n_10s_10t_10ns_10ss_10ts(){

    let mut size = 0;
    boolean_var!(size, _b0, _b1, _b2, _b3, _b4, _b5, _b6, _b7, _b8, _b9);
    boolean_slice!(size, 0, bs0, bs1, bs2, bs3, bs4, bs5, bs6, bs7, bs8, bs9);
    numeric_var!(size, _n0:u8, _n1:u16, _n2:u32, _n3:u64, _n4:u128, _n5:f32, _n6:f64,
        _n7:i8, _n8:i16, _n9:i32, _n10:i64, _n11:i128);
    numeric_slice!(size, 0, ns0:u8, ns1:u16, ns2:u32, ns3:u64, ns4:u128, ns5:f32, ns6:f64,
        ns7:i8, ns8:i16, ns9:i32, ns10:i64, ns11:i128);
    string_var!(size, STRINGS, 0, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9);
    string_slice!(size, STRINGS, 0, ss0, ss1, ss2, ss3, ss4, ss5, ss6, ss7, ss8, ss9);
    tampon_var!(size, t0:TamponS1, t1:TamponS2, t2:TamponS1, t3:TamponS1, t4:TamponS1, t5:TamponS2, 
        t6:TamponS1, t7:TamponS2, t8:TamponS1, t9:TamponS2);
    tampon_slice!(size, 0, ts0:TamponS1, ts1:TamponS2, ts2:TamponS1, ts3:TamponS1, ts4:TamponS1, ts5:TamponS2, 
        ts6:TamponS1, ts7:TamponS2, ts8:TamponS1, ts9:TamponS2);



    assert!(macro_test_validation(size,
        serialize_size!(
            (b0,b1):bool, (b2):bool, (b3,b4,b5,b6,b7):bool, (b8,b9):bool,
            [bs0,bs1]:bool, [bs2]:bool, [bs3,bs4,bs5,bs6,bs7]:bool, [bs8,bs9]:bool,
            (n0):u8, (n1):u16, (n2):u32, (n3):u64, (n4):u128, (n5):f32, (n6):f64,
            (n7):i8, (n8):i16, (n9):i32, (n10):i64, (n11):i128,
            [ns0]:u8, [ns1]:u16, [ns2]:u32, [ns3]:u64, [ns4]:u128, [ns5]:f32, [ns6]:f64,
            [ns7]:i8, [ns8]:i16, [ns9]:i32, [ns10]:i64, [ns11]:i128,
            (s0,s1):String, (s2,s3,s4):String, (s5):String, (s6,s7,s8,s9):String,
            [ss0,ss1]:String, [ss2,ss3,ss4]:String, [ss5]:String, [ss6,ss7,ss8,ss9]:String,
            (t0,t2):TamponS1,(t1,t5):TamponS2, (t3):TamponS1, (t4,t6,t8):TamponS1, (t7):TamponS2 ,(t9):TamponS2,
            [ts0,ts2]:TamponS1,[ts1,ts5]:TamponS2, [ts3]:TamponS1, [ts4,ts6,ts8]:TamponS1, [ts7]:TamponS2 ,[ts9]:TamponS2
        )
    ));
}
