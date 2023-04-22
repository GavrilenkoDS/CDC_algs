mod rolling_hash;
mod rolling_hash_parallel;
mod rabin_karp;
mod rabin_karp_parallel;
mod fixed_size;
mod fixed_size_parallel;

use rolling_hash::rolling_hash;
use rolling_hash_parallel::rolling_hash_parallel;
use rabin_karp::rabin_karp;
use rabin_karp_parallel::rabin_karp_parallel;
use fixed_size::fixed_size;
use fixed_size_parallel::fixed_size_parallel;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;


fn main() {
    
    let mut file: File = File::open("C:\\Users\\Dmitrii\\Desktop\\study_rust\\bigtest.txt").expect("Failed to open file");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect("Failed to read file");

    let pattern: &[u8] = b"harmony ";

    

    // Finding result
    let start = Instant::now();
    let rabin_karp_result: Vec<usize> = rabin_karp(pattern, &content);
    let rabin_karp_duration = start.elapsed();

    let start = Instant::now();
    let rabin_karp_parallel_result: Vec<usize> = rabin_karp_parallel(pattern, &content);
    let rabin_karp_parallel_duration = start.elapsed();

    let start = Instant::now();
    let rolling_hash_result: Vec<usize> = rolling_hash(pattern, &content);
    let rolling_hash_duration = start.elapsed();

    let start = Instant::now();
    let rolling_hash_parallel_result: Vec<usize> = rolling_hash_parallel(pattern, &content);
    let rolling_hash_parallel_duration = start.elapsed();

    let start = Instant::now();
    let fixed_size_result: Vec<usize> = fixed_size(pattern, &content, 1024); // use chunk size of 1024 bytes
    let fixed_size_duration = start.elapsed();

    let start = Instant::now();
    let fixed_size_parallel_result: Vec<usize> = fixed_size_parallel(pattern, &content, 1024);
    let fixed_size_parallel_duration = start.elapsed();

    println!();
    //println!("Rabin-Karp result: {:?}", rabin_karp_result);
    println!("Rabin-Karp completed in {} seconds", (rabin_karp_duration).as_secs_f32());
    println!();
    //println!("Rabin-Karp parallel result: {:?}", rabin_karp_parallel_result);
    println!("Rabin-Karp parallel completed in {} seconds", (rabin_karp_parallel_duration).as_secs_f32());
    println!();
    //println!("Rolling hash result: {:?}", rolling_hash_result);
    println!("Rolling hash completed in {} seconds", (rolling_hash_duration).as_secs_f32());
    println!();
    //println!("Rolling hash parallel result: {:?}", rolling_hash_parallel_result);
    println!("Rolling hash parallel completed in {} seconds", (rolling_hash_parallel_duration).as_secs_f32());
    println!();
    //println!("Fixed-size result: {:?}", fixed_size_result);
    println!("Fixed-size completed in {} seconds", (fixed_size_duration).as_secs_f32());
    println!();
    //println!("Fixed-size parallel result: {:?}", fixed_size_parallel_result);
    println!("Fixed-size parallel completed in {} seconds", (fixed_size_parallel_duration).as_secs_f32());
    println!();

}


#[test]
fn test_rolling_hash() {
    let start = Instant::now();

    let pattern = b"abc";
    let text = b"abcdabc";
    let result = rolling_hash(pattern, text);
    assert_eq!(result, vec![0, 4]);

    let pattern = b"abab";
    let text = b"ababcababa";
    let result = rolling_hash(pattern, text);
    assert_eq!(result, vec![0, 5]);

    let pattern = b"aaaa";
    let text = b"aaaabaaaaaa";
    let result = rolling_hash(pattern, text);
    assert_eq!(result, vec![0, 5, 6, 7]);

    let pattern = b"abc";
    let text = b"defg";
    let result = rolling_hash(pattern, text);
    assert_eq!(result, vec![]);

    let pattern = b"";
    let text = b"abcd";
    let result = rolling_hash(pattern, text);
    assert_eq!(result, vec![]);

    let pattern = b"abcd";
    let text = b"ab";
    let result = rolling_hash(pattern, text);
    assert_eq!(result, vec![]);

    let duration = start.elapsed();
    println!("Test completed in {} seconds", duration.as_secs_f32());

}

