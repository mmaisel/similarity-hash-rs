use super::SPAMSUM_LENGTH;
use std::num::Wrapping;

const FNV_HASH_PRIME: u32 = 0x01000193;
const FNV_HASH_INIT: u32 = 0x28021967;

// Traditional Hash, Fowler-Noll-Vo Hash FN
#[derive(Debug, Clone)]
pub struct Hash {
    pub hash: u32,
    pub half_hash: u32,
    pub digest: Vec<u8>,
    pub half_digest: u8,
    pub digest_length: u32,
}

impl Hash {
    pub fn new() -> Hash {
        Hash {
            hash: 0,
            half_hash: 0,
            digest: vec![0; SPAMSUM_LENGTH as usize],
            half_digest: 0,
            digest_length: 0,
        }
    }

    pub fn update(&mut self, byte: u8) {
        self.hash = self.calculate(byte, self.hash);
        self.half_hash = self.calculate(byte, self.half_hash);
    }

    pub fn calculate(&mut self, byte: u8, hash: u32) -> u32 {
        ((Wrapping(hash) * Wrapping(FNV_HASH_PRIME)) ^ Wrapping(byte as u32)).0
    }

    pub fn reset(&mut self, initialize: bool) {
        if !initialize {
            self.digest_length += 1;
        }
        self.digest[self.digest_length as usize] = 0;
        self.hash = FNV_HASH_INIT;
        if self.digest_length < SPAMSUM_LENGTH / 2 {
            self.half_hash = FNV_HASH_INIT;
            self.half_digest = 0;
        }
    }
}
