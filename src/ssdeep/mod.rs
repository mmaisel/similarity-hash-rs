mod compare;
mod digest;
/// Fuzzy hashing implementation as definied in the ssdeep paper [1], the ssdeep project implementation [2], and rustysec's implementation [3].
///
/// [1] Jesse Kornblum, Identifying almost identical files using context triggered piecewise hashing, Digital Investigation, Volume 3, Supplement, 2006, Pages 91-97
/// [2] https://github.com/ssdeep-project/ssdeep
/// [3] https://github.com/rustysec/fuzzyhash-rs
///
mod rolling;
mod traditional;

use self::compare::score;
pub use self::digest::Signature;
pub use super::*;
use std::cmp::max;

const MIN_BLOCK_SIZE: u32 = 3;
const NUM_BLOCKHASHES: usize = 31;
const SPAMSUM_LENGTH: u32 = 64;
const MAX_SPAMSUM_RESULT_LENGTH: u32 = 2 * SPAMSUM_LENGTH + 20;
const BASE64_CHARS: &'static str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn get_base64_char(idx: usize) -> u8 {
    BASE64_CHARS.bytes().nth(idx).unwrap_or(0)
}

// Context Triggered Piecewise Hashing
pub struct Hash {
    rolling: rolling::Hash,
    blocks: [traditional::Hash; NUM_BLOCKHASHES],
    start: usize,
    end: usize,
    size: u32,
}

impl Hash {
    pub fn new() -> Hash {
        let mut hasher = Hash {
            rolling: rolling::Hash::new(),
            blocks: [traditional::Hash::new(); NUM_BLOCKHASHES],
            start: 0,
            end: 1,
            size: 0,
        };
        hasher.blocks[0].reset(true);
        hasher
    }

    fn fork_blocks(&mut self) {
        if self.end < NUM_BLOCKHASHES {
            self.blocks[self.end].hash = self.blocks[self.end - 1].hash;
            self.blocks[self.end].half_hash = self.blocks[self.end - 1].half_hash;
            self.blocks[self.end].digest[0] = 0;
            self.blocks[self.end].half_digest = 0;
            self.blocks[self.end].digest_length = 0;
            self.end += 1;
        } else if self.end == NUM_BLOCKHASHES - 1 {
            self.blocks[self.end].hash = self.blocks[self.end - 1].hash;
        }
    }

    fn reduce_blocks(&mut self) {
        if (self.end - self.start < 2)
            | ((MIN_BLOCK_SIZE << self.start) * SPAMSUM_LENGTH >= self.size)
            | (self.blocks[self.start + 1].digest_length < SPAMSUM_LENGTH / 2)
        {
            return;
        }
        self.start += 1;
    }
}

impl SimilarityHash for Hash {
    type Digest = Signature;
    type Score = u32;

    fn compare(a: &Signature, b: &Signature) -> u32 {
        // non-zero matches must have block size or within of power of two
        if a.blocksize != b.blocksize
            && a.blocksize != (2 * b.blocksize)
            && (2 * a.blocksize) != b.blocksize
        {
            return 0;
        }
        // remove reoccuring sequences from first and second signature chunks
        let mut a_1 = a.signature_1.to_vec();
        a_1.dedup();
        let mut a_2 = a.signature_2.to_vec();
        a_2.dedup();
        let mut b_1 = b.signature_1.to_vec();
        b_1.dedup();
        let mut b_2 = b.signature_2.to_vec();
        b_2.dedup();

        // identical chunksize and first signature is match
        if a.blocksize == b.blocksize && a_1 == b_1 {
            return 100;
        }

        if a.blocksize == b.blocksize {
            return max(
                score(a_1, b_1, a.blocksize),
                score(a_2, b_2, a.blocksize * 2),
            );
        } else if a.blocksize == 2 * b.blocksize {
            return score(a_1, b_2, a.blocksize);
        } else {
            return score(a_2, b_1, b.blocksize);
        }
    }

    fn update(&mut self, byte: u8) {
        // increment byte size
        self.size += 1;
        // update rolling hash
        self.rolling.update(byte);
        let hash = self.rolling.sum();

        // update block hashes
        for i in self.start..self.end {
            self.blocks[i].update(byte);
        }

        let mut i = self.start;
        while i < self.end {
            if hash % (MIN_BLOCK_SIZE << i) != (MIN_BLOCK_SIZE << i) - 1 {
                break;
            }

            if self.blocks[i].digest_length == 0 {
                self.fork_blocks();
            }

            let j = self.blocks[i].digest_length as usize;
            self.blocks[i].digest[j] = get_base64_char((self.blocks[i].hash % 64) as usize);
            self.blocks[i].half_digest = get_base64_char((self.blocks[i].half_hash % 64) as usize);

            if self.blocks[i].digest_length < SPAMSUM_LENGTH - 1 {
                self.blocks[i].reset(false);
            } else {
                self.reduce_blocks();
            }
            i += 1;
        }
    }

