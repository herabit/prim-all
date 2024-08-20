use std::fs;
use std::time::Instant;

use bytemuck::{cast_slice, try_zeroed_slice_box};
// use primality::IsPrime;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

const BITS: u64 = u32::MAX as u64 + 1;
const WORDS: u32 = (BITS / (u32::BITS as u64)) as u32;

pub mod primality;

fn main() {
    let word_len = usize::try_from(WORDS).expect("Required amount of u32s is too large.");

    let mut words = try_zeroed_slice_box::<u32>(word_len)
        .expect("Failed to allocate the necessary amount of u32s");

    let chunk_size = (rayon::max_num_threads() + 1) >> 1;
    let chunks = word_len / chunk_size;

    assert!(
        word_len % chunk_size == 0,
        "Chunk size is not a factor of the word length"
    );

    let start = Instant::now();

    words
        .par_chunks_exact_mut(chunk_size)
        .enumerate()
        .map(|(i, chunk)| {
            let start = i * chunk_size;
            let end = start + chunk_size;

            // let check = IsPrime::new();

            let mut value = start as u32;

            for word in chunk.iter_mut() {
                for shift in 0u32..32 {
                    // let mut bit = check.is_prime(value) as u32;
                    let mut bit = primal::is_prime(value as u64) as u32;
                    bit <<= shift;

                    *word |= bit;

                    value += 1;
                }

                // Little endian is superior.
                *word = word.to_le();
            }

            start..end
        })
        .for_each(|range| println!("Done: {range:?}"));

    let elapsed = start.elapsed();

    println!(
        "Generated {chunks} chunks in {} seconds. Chunk size: {chunk_size}",
        elapsed.as_secs_f32()
    );

    fs::write("primes.bin", cast_slice(&*words)).expect("Failed to write the bitset");
}
