use std::thread;
use num_cpus;
use std::arch::x86_64::*;
const MOD: usize = 1_000_007;
const BASE: usize = 256;
pub fn rolling_hash_parallel_vectorisation(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    
    ///let num_threads = num_cpus::get();
    let num_threads = 1;
    //Length exeption
    if pattern.len() > text.len() || pattern.len() == 0 {
        return Vec::new();
    }

    // Split the text into chunks for each thread
    let chunk_size = text.len() / num_threads;
    let chunks: Vec<Vec<u8>> = (0..num_threads)
    .map(|i| text[i * chunk_size..(i + 1) * chunk_size].to_vec())
    .collect();

    // Define a mutable vector to store the results
    let mut result: Vec<usize> = Vec::new();

    // Spawn a thread for each chunk
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let pattern = pattern.to_owned();
            let mut chunk_result = Vec::new();
            let handle = thread::spawn(move || {
                chunk_result = rolling_hash(&pattern, &chunk);
                chunk_result
            });
            handle
        })
        .collect();

    // Collect the results from each thread
    for handle in handles {
        let chunk_result: Vec<usize> = handle.join().unwrap();
        result.extend(chunk_result);
    }

    result
}



fn rolling_hash(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    if pattern.len() > text.len() || pattern.len() == 0 {
        return Vec::new();
    }

    let mut result: Vec<usize> = Vec::new();
    // Calculate hash of pattern
    
    let mut base_pow: usize = 1;
    for _ in 0..pattern.len() {
        base_pow = (base_pow * BASE) & (MOD - 1);
    }

    let pattern_hash = calculate_pattern_hash(pattern,base_pow);

    
    // Calculate initial hash of text
    let text_len = text.len();
    let simd_pattern = pattern.chunks_exact(16);
    let simd_text = text.chunks_exact(16).map(unwrap_chunk);
    let text_hash = hash_initial(simd_text.clone(), text_len);
    
    
    // Check for match in initial substring
    for (i, (simd_p, simd_t)) in simd_pattern.zip(simd_text).enumerate() {
        if pattern_hash == text_hash[i] && simd_equal(simd_p, simd_t) {
            result.push(i * 16);
        }
    }

    // Precompute powers of the base
    

    // Iterate over remaining substrings of text
    let simd_text = text.chunks_exact(pattern.len()).map(unwrap_chunk);
    let mut text_hash: Vec<usize> = text_hash;
    for (i, simd_t) in simd_text.enumerate() {
        text_hash = hash_sliding(simd_t, pattern.len(), base_pow, &text_hash);

        let start = i * 16;
        let end = start + pattern.len();


        println!("simd_text i {:?} ", simd_t);
        println!("pattern hash {} ", pattern_hash);
        println!("text hash {} ", text_hash[i]);
        println!("pattern {:?} ", pattern);
        println!("text cicle {:?} ", &text[start..end]);

        if pattern_hash == text_hash[i] && pattern == &text[start..end] {
            result.push(start);
            
        }
        
    }

    result
}


fn hash_initial<'a, T>(text: T,text_len:usize) -> Vec<usize>
where
    T: Iterator<Item = &'a [u8]> + ExactSizeIterator,
{
    let simd_zero = unsafe { _mm_setzero_si128() };
    let simd_base = unsafe { _mm_set1_epi8(BASE as i8) };
    let simd_modulo = unsafe { _mm_set1_epi8((MOD - 1) as i8) };
    let mut hash: Vec<usize> = vec![0; text_len];

    for (i, simd_t) in text.enumerate() {
        let mut simd_hash = simd_zero;
        for j in 0..16 {
            let value = unsafe { _mm_set1_epi8(simd_t[j] as i8) };
            simd_hash = unsafe { _mm_and_si128(_mm_add_epi8(_mm_mullo_epi16(simd_hash, simd_base), value), simd_modulo) };
            let extracted_hash = unsafe { _mm_extract_epi64(simd_hash, 0) };
            hash[i * 16 + j] = extracted_hash as usize;
        }
    }

    hash
}

fn calculate_pattern_hash(pattern: &[u8], base_pow: usize) -> usize {
    let simd_pattern_len = pattern.len() as i16;
    let simd_zero = unsafe { _mm_setzero_si128() };
    let simd_base_pow = unsafe { _mm_set1_epi16(base_pow as i16) };

    let mut pattern_hash = 0;

    for i in 0..pattern.len() {
        let value = unsafe { _mm_set1_epi16(pattern[i] as i16) };

        let updated_hash = unsafe {
            let shifted_pattern_len = _mm_slli_epi16(_mm_set1_epi16(simd_pattern_len), 8);
            let shifted_hash = _mm_slli_epi16(_mm_set1_epi16(pattern_hash as i16), 8);
            let cmp_mask = _mm_cmpeq_epi16(shifted_hash, simd_zero);
            let cmp_result = _mm_movemask_epi8(cmp_mask) as i32;
            let cmp_shifted = cmp_result << (16 - pattern.len());

            let updated_value_lo = _mm_and_si128(_mm_srli_epi16(shifted_hash, 8), simd_base_pow);
            let updated_value_hi = _mm_and_si128(shifted_hash, value);
            let updated_value_shifted_lo = _mm_slli_epi16(updated_value_lo, 8);
            let updated_value_shifted_hi = _mm_srli_epi16(updated_value_hi, 8);
            let cmp_shifted_vec = _mm_set1_epi16(cmp_shifted as i16);
            let updated_value_lo_cond =
                _mm_blendv_epi8(updated_value_shifted_lo, shifted_pattern_len, cmp_shifted_vec);
            let updated_value_hi_cond = _mm_blendv_epi8(updated_value_shifted_hi, simd_zero, cmp_shifted_vec);

            _mm_or_si128(updated_value_lo_cond, updated_value_hi_cond)
        };

        let hash_element = unsafe {
            let mut hash = [0; 8];
            _mm_storeu_si128(hash.as_mut_ptr() as *mut __m128i, updated_hash);
            hash[0] as usize
        };

        pattern_hash = (pattern_hash * BASE + hash_element) % MOD;
    }

    pattern_hash
}


