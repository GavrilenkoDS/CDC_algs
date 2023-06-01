use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;


mod fixed_size;
mod fixed_size_parallel_rayon;
mod fixed_size_parallel_threads;
mod rolling_hash;
mod rolling_hash_parallel_threads;
mod rolling_hash_parallel_rayon;
mod rolling_hash_parallel_crossbeam;
mod rabin_karp;
mod rabin_karp_parallel_threads;
mod rabin_karp_parallel_rayon;
mod rolling_hash_avx2;

use fixed_size::fixed_size;
use fixed_size_parallel_rayon::fixed_size_parallel_rayon;
use fixed_size_parallel_threads::fixed_size_parallel_threads;
use rolling_hash::rolling_hash;
use rolling_hash_parallel_threads::rolling_hash_parallel_threads;
use rolling_hash_parallel_rayon::rolling_hash_parallel_rayon;
use rolling_hash_parallel_crossbeam::rolling_hash_parallel_crossbeam;
use rabin_karp::rabin_karp;
use rabin_karp_parallel_threads::rabin_karp_parallel_threads;
use rabin_karp_parallel_rayon::rabin_karp_parallel_rayon;
use rolling_hash_avx2::rolling_hash_avx2;

fn main() {
    let mut file: File = File::open("C:\\Users\\Dmitrii\\Desktop\\myy\\merged.txt").expect("Failed to open file");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect("Failed to read file");
    
    let pattern: &[u8] = b"click";
    //let pattern: &[u8] = b"political and economic stability and for future";
    println!("text.len(): {:?}", &content.len());

    //Finding result
    let start = Instant::now();

    let result: Vec<usize> = rolling_hash_avx2(pattern, &content);

    let duration = start.elapsed();

    println!("result: {}", result.len());
    println!("completed in {} seconds", (duration).as_secs_f32());
    println!();
}
