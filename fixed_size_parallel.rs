use rayon::prelude::*;

pub fn fixed_size_parallel(pattern: &[u8], text: &[u8], chunk_size: usize) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();

    let mut results: Vec<Vec<usize>> = Vec::new();

    if m > n || m == 0 {
        return Vec::new();
    }

    if m > chunk_size {
        return Vec::new();
    }

    let mut start: usize = 0;
    let mut end: usize = chunk_size.min(n);

    while start < n {
        let chunk: &[u8] = &text[start..end];

        let chunk_results = chunk.par_iter().enumerate().filter_map(|(i, &byte)| {
            if byte == pattern[0] {
                if chunk.len() - i >= m {
                    let window: &[u8] = &chunk[i..i + m];
                    if window == pattern {
                        let offset: usize = start + i;
                        Some(offset)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }).collect::<Vec<usize>>();

        results.push(chunk_results);

        start += chunk_size;
        end = (end + chunk_size).min(n);
    }

    results.into_par_iter().flatten().collect()
}