fn hash_sliding(text: &[u8], pattern_len: usize, base_pow: usize, text_hash: &[usize]) -> Vec<usize> {
    println!("text  {:?}", text);

    let simd_base = unsafe { _mm_set1_epi16(BASE as i16) };
    let simd_modulo = unsafe { _mm_set1_epi16((MOD - 1) as i16) };
    let mut updated_text_hash: Vec<usize> = vec![0; text_hash.len()];

    let simd_zero = unsafe { _mm_setzero_si128() };

    let simd_pattern_len = pattern_len as i16;
    let simd_pattern_len_vec = unsafe {
        _mm_setr_epi16(
            simd_pattern_len,
            simd_pattern_len,
            simd_pattern_len,
            simd_pattern_len,
            simd_pattern_len,
            simd_pattern_len,
            simd_pattern_len,
            simd_pattern_len,
        )
    };

    for i in 0..text.len() - pattern_len + 1 {
        let simd_prev_value = unsafe {
            let ptr = text[i..].as_ptr() as *const i16;
            _mm_loadu_si128(ptr as *const __m128i)
        };

        let simd_hash = unsafe {
            let simd_hash_index = i / 8;
            let simd_hash_ptr = text_hash.as_ptr().add(simd_hash_index);
            _mm_loadu_si128(simd_hash_ptr as *const __m128i)
        };

        let value = unsafe { _mm_set1_epi16(text[i + pattern_len - 1] as i16) };

        let updated_hash = unsafe {
            let mul_lo = _mm_mullo_epi16(simd_hash, simd_base);
            let mul_hi = _mm_mulhi_epi16(simd_hash, simd_base);
            let add_lo = _mm_add_epi16(mul_lo, value);
            let add_hi = _mm_add_epi16(mul_hi, value);
            let mod_lo = _mm_and_si128(add_lo, simd_modulo);
            let mod_hi = _mm_and_si128(add_hi, simd_modulo);
            let shifted_lo = _mm_slli_epi16(mod_lo, 8);
            let shifted_hi = _mm_srli_epi16(mod_hi, 8);
            _mm_or_si128(shifted_lo, shifted_hi)
        };

        let updated_value = unsafe {
            let simd_base_pow = _mm_set1_epi16(base_pow as i16);
            let simd_value_shifted = _mm_slli_epi16(value, 8);
            let shifted_hash = _mm_slli_epi16(updated_hash, 8);
            let shifted_pattern_len = _mm_slli_epi16(simd_pattern_len_vec, 8);
            let cmp_mask = _mm_cmpeq_epi16(shifted_hash, simd_zero);
            let cmp_result = _mm_movemask_epi8(cmp_mask) as i32;
            let cmp_shifted = cmp_result << (16 - pattern_len);

            let updated_value_lo = _mm_and_si128(_mm_srli_epi16(updated_hash, 8), simd_base_pow);
            let updated_value_hi = _mm_and_si128(updated_hash, simd_value_shifted);
            let updated_value_shifted_lo = _mm_slli_epi16(updated_value_lo, 8);
            let updated_value_shifted_hi = _mm_srli_epi16(updated_value_hi, 8);
            let cmp_shifted_vec = _mm_set1_epi16(cmp_shifted as i16);
            let updated_value_lo_cond =
                _mm_blendv_epi8(updated_value_shifted_lo, shifted_pattern_len, cmp_shifted_vec);
            let updated_value_hi_cond =
                _mm_blendv_epi8(updated_value_shifted_hi, simd_zero, cmp_shifted_vec);

            _mm_or_si128(updated_value_lo_cond, updated_value_hi_cond)
        };

        let hash_slice = &mut updated_text_hash[i..i + 8];
        unsafe { _mm_storeu_si128(hash_slice.as_mut_ptr() as *mut __m128i, updated_value) };
    }

    updated_text_hash
}


fn simd_equal(a: &[u8], b: &[u8]) -> bool {
    let len = a.len();
    let simd_a_ptr = a.as_ptr() as *const __m128i;
    let simd_b_ptr = b.as_ptr() as *const __m128i;
    let cmp = unsafe { _mm_cmpeq_epi8(_mm_loadu_si128(simd_a_ptr), _mm_loadu_si128(simd_b_ptr)) };
    let mask: i32 = unsafe { _mm_movemask_epi8(cmp) };

    mask == (1 << len) - 1
}

fn unwrap_chunk(chunk: &[u8]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(chunk.as_ptr(), chunk.len()) }
}