    fn digest(&self) -> Signature {
        let mut digest = vec![0; MAX_SPAMSUM_RESULT_LENGTH as usize];
        // idx in digest
        let mut position = 0;
        // index into traditional block hashes
        let mut idx = self.start;
        // rolling hash
        let mut hash = self.rolling.sum();

        while (MIN_BLOCK_SIZE << idx) * SPAMSUM_LENGTH < self.size {
            idx += 1;
        }

        while idx >= self.end {
            idx -= 1;
        }

        while idx > self.start && self.blocks[idx as usize].digest_length < SPAMSUM_LENGTH / 2 {
            idx -= 1;
        }

        let blocksize = MIN_BLOCK_SIZE << idx;
        let mut i = self.blocks[idx as usize].digest_length as usize;

        for k in 0..i {
            digest[position + k] = self.blocks[idx as usize].digest[k];
        }

        position += i;
        if hash != 0 {
            digest[position as usize] = get_base64_char((self.blocks[idx].hash % 64) as usize);
            position += 1;
        } else if self.blocks[idx as usize].digest[i as usize] != 0 {
            digest[position as usize] = self.blocks[idx].digest[i as usize];
            position += 1;
        }
        let signature_1 = digest[0..position].to_vec();

        digest = vec![0; MAX_SPAMSUM_RESULT_LENGTH as usize];
        position = 0;

        if idx < self.end - 1 {
            idx += 1;
            i = self.blocks[idx as usize].digest_length as usize;

            if i > ((SPAMSUM_LENGTH / 2) - 1) as usize {
                i = ((SPAMSUM_LENGTH / 2) - 1) as usize;
            }

            for k in 0..i {
                digest[position + k] = self.blocks[idx as usize].digest[k];
            }

            position += i;

            if hash != 0 {
                hash = self.blocks[idx as usize].half_hash;
                digest[position] = get_base64_char((hash % 64) as usize);
                position += 1;
            } else {
                i = self.blocks[idx as usize].half_digest as usize;
                if i != 0 {
                    digest[position] = i as u8;
                    position += 1;
                }
            }
        } else if hash != 0 {
            digest[position] = get_base64_char((self.blocks[idx as usize].hash % 64) as usize);
        }

        let signature = Signature {
            blocksize: blocksize,
            signature_1: signature_1,
            signature_2: digest[0..position].to_vec(),
        };

        signature
    }

    /// Hash an input file
    fn hash(sample: &str) -> Result<Self::Digest, io::Error> {
        let bytes = sample.as_bytes();
        let mut hasher = Hash::new();
        return hasher.hash_buffer(bytes);
    }

    /// Hash an input file
    fn hash_from_file(path: &str) -> Result<Self::Digest, io::Error> {
        let sample = File::open(path)?;
        let mut hasher = Hash::new();
        return hasher.hash_buffer(sample);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &'static str = "0:FJKKIUKact:FHIGi";
    const FOX_HASH: &'static str = "3:FJKKIUKact:FHIGi";
    const FOX: &'static str = "The quick brown fox jumps over the lazy dog";
    const FOX1: &'static str = "The quick brown fox jumps over the lazy cat";

    #[test]
    fn hash() {
        let result = Hash::hash(FOX).unwrap();
        assert!(result.to_string() == FOX_HASH);
    }

    #[test]
    fn hash_from_string() {
        let a = Signature::from_string(FOX_HASH);
        assert!(a.blocksize == 3);
    }

    #[test]
    fn compare_equal() {
        let a = Hash::hash(FOX).unwrap();
        let score = Hash::compare(&a, &a);
        assert!(score == 100);
    }

    #[test]
    fn compare_similar() {
        let a = Hash::hash(FOX).unwrap();
        let b = Hash::hash(FOX1).unwrap();
        let score = Hash::compare(&a, &b);
        println!("{}", score);
        assert!(score == 10);
    }

    #[test]
    fn compare_different_blocksize() {
        let score = Hash::compare(
            &Signature::from_string(FOX_HASH),
            &Signature::from_string(HASH),
        );
        assert!(score == 0)
    }
}
