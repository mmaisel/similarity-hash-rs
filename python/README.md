# similarity-hash

This module contains python bindings to `similarity-hash-rs` using [PyO3](https://github.com/PyO3). It's a work in progress and does not currently build or deploy to pypi.

## Setup

This module has only been tested in python3.6 at this time in a virtualenv.

```
# virtualenv is used for a local development environment
$ make env
$ make test
```

## Examples

```python
import fuzzy.ssdeep as ssdeep

a = ssdeep.hash("The quick brown fox jumps over the lazy dog")
b = ssdeep.hash("The quick brown fox jumps over the lazy cat")

score = ssdeep.compare(a, b)

c = ssdeep.hash("path/to/sample.exe")
```

## Benchmarks

I'm getting worse benchmarks when comparing `libfuzzy` (python-ssdeep) to `similarity-hash-rs`. Interestingly enough, when running benchmarks in cargo, I observe ~6.2ms avg for hashing 100K chars and ~57ms avg for hashing 1M chars. I'm still investigating why there's a performance hit. There was no difference when changing the PyO3 bindings to use the `rust-cpython` backend.

```
---------------------------------------------------------------------------------------------------------------- benchmark: 12 tests -----------------------------------------------------------------------------------------------------------------
Name (time in us)                                      Min                       Max                      Mean                 StdDev                    Median                    IQR            Outliers           OPS            Rounds  Iterations
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_bench_compare_libfuzzy                         2.0000 (1.0)            513.0000 (14.66)            2.4599 (1.0)           2.4348 (1.64)             2.0000 (1.0)           1.0000 (1.0)     1070;1070  406,527.1474 (1.0)       83334           1
test_bench_hash_str_libfuzzy                        3.0000 (1.50)            35.0000 (1.0)              3.5587 (1.45)          1.4868 (1.0)              3.0000 (1.50)          1.0000 (1.0)       323;323  281,001.3221 (0.69)      31251           1
test_bench_hash_str                                64.0000 (32.00)          435.0000 (12.43)           69.0221 (28.06)        11.2837 (7.59)            66.0000 (33.00)         2.0000 (2.00)     632;1085   14,488.1045 (0.04)       8265           1
test_bench_compare                                158.0000 (79.00)          568.0000 (16.23)          185.3031 (75.33)        38.3575 (25.80)          171.0000 (85.50)        14.0000 (14.00)     509;710    5,396.5627 (0.01)       3985           1
test_bench_hash_100K_libfuzzy                     604.0000 (302.00)       1,144.0000 (32.69)          658.4549 (267.68)       59.5136 (40.03)          638.0000 (319.00)       29.0000 (29.00)     128;144    1,518.7070 (0.00)       1429           1
test_bench_hash_from_file_100K_libfuzzy         1,150.0000 (575.00)       2,540.0000 (72.57)        1,296.4807 (527.05)      176.5005 (118.71)       1,237.5000 (618.75)      147.0000 (147.00)      80;54      771.3189 (0.00)        724           1
test_bench_hash_1M_libfuzzy                     5,909.0000 (>1000.0)      6,747.0000 (192.77)       6,176.9222 (>1000.0)     171.4587 (115.32)       6,123.0000 (>1000.0)     243.5000 (243.50)       45;3      161.8929 (0.00)        167           1
test_bench_hash_10M_libfuzzy                   51,281.0000 (>1000.0)     53,217.0000 (>1000.0)     51,764.0000 (>1000.0)     472.9758 (318.12)      51,742.0000 (>1000.0)     319.5000 (319.50)        3;2       19.3184 (0.00)         17           1
test_bench_hash_100K                           68,212.0000 (>1000.0)     75,012.0000 (>1000.0)     69,664.8667 (>1000.0)   1,899.1004 (>1000.0)     69,038.0000 (>1000.0)     965.0000 (965.00)        2;2       14.3544 (0.00)         15           1
test_bench_hash_from_file_100K                 68,796.0000 (>1000.0)     79,881.0000 (>1000.0)     72,332.0667 (>1000.0)   3,148.0455 (>1000.0)     70,927.0000 (>1000.0)   3,923.2500 (>1000.0)       3;1       13.8251 (0.00)         15           1
test_bench_hash_1M                            525,986.0000 (>1000.0)    528,892.0000 (>1000.0)    527,238.4000 (>1000.0)   1,289.2164 (867.13)     526,764.0000 (>1000.0)   2,270.0000 (>1000.0)       1;0        1.8967 (0.00)          5           1
test_bench_hash_10M                         3,732,097.0000 (>1000.0)  3,835,714.0000 (>1000.0)  3,770,268.8000 (>1000.0)  40,912.8091 (>1000.0)  3,756,438.0000 (>1000.0)  53,433.0000 (>1000.0)       1;0        0.2652 (0.00)          5           1
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```