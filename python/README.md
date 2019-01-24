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

When benchmarking `libfuzzy` (python-ssdeep) to `similarity-hash-rs` in Python, *the rust implementation is about 2-4x slower on average*. There's still room for optimizations in rust, such as eliminating the use of `Vector`, which allocates on the heap, in traditional and rolling hashes structs.

```
----------------------------------------------------------------------------------------------------------- benchmark: 12 tests ------------------------------------------------------------------------------------------------------------
Name (time in us)                                    Min                     Max                    Mean                StdDev                  Median                   IQR            Outliers           OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_bench_compare_libfuzzy                       2.0000 (1.0)           81.0000 (1.16)           2.3436 (1.0)          1.6687 (1.0)            2.0000 (1.0)          0.0000 (inf)     776;14928  426,686.3396 (1.0)       71429           1
test_bench_compare                                7.0000 (3.50)         111.0000 (1.59)           8.6303 (3.68)         3.6820 (2.21)           8.0000 (4.00)         0.0000 (1.0)      453;3589  115,870.9800 (0.27)      12196           1

test_bench_hash_str                               3.0000 (1.50)          70.0000 (1.0)            4.1051 (1.75)         2.2808 (1.37)           4.0000 (2.00)         0.0000 (1.0)      482;7099  243,600.4993 (0.57)      23810           1
test_bench_hash_str_libfuzzy                      3.0000 (1.50)         244.0000 (3.49)           3.6962 (1.58)         2.4306 (1.46)           3.0000 (1.50)         1.0000 (inf)       347;412  270,545.9031 (0.63)      26316           1

test_bench_hash_100K_libfuzzy                   603.0000 (301.50)     1,124.0000 (16.06)        660.4023 (281.78)      65.5147 (39.26)        638.0000 (319.00)      49.0000 (inf)       166;141    1,514.2285 (0.00)       1397           1
test_bench_hash_100K                          2,080.0000 (>1000.0)    3,906.0000 (55.80)      2,245.7965 (958.25)     173.2093 (103.80)     2,192.0000 (>1000.0)    120.0000 (inf)         51;38      445.2763 (0.00)        452           1

test_bench_hash_from_file_100K_libfuzzy       1,179.0000 (589.50)     2,447.0000 (34.96)      1,263.0767 (538.94)     123.4018 (73.95)      1,218.0000 (609.00)      83.0000 (inf)         96;91      791.7175 (0.00)        769           1
test_bench_hash_from_file_100K                2,219.0000 (>1000.0)    3,045.0000 (43.50)      2,362.4366 (>1000.0)    152.0506 (91.12)      2,298.0000 (>1000.0)    147.0000 (inf)         69;27      423.2918 (0.00)        426           1

test_bench_hash_1M_libfuzzy                   5,889.0000 (>1000.0)    8,542.0000 (122.03)     6,216.7391 (>1000.0)    291.7063 (174.81)     6,141.0000 (>1000.0)    273.0000 (inf)          14;7      160.8560 (0.00)        161           1
test_bench_hash_1M                           19,669.0000 (>1000.0)   23,240.0000 (332.00)    20,241.2955 (>1000.0)    653.0711 (391.35)    20,055.5000 (>1000.0)    643.5000 (inf)           5;2       49.4040 (0.00)         44           1

test_bench_hash_10M_libfuzzy                 51,764.0000 (>1000.0)   56,094.0000 (801.34)    53,589.1250 (>1000.0)  1,476.5263 (884.81)    53,300.0000 (>1000.0)  2,308.0000 (inf)           5;0       18.6605 (0.00)         16           1
test_bench_hash_10M                         175,885.0000 (>1000.0)  182,062.0000 (>1000.0)  179,005.6667 (>1000.0)  2,016.5229 (>1000.0)  178,885.5000 (>1000.0)  1,534.0000 (inf)           2;1        5.5864 (0.00)          6           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```