# fuzzy-hash-rs

This library implements similarity digest hashing algorithms in rust and exposes ffi bindings for python3. 

## Status

Supported algorithms include:

- ssdeep

This is a work in progress. I'd like to include these algorithms in future work:

- [TLSH](https://github.com/trendmicro/tlsh)
- [sdhash](https://github.com/sdhash/sdhash)
- [LZJD](https://github.com/EdwardRaff/pyLZJD)

I don't currently have this repo configured to publish to crates.io or pypi.

## Example

```
use fuzzy::ssdeep::*;

let digest = Hash::hash("The quick brown fox jumps over the lazy dog");

let digest_b = Hash::hash_from_file("/path/to/sample.exe");

let score = Hash::compare(&digest, &digest_b);
```

See the README in `python/` for python3 usage.

## About

This project started out as a exercise to learn rust and ffi in python. All comments or PRs for improvements are welcome!

## References

1. Jesse Kornblum, Identifying almost identical files using context triggered piecewise hashing, Digital Investigation, Volume 3, Supplement, 2006, Pages 91-97
2. https://github.com/ssdeep-project/ssdeep
3. https://github.com/rustysec/fuzzyhash-rs