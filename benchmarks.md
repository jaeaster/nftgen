# Benchmarks

## Setup

The benchmarks were ran for:

- [nftgen @ commit a41febb](https://github.com/jaeaster/nftgen/tree/a41febbaf7f933bf34cabc76a6193c2d232173fa)
- [Hashlips Art Engine @ commit d8ee279](https://github.com/HashLips/hashlips_art_engine/tree/d8ee279043d2d4a8de3bdfac0d89d0e966fb04a2)

The benchmarks were run with the following specs:

- Macbook Pro
- Apple M1 Max chip (10 cores)
- 32GB RAM
- MacOS 12.4

The benchmarks used [hyperfine](https://github.com/sharkdp/hyperfine), a tool that runs the programs multiple times and calculates averages and variance.

## Conclusion

nftgen is ~10x faster than Hashlips Art Engine! Blazingly Fast!
The speedup is ~1.4x on a machine with a single core.

## Results


### Hyperfine Command

`N=10 hyperfine -m 3 -n hashlips "cd ../hashlips_art_engine && sed -i '' 's/growEditionSizeTo: [0-9]*/growEditionSizeTo: ${N}/' src/config.js && yarn build" -n nftgen "NFTGEN_CONFIG_PATH=./config target/release/nftgen generate -n ${N}"`

### N = 10, Multi-Core

```
Benchmark 1: hashlips
  Time (mean ± σ):      6.171 s ±  0.033 s    [User: 5.828 s, System: 0.298 s]
  Range (min … max):    6.141 s …  6.206 s    3 runs

Benchmark 2: nftgen
  Time (mean ± σ):     852.3 ms ±  50.6 ms    [User: 5524.2 ms, System: 375.0 ms]
  Range (min … max):   795.5 ms … 892.2 ms    3 runs

Summary
  'nftgen' ran
    7.24 ± 0.43 times faster than 'hashlips'
```

### N = 100, Multi Core

```
Benchmark 1: hashlips
  Time (mean ± σ):     65.411 s ±  1.955 s    [User: 63.805 s, System: 1.957 s]
  Range (min … max):   64.208 s … 67.667 s    3 runs

Benchmark 2: nftgen
  Time (mean ± σ):      6.647 s ±  0.076 s    [User: 53.353 s, System: 2.304 s]
  Range (min … max):    6.560 s …  6.695 s    3 runs

Summary
  'nftgen' ran
    9.84 ± 0.31 times faster than 'hashlips'
```

### N = 1000, Multi Core

```
Benchmark 1: hashlips
  Time (mean ± σ):     653.654 s ± 11.926 s    [User: 641.144 s, System: 18.183 s]
  Range (min … max):   646.027 s … 667.397 s    3 runs

Benchmark 2: nftgen
  Time (mean ± σ):     62.256 s ±  0.909 s    [User: 542.977 s, System: 20.641 s]
  Range (min … max):   61.292 s … 63.097 s    3 runs

Summary
  'nftgen' ran
   10.50 ± 0.25 times faster than 'hashlips'
```

### N = 10, Single Core

```
Benchmark 1: hashlips
  Time (mean ± σ):      6.913 s ±  0.934 s    [User: 6.452 s, System: 0.326 s]
  Range (min … max):    6.241 s …  7.980 s    3 runs

Benchmark 2: nftgen
  Time (mean ± σ):      4.914 s ±  0.125 s    [User: 4.670 s, System: 0.192 s]
  Range (min … max):    4.773 s …  5.007 s    3 runs

Summary
  'nftgen' ran
    1.41 ± 0.19 times faster than 'hashlips'
```

### N = 100, Single Core

```
Benchmark 1: hashlips
  Time (mean ± σ):     69.062 s ±  4.844 s    [User: 67.436 s, System: 2.024 s]
  Range (min … max):   65.048 s … 74.442 s    3 runs

Benchmark 2: nftgen
  Time (mean ± σ):     49.048 s ±  0.634 s    [User: 47.172 s, System: 1.824 s]
  Range (min … max):   48.643 s … 49.779 s    3 runs

Summary
  'nftgen' ran
    1.41 ± 0.10 times faster than 'hashlips'
```

### N = 1000, single core

```
Benchmark 1: hashlips
  Time (mean ± σ):     638.141 s ±  5.959 s    [User: 627.131 s, System: 16.814 s]
  Range (min … max):   633.551 s … 644.875 s    3 runs

Benchmark 2: nftgen
  Time (mean ± σ):     484.487 s ±  1.373 s    [User: 468.549 s, System: 15.352 s]
  Range (min … max):   482.913 s … 485.437 s    3 runs

Summary
  'nftgen' ran
    1.32 ± 0.01 times faster than 'hashlips'
```

## Results from an older, unoptimized version of nftgen

| N      | Tool     | Clock Time (s) |
| ------ | -------- | -------------- |
| 10     | nftgen   | 2.859          |
| 10     | Hashlips | 5.999          |
| 100    | nftgen   | 21.988         |
| 100    | Hashlips | 66.15          |
| 1,000  | nftgen   | 221.53         |
| 1,000  | Hashlips | 647.88         |
| 10,000 | nftgen   | 3302.88        |
| 10,000 | Hashlips | 6460.75        |

### N = 10

- nftgen
  `NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen -n 10 21.89s user 0.77s system 792% cpu 2.859 total`

- Hashlips
  `yarn build 5.70s user 0.29s system 99% cpu 5.999 total`

### N = 100

- nftgen
  `NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen -n 100 180.19s user 2.77s system 832% cpu 21.988 total`

- Hashlips
  `yarn build 64.58s user 1.94s system 100% cpu 1:06.15 total`

### N = 1000

- nftgen
  `NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen -n 1000 1768.36s user 20.95s system 887% cpu 3:21.53 total`

- Hashlips
  `yarn build 635.19s user 17.88s system 100% cpu 10:47.88 total`

### N = 10,000

- nftgen
  `NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen 17644.56s user 199.67s system 540% cpu 55:02.88 total`

- Hashlips
  `yarn build 6432.55s user 192.58s system 102% cpu 1:47:40.75 total`
