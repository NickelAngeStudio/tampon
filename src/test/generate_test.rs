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

use std::time::{Duration, Instant};
use rand::{Rng, prelude::ThreadRng};
use crate::{buffer_generator_charset, generate::buffer_generator_range};

// Size of buffer for tests (except Stress)
static BUFFER_SIZE: usize = 255;

// Stress test duration in seconds (10 mins)
static STRESS_DURATION: Duration = Duration::from_secs(600); 

// Max buffer stress size set to 4mb, which is the typical 3d model file. https://www.google.com/search?q=typical+3d+model+file+size
static STRESS_MAX_SIZE : usize = 4*1024*1024;


/******************
* GENERATE_BUFFER *
******************/
#[test]
#[should_panic]
// Generate a buffer with size 0
fn generate_buffer_size_0(){

    let mut rng = rand::rng();

    let buffer = crate::generate_buffer(&mut rng, 0, buffer_generator_charset::ALL);

    println!("Buffer = [{}]", String::from_utf8_lossy(&buffer));

}

#[test]
#[should_panic]
// Generate a buffer without charset
fn generate_buffer_no_charset(){

    let mut rng = rand::rng();

    let buffer = crate::generate_buffer(&mut rng, BUFFER_SIZE, 0);

    println!("Buffer = [{}]", String::from_utf8_lossy(&buffer));

}

#[test]
// Generate a buffer with each charset and test set validity
fn generate_and_test_characters_set(){

    let mut rng = rand::rng();

    test_character_set_validity(&mut rng, buffer_generator_charset::NUMBER, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::LOWER_CASE, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::UPPER_CASE, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::SYMBOL, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::UNREADABLE, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::ALL, BUFFER_SIZE, true);


}

#[test]
#[ignore]
// Stress test buffer for STRESS_DURATION. Can take long time.
fn generate_buffer_stress_test() {

    let mut rng = rand::rng();
    let mut stress_loop = 0;
    let started = Instant::now();

    while Instant::now() - started <= STRESS_DURATION {    
        stress_loop += 1;

        // Generate buffer size
        let size = rng.random_range(0..=STRESS_MAX_SIZE);
        
        // Generate buffer charset
        let charset = generate_charset(&mut rng);

        // Print loop # and variables
        println!("Stress #{} | Buffer size={}/{} | Charset={} | Remaining time : {:?}...", stress_loop, size, STRESS_MAX_SIZE,
            charset, STRESS_DURATION - (Instant::now() - started));

        // Test character set validity
        test_character_set_validity(&mut rng, charset, size, false);
    }
}


/***************************
 * FUNCTIONS USED IN TESTS *
 **************************/
/// Generate a valid random charset
fn generate_charset(rng : &mut ThreadRng) -> u8{

    let mut charset:u8 = 0;


    // While charset is invalid, generate a charset
    while charset == 0 {

        // Pick numbers for charset generation 
        let number = rng.random_range(0..=1);
        let lower_case = rng.random_range(0..=1);
        let upper_case = rng.random_range(0..=1);
        let symbol = rng.random_range(0..=1);
        let unreadable = rng.random_range(0..=1);

        // Lower chance to get all (1 out of 10)
        let all = rng.random_range(0..=10);

        if number == 1 {
            charset = charset | buffer_generator_charset::NUMBER;
        }

        if lower_case == 1 {
            charset = charset | buffer_generator_charset::LOWER_CASE;
        }

        if upper_case == 1 {
            charset = charset | buffer_generator_charset::UPPER_CASE;
        }

        if symbol == 1 {
            charset = charset | buffer_generator_charset::SYMBOL;
        }

        if unreadable == 1 {
            charset = charset | buffer_generator_charset::UNREADABLE;
        }

        if all == 1 {
            charset = charset | buffer_generator_charset::ALL;
        }
    }

    charset
}

/// Will test a specific charset and panic if invalid 
fn test_character_set_validity(rng : &mut ThreadRng, charset : u8, size: usize, print_buf : bool){

    let buffer = crate::generate_buffer(rng, size, charset);

    // Print buffer if print_buf
    if print_buf {
        println!("Buffer = {}", String::from_utf8_lossy(&buffer));
    }

    // Make sure that buffer contains desired charset
    assert!(buffer_charset_valid(&buffer, charset));

}

/// Verify that a buffer charset is in range
fn buffer_charset_valid(buffer: &Vec<u8>, charset : u8) -> bool {

    // Start with result as valid
    let mut result = true;

    // Vector of accepted characters
    let mut vec_char: Vec<bool> = Vec::with_capacity(256);

    // Init the vector as all characters invalids
    for _ in 0..256 {
        vec_char.push(false);
    }

    // Fill vec_char to see if it is a valid character
    if charset >= 31 {
        for i in buffer_generator_range::ALL_RANGE_0 {
            vec_char[i as usize] = true;
        }
    } else {
        if charset & buffer_generator_charset::NUMBER > 0 {
            for i in buffer_generator_range::NUMBER_RANGE_0 {
                vec_char[i as usize] = true;
            }
        }

        // Add lower case to charset
        if charset & buffer_generator_charset::LOWER_CASE > 0 {
            for i in buffer_generator_range::LOWER_CASE_RANGE_0 {
                vec_char[i as usize] = true;
            }
        }

        // Add upper case to charset
        if charset & buffer_generator_charset::UPPER_CASE > 0 {
            for i in buffer_generator_range::UPPER_CASE_RANGE_0 {
                vec_char[i as usize] = true;
            }
        }

        // Add symbol to charset
        if charset & buffer_generator_charset::SYMBOL > 0 {
            for i in buffer_generator_range::SYMBOL_RANGE_0 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::SYMBOL_RANGE_1 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::SYMBOL_RANGE_2 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::SYMBOL_RANGE_3 {
                vec_char[i as usize] = true;
            }
        }

        // Add unreadable to charset
        if charset & buffer_generator_charset::UNREADABLE > 0 {
            for i in buffer_generator_range::UNREADABLE_RANGE_0 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::UNREADABLE_RANGE_1 {
                vec_char[i as usize] = true;
            }
        } 
    }

    // Verify if character is valid. If ANY character is invalid, result will become false.
    for i in 0..buffer.len() {
        result = result && vec_char[buffer[i] as usize];
    }


    result
}