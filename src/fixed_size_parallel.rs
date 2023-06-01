use rayon::prelude::*;
use rayon::slice::ParallelSlice;
use std::sync::Mutex;

pub fn fixed_size_parallel(pattern: &[u8], text: &[u8], chunk_size: usize) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();
    let results: Mutex<Vec<usize>> = Mutex::new(Vec::new());

    if m == 0 || m > n {
        return Vec::new();
    }

    let chunks = text.par_chunks(chunk_size).enumerate();

    chunks
        .flat_map(|(i, chunk)| {
            chunk
                .windows(m)
                .enumerate()
                .filter_map(move |(j, slice)| {
                    let pos = i * chunk_size + j;
                    if slice == pattern {
                        Some(pos)
                    } else {
                        None
                    }
                })
                .par_bridge()
        })
        .for_each(|pos| {
            results.lock().unwrap().push(pos);
        });

    results.into_inner().unwrap()
}
