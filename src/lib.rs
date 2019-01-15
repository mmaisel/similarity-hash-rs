pub mod ssdeep;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

/// Digest trait is used for output types of similarity dhashing algorithms
pub trait Digest: fmt::Display {}

/// Score trait is used for output types from hash comparison
pub trait Score: fmt::Display {}

pub trait SimilarityHash {
    /// Associated digest type that's the output of hashing a message, should implement the `Digest` trait
    type Digest;
    /// Associated score type representing similarity between two digests, should implement the `Score` trait
    type Score;

    /// Returns the `Digest` instance at the current state of similarity hash.
    fn digest(&self) -> Self::Digest;

    /// Update the hash calculation with a single byte.
    ///
    /// # Arguments
    /// * `byte` - Byte from input to hash
    fn update(&mut self, byte: u8);

    /// Compare two digests and return a `Score` representing the similarity
    ///
    /// # Arguments
    /// * `a` - Reference to associated Digest type
    /// * `b` - Reference to associated Digest type
    fn compare(a: &Self::Digest, b: &Self::Digest) -> Self::Score;

    /// Hash an input string and return a digest
    ///
    /// # Arguments
    /// * `sample` - A string slice of input message to hash
    fn hash(sample: &str) -> Result<Self::Digest, io::Error>;

    /// Hash an input file and return a digest result
    ///
    /// # Arguments
    /// * `path` - A string slice to a local file path that will be hashed
    fn hash_from_file(path: &str) -> Result<Self::Digest, io::Error>;

    /// Hash an input buffer that implements Read trait and return a digest result
    ///
    /// # Arguments
    /// * `sample` - Type that implements the Read trait
    fn hash_buffer<T: Read>(&mut self, sample: T) -> Result<Self::Digest, io::Error> {
        let reader = BufReader::new(sample);
        for byte in reader.bytes() {
            let byte = byte?;
            self.update(byte);
        }
        Ok(self.digest())
    }
}