#[test]
fn test_rabin_karp() {
    let start = Instant::now();

    let pattern = b"abc";
    let text = b"abcdabc";
    let result = rabin_karp(pattern, text);
    assert_eq!(result, vec![0, 4]);

    let pattern = b"abab";
    let text = b"ababcababa";
    let result = rabin_karp(pattern, text);
    assert_eq!(result, vec![0, 5]);

    let pattern = b"aaaa";
    let text = b"aaaabaaaaaa";
    let result = rabin_karp(pattern, text);
    assert_eq!(result, vec![0, 5, 6, 7]);

    let pattern = b"abc";
    let text = b"defg";
    let result = rabin_karp(pattern, text);
    assert_eq!(result, vec![]);

    let pattern = b"";
    let text = b"abcd";
    let result = rabin_karp(pattern, text);
    assert_eq!(result, vec![]);

    let pattern = b"abcd";
    let text = b"ab";
    let result = rabin_karp(pattern, text);
    assert_eq!(result, vec![]);

    let duration = start.elapsed();
    println!("Test completed in {} seconds", duration.as_secs_f32());

}

#[test]
fn test_fixed_size() {
    let start = Instant::now();

    let pattern = b"abc";
    let text = b"abcbabc";
    let result = fixed_size(pattern, text, 3);
    assert_eq!(result, vec![0]); // cause chunk size is 3

    let pattern = b"abc";
    let text = b"abcbcdabc";
    let result = fixed_size(pattern, text, 3);
    assert_eq!(result, vec![0,6]); // its good
   

    let pattern = b"abab";
    let text = b"ababcababa";
    let result = fixed_size(pattern, text, 4);
    assert_eq!(result, vec![0]); // cause chunk size is 4


    let pattern = b"abab";
    let text = b"ababcqweababa";
    let result = fixed_size(pattern, text, 4);
    assert_eq!(result, vec![0,8]); // its good

    let pattern = b"aaaa";
    let text = b"aaaabaaaaaa";
    let result = fixed_size(pattern, text, 6);
    assert_eq!(result, vec![0, 6, 7]); //not 0, 5, 6, 7

    let pattern = b"abc";
    let text = b"defg";
    let result = fixed_size(pattern, text, 2);
    assert_eq!(result, vec![]);

    let pattern = b"";
    let text = b"abcd";
    let result = fixed_size(pattern, text, 2);
    assert_eq!(result, vec![]);

    let pattern = b"abcd";
    let text = b"ab";
    let result = fixed_size(pattern, text, 2);
    assert_eq!(result, vec![]);

    let duration = start.elapsed();
    println!("Test completed in {} seconds", duration.as_secs_f32());

}

#[test]
fn test_fixed_size_parallel() {
    let start = Instant::now();

    let pattern = b"abc";
    let text = b"abcbabc";
    let result = fixed_size_parallel(pattern, text, 3);
    assert_eq!(result, vec![0]); // cause chunk size is 3

    let pattern = b"abc";
    let text = b"abcbcdabc";
    let result = fixed_size_parallel(pattern, text, 3);
    assert_eq!(result, vec![0,6]); // its good
   

    let pattern = b"abab";
    let text = b"ababcababa";
    let result = fixed_size_parallel(pattern, text, 4);
    assert_eq!(result, vec![0]); // cause chunk size is 4


    let pattern = b"abab";
    let text = b"ababcqweababa";
    let result = fixed_size_parallel(pattern, text, 4);
    assert_eq!(result, vec![0,8]); // its good

    let pattern = b"aaaa";
    let text = b"aaaabaaaaaa";
    let result = fixed_size_parallel(pattern, text, 6);
    assert_eq!(result, vec![0, 6, 7]); //not 0, 5, 6, 7

    let pattern = b"abc";
    let text = b"defg";
    let result = fixed_size_parallel(pattern, text, 2);
    assert_eq!(result, vec![]);

    let pattern = b"";
    let text = b"abcd";
    let result = fixed_size_parallel(pattern, text, 2);
    assert_eq!(result, vec![]);

    let pattern = b"abcd";
    let text = b"ab";
    let result = fixed_size_parallel(pattern, text, 2);
    assert_eq!(result, vec![]);

    let duration = start.elapsed();
    println!("Test completed in {} seconds", duration.as_secs_f32());

}
