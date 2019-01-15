use fuzzy::ssdeep::*;

#[test]
fn test_calculate() {
    let _digest = Hash::hash_from_file("tests/a.txt").unwrap();
    // assert!(digest);
}