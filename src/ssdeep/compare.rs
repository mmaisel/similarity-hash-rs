use super::{MIN_BLOCK_SIZE, SPAMSUM_LENGTH};
use bcmp::{longest_common_substring, AlgoSpec};
use std::cmp::min;

const INSERT_COST: u32 = 1;
const DELETE_COST: u32 = 1;
const REPLACE_COST: u32 = 2;

/// Weighted edit distance
/// The ssdeep c++ implementation uses different costs compared to original paper,
/// in [1], the author notes spansum uses weights: insert/delete=1, change:3, swap:5
fn edit_distance(a: &[u8], b: &[u8]) -> u32 {
    let mut t1: Vec<u32> = vec![0; SPAMSUM_LENGTH as usize + 1];
    let mut t2: Vec<u32> = vec![0; SPAMSUM_LENGTH as usize + 1];
    let mut t3;

    for i2 in 0..b.len() + 1 {
        t1[i2] = i2 as u32 * DELETE_COST;
    }
    for i1 in 0..a.len() {
        t2[0] = (i1 as u32 + 1) * INSERT_COST;
        for i2 in 0..b.len() {
            let cost_a = t1[i2 + 1] + INSERT_COST;
            let cost_d = t2[i2] + DELETE_COST;
            let cost_r = t1[i2] + if a[i1] == b[i2] { 0 } else { REPLACE_COST };
            t2[i2 + 1] = min(min(cost_a, cost_d), cost_r);
        }
        t3 = t1;
        t1 = t2;
        t2 = t3;
    }
    t1[b.len()]
}

pub fn score(a: Vec<u8>, b: Vec<u8>, blocksize: u32) -> u32 {
    // invalid digest length
    if a.len() > SPAMSUM_LENGTH as usize || b.len() > SPAMSUM_LENGTH as usize {
        return 0;
    }

    let substring_match = longest_common_substring(&a, &b, AlgoSpec::TreeMatch(1));
    // no common substrings, so there's no match
    if substring_match.length == 0 {
        return 0;
    }

    let mut score = edit_distance(&a, &b);

    // Equation 6 from [1]:
    //  M = 100 - (100 * S  * e(s1, s2) / 64(l1 +l2)))
    score = (score * SPAMSUM_LENGTH) / ((a.len() + b.len()) as u32);
    score = (100 * score) / 64;
    // invalid score
    if score >= 100 {
        return 0;
    }
    score = 100 - score;

    let match_size = blocksize / MIN_BLOCK_SIZE * (min(a.len(), b.len()) as u32);

    if score > match_size {
        match_size
    } else {
        score
    }
}
