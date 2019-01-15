import os
import random
import string

import pytest
import fuzzy.ssdeep as ssdeep
import ssdeep as baseline

random.seed(42)


output_dir = os.path.abspath(os.path.dirname(__file__))
output_path = os.path.join(output_dir, 'a.txt')

n = 100000
random_data1 = ''.join([random.choice(string.ascii_letters + string.digits) for _ in range(n)])
random_data2 = ''.join([random.choice(string.ascii_letters + string.digits) for _ in range(n)])

random_data_1M = ''.join([random.choice(string.ascii_letters + string.digits) for _ in range(1000000)])
random_data_10M = ''.join([random.choice(string.ascii_letters + string.digits) for _ in range(10000000)])

a = "The quick brown fox jumps over the lazy dog"
digest = ssdeep.hash(a)
b = "The quick brown fox jumps over the lazy cat"
digest2 = ssdeep.hash(b)

@pytest.fixture(scope="session", autouse=True)
def testfile():
    with open(output_path, "w") as f:
        f.write(random_data1)
    yield
    os.remove(output_path)


def test_compare():
    score = ssdeep.compare(digest, digest2)
    score2 = baseline.compare(digest, digest2)
    assert score == score2

def test_hash():
    digest = ssdeep.hash(random_data1)
    digest2 = baseline.hash(random_data1)
    assert digest == digest2

def test_hash_from_file(testfile):
    digest = ssdeep.hash_from_file(output_path)
    assert len(digest) > 0

def test_bench_hash_str_libfuzzy(benchmark):
    benchmark(baseline.hash, a)

def test_bench_hash_str(benchmark):
    benchmark(ssdeep.hash, a)

def test_bench_hash_100K_libfuzzy(benchmark):
    benchmark(baseline.hash, random_data1)

def test_bench_hash_100K(benchmark):
    benchmark(ssdeep.hash, random_data1)

def test_bench_hash_1M_libfuzzy(benchmark):
    benchmark(baseline.hash, random_data_1M)

def test_bench_hash_1M(benchmark):
    benchmark(ssdeep.hash, random_data_1M)

def test_bench_hash_10M_libfuzzy(benchmark):
    benchmark(baseline.hash, random_data_10M)

def test_bench_hash_10M(benchmark):
    benchmark(ssdeep.hash, random_data_10M)

def test_bench_hash_from_file_100K_libfuzzy(benchmark, testfile):
    benchmark(baseline.hash_from_file, output_path)

def test_bench_hash_from_file_100K(benchmark):
    benchmark(ssdeep.hash_from_file, output_path)

def test_bench_compare_libfuzzy(benchmark):
    benchmark(baseline.compare, digest, digest2)

def test_bench_compare(benchmark):
    benchmark(ssdeep.compare, digest, digest2)