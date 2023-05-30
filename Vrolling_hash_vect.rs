const MOD: usize = 1000003;

const BASE: usize = 2;
use std::arch::x86_64::*;


pub fn rolling_hash_vect(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    //println!("pattern {:?} ", pattern);

    //Length exeption
    if pattern.len() > text.len() || pattern.len() == 0 {
        return Vec::new();
    }

    let mut result: Vec<usize> = Vec::new();

    let base = BASE as u32;
    let modulus = MOD as u32;
    let pattern_len = pattern.len();
    let text_len = text.len();


    // Calculate hash of pattern
    let pattern_hash: usize = simd_polynomial_hash(pattern,base,modulus);
    //println!("pattern hash {} ", pattern_hash);
    // Calculate initial hash of text
    
    let first_text = &text[..pattern.len()];
    //println!("first_text {:?} ", first_text);
    let mut text_hash: usize = simd_polynomial_hash(first_text,base,modulus);
    //println!("initial_text_hash {} ", text_hash);
    // Check for match in initial substring
    if pattern_hash == text_hash && pattern == &text[0..pattern.len()] {
        result.push(0);
    }
    

    // Precompute base^(pattern_len - 1) for later use
    let mut base_pow: u32 = 1;
    for _ in 1..pattern_len {
        base_pow = (base_pow * base) % modulus;
    }

    // Iterate over the remaining substrings in the text
    for i in 1..=text_len - pattern_len {
        let removed_byte = text[i - 1];
        let added_byte = text[i + pattern_len - 1];
         //println!("");
         //println!("removed_byte {} ", removed_byte);
         //println!("added_byte {} ", added_byte);
        
        // Update the text hash using SIMD instructions
        text_hash = update_hash_simd(text_hash, removed_byte, added_byte, base, base_pow, modulus);
         //println!("text_hash {} ", text_hash);
        
        // Check for match with the pattern
        if pattern_hash == text_hash && pattern == &text[i..i + pattern_len] {
            result.push(i);
        }
    }

    result
}



fn simd_polynomial_hash(text: &[u8], base: u32, modulus: u32) -> usize {
    let mut hash: u32 = 0;
    let base_vector = unsafe { _mm_set1_epi32(base as i32) };

    let mut i = 0;
    while i + 16 <= text.len() {
        let text_chunk = unsafe { _mm_loadu_si128(text.as_ptr().add(i) as *const __m128i) };
        let text_values = unsafe { _mm_cvtepu8_epi32(text_chunk) };

        let product = unsafe { _mm_mullo_epi32(text_values, base_vector) };
        let sum = unsafe { _mm_add_epi32(product, _mm_srli_si128(product, 4)) };
        let sum_values = unsafe { _mm_cvtsi128_si32(sum) };

        hash = (hash.wrapping_mul(base) + sum_values as u32) % modulus;

        i += 16;
    }

    for j in i..text.len() {
        hash = (hash.wrapping_mul(base) + text[j] as u32) % modulus;
    }

    hash as usize
}





fn update_hash_simd(hash: usize, removed: u8, added: u8, base: u32, base_pow: u32, modulus: u32) -> usize {
    unsafe {
        // Convert constant values to SIMD registers
        let base_simd = _mm_set1_epi32(base as i32);
        let removed_simd = _mm_set1_epi32(removed as i32);
        let added_simd = _mm_set1_epi32(added as i32);
        let hash_simd = _mm_set1_epi32(hash as i32);
        let base_pow_simd = _mm_set1_epi32(base_pow as i32);
        

        // Multiply removed value by base^(pattern_len - 1)
        let removed_val_pow = _mm_mullo_epi32(removed_simd, base_pow_simd);

        // Subtract the removed hash value from the hash
        let subtracted_hash = _mm_sub_epi32(hash_simd, removed_val_pow);

        // Multiply base by the hash value
        let multiplied_hash = _mm_mullo_epi32(subtracted_hash, base_simd);

        // Add the added value to the subtracted hash
        let added_hash = _mm_add_epi32(multiplied_hash, added_simd);

        // Extract the lower 32 bits from the 128-bit result
        let multiplied_hash_low = _mm_cvtsi128_si32(added_hash) as u32;

        // Apply modulus to the hash
        let modulus_hash = multiplied_hash_low % modulus;

        modulus_hash as usize
    }
}

