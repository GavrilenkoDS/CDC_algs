mod fixed_size;
mod fixed_size_parallel_rayon;
mod fixed_size_parallel_threads;
mod rolling_hash;
mod rolling_hash_parallel_threads;
mod rolling_hash_parallel_rayon;
mod rabin_karp;
mod rabin_karp_parallel_threads;
mod rabin_karp_parallel_rayon;
mod rolling_hash_vectorisation;
mod rolling_hash_parallel_vectorisation;


use fixed_size::fixed_size;
use fixed_size_parallel_rayon::fixed_size_parallel_rayon;
use fixed_size_parallel_threads::fixed_size_parallel_threads;
use rolling_hash::rolling_hash;
use rolling_hash_parallel_threads::rolling_hash_parallel_threads;
use rolling_hash_parallel_rayon::rolling_hash_parallel_rayon;
use rabin_karp::rabin_karp;
use rabin_karp_parallel_threads::rabin_karp_parallel_threads;
use rabin_karp_parallel_rayon::rabin_karp_parallel_rayon;
use rolling_hash_parallel_vectorisation::rolling_hash_parallel_vectorisation;
use rolling_hash_vectorisation::rolling_hash_vectorisation;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;


fn main() {
    
    let mut file: File = File::open("your file").expect("Failed to open file");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect("Failed to read file");
    
    let pattern: &[u8] = b"your pattern";

    println!("text.len(): {:?}", &content.len());

    //Finding result
    let start = Instant::now();
    let rabin_karp_result: Vec<usize> = rabin_karp(pattern, &content);
    let rabin_karp_duration = start.elapsed();

    let start = Instant::now();
    let rabin_karp_parallel_rayon_result: Vec<usize> = rabin_karp_parallel_rayon(pattern, &content);
    let rabin_karp_parallel_rayon_duration = start.elapsed();

    let start = Instant::now();
    let rabin_karp_parallel_threads_result: Vec<usize> = rabin_karp_parallel_threads(pattern, &content);
    let rabin_karp_parallel_threads_duration = start.elapsed();

    let start = Instant::now();
    let rolling_hash_result: Vec<usize> = rolling_hash(pattern, &content);
    let rolling_hash_duration = start.elapsed();

    let start = Instant::now();
    let rolling_hash_parallel_rayon_result: Vec<usize> = rolling_hash_parallel_rayon(pattern, &content);
    let rolling_hash_parallel_rayon_duration = start.elapsed();

    let start = Instant::now();
    let rolling_hash_parallel_threads_result: Vec<usize> = rolling_hash_parallel_threads(pattern, &content);
    let rolling_hash_parallel_threads_duration = start.elapsed();

    let start = Instant::now();
    let rolling_hash_parallel_vectorisation_result: Vec<usize> = rolling_hash_parallel_vectorisation(pattern, &content);
    let rolling_hash_parallel_vectorisation_duration = start.elapsed();

    let start = Instant::now();
    let rolling_hash_vectorisation_result: Vec<usize> = rolling_hash_vectorisation(pattern, &content);
    let rolling_hash_vectorisation_duration = start.elapsed();

    let start = Instant::now();
    let fixed_size_result: Vec<usize> = fixed_size(pattern, &content, 1024); // use chunk size of 1024 bytes
    let fixed_size_duration = start.elapsed();

    let start = Instant::now();
    let fixed_size_parallel_rayon_result: Vec<usize> = fixed_size_parallel_rayon(pattern, &content, 1024);
    let fixed_size_parallel_rayon_duration = start.elapsed();

    let start = Instant::now();
    let fixed_size_parallel_threads_result: Vec<usize> = fixed_size_parallel_threads(pattern, &content, 1024);
    let fixed_size_parallel_threads_duration = start.elapsed();

    println!();
    println!("Rabin-Karp result is {} matches", (rabin_karp_result.len()));
    println!("Rabin-Karp completed in {} seconds", (rabin_karp_duration).as_secs_f32());
    println!();
    println!("Rabin-Karp parallel_rayon result: {}", rabin_karp_parallel_rayon_result.len());
    println!("Rabin-Karp parallel_rayon completed in {} seconds", (rabin_karp_parallel_rayon_duration).as_secs_f32());
    println!();
    println!("Rabin-Karp parallel_threads result: {}", rabin_karp_parallel_threads_result.len());
    println!("Rabin-Karp parallel_threads completed in {} seconds", (rabin_karp_parallel_threads_duration).as_secs_f32());
    println!();
    println!("Rolling hash result is {} matches", (rolling_hash_result.len()));
    println!("Rolling hash completed in {} seconds", (rolling_hash_duration).as_secs_f32());
    println!();
    println!("Rolling hash parallel_rayon is {} matches", (rolling_hash_parallel_rayon_result.len()));
    println!("Rolling hash parallel_rayon completed in {} seconds", (rolling_hash_parallel_rayon_duration).as_secs_f32());
    println!();
    println!("Rolling hash parallel_threads is {} matches", (rolling_hash_parallel_threads_result.len()));
    println!("Rolling hash parallel_threads completed in {} seconds", (rolling_hash_parallel_threads_duration).as_secs_f32());
    println!();
    println!("Rolling hash vectorisation completed in {} seconds", (rolling_hash_vectorisation_duration).as_secs_f32());
    println!("Rolling hash vectorisation is {} matches", (rolling_hash_vectorisation_result.len()));
    println!();
    println!("Rolling hash parallel vectorisation completed in {} seconds", (rolling_hash_parallel_vectorisation_duration).as_secs_f32());
    println!("Rolling hash parallel vectorisation is {} matches", (rolling_hash_parallel_vectorisation_result.len()));
    println!();
    println!("Fixed-size result is {} matches", fixed_size_result.len());
    println!("Fixed-size completed in {} seconds", (fixed_size_duration).as_secs_f32());
    println!();
    println!("Fixed-size parallel_rayon resultis {} matches", fixed_size_parallel_rayon_result.len());
    println!("Fixed-size parallel_rayon completed in {} seconds", (fixed_size_parallel_rayon_duration).as_secs_f32());
    println!();
    println!("Fixed-size parallel_threads resultis {} matches", fixed_size_parallel_threads_result.len());
    println!("Fixed-size parallel_threads completed in {} seconds", (fixed_size_parallel_threads_duration).as_secs_f32());
    println!();

}

#[test]
fn single_test(){
    let mut file: File = File::open("C:\\Users\\Dmitrii\\Desktop\\myy\\merged.txt").expect("Failed to open file");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect("Failed to read file");
    
    let pattern: &[u8] = b"click";

    println!("text.len(): {:?}", &content.len());

    //Finding result
    let start = Instant::now();
    let result: Vec<usize> = rolling_hash_parallel_vectorisation(pattern, &content);
    let duration = start.elapsed();

    println!("result: {}", result.len());
    println!("completed in {} seconds", (duration).as_secs_f32());
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
