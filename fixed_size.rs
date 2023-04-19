pub fn fixed_size(pattern: &[u8], text: &[u8], chunk_size: usize) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();

    let mut results: Vec<usize> = Vec::new();

    if m > n || m == 0 {
        return Vec::new();
    }

    if m > chunk_size {
        return results;
    }

    let mut start: usize = 0;
    let mut end: usize = chunk_size.min(n);

    while start < n {
        let chunk: &[u8] = &text[start..end];

        for (i, &byte) in chunk.iter().enumerate() {
            if byte == pattern[0] {
                if chunk.len() - i >= m {
                    let window: &[u8] = &chunk[i..i + m];
                    if window == pattern {
                        let offset: usize = start + i;
                        results.push(offset);
                    }
                }
            }
        }

        start += chunk_size;
        end = (end + chunk_size).min(n);
    }

    results
}
