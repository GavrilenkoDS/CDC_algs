use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod rolling_hash;
mod rolling_hash_vect;
use rolling_hash::rolling_hash;
use rolling_hash_vect::rolling_hash_vect;




fn main() {
    let mut file: File = File::open("C:\\Users\\Dmitrii\\Desktop\\myy\\merged.txt").expect("Failed to open file");
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content).expect("Failed to read file");
    
    let pattern: &[u8] = b"click";

    println!("text.len(): {:?}", &content.len());
    
    //Finding result 
    let start = Instant::now();
    let rolling_hash_result: Vec<usize> = rolling_hash(pattern, &content);
    let rolling_hash_result_time = start.elapsed();

    

    //Finding result vect
    let start = Instant::now();
    let rolling_hash_vect_result: Vec<usize> = rolling_hash_vect(pattern, &content);
    let rolling_hash_vect_result_time = start.elapsed();


    //println!("Rolling hash result: {:?}", rolling_hash_result);
    println!("Rolling hash result is {} matches", (rolling_hash_result.len()));
    println!("Rolling hash completed in {} seconds", (rolling_hash_result_time).as_secs_f32());

    //println!("Rolling hash vect result: {:?}", rolling_hash_vect_result);
    println!("Rolling hash vect result is {} matches", (rolling_hash_vect_result.len()));
    println!("Rolling hash vect completed in {} seconds", (rolling_hash_vect_result_time).as_secs_f32());
}

