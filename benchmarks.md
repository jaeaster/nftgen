# Benchmarks

## Setup

The benchmarks were ran for:

- [nftgen @ commit e73f2f5](https://github.com/diligentcodoor/nftgen/tree/e73f2f5e8d229de8b57d1e7b8f3cb4a1a8c17a36)
- [Hashlips Art Engine @ commit d8ee279](https://github.com/HashLips/hashlips_art_engine/tree/d8ee279043d2d4a8de3bdfac0d89d0e966fb04a2)

The benchmarks were run with the following specs:

- Macbook Pro
- Apple M1 Max chip
- 32GB RAM
- MacOS 12.4

The benchmarks used the built-in shell function `time`, which measures CPU time and real-clock time

## Conclusion

nftgen is 2 - 3X faster than Hashlips Art Engine for N >= 100 in terms of real clock time.
nftgen uses significantly more CPU time as it parallelizes the workload to maximize the usage of multiple CPU cores.

## Results

## N = 10

### nftgen

`NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen -n 10 21.89s user 0.77s system 792% cpu 2.859 total`

### Hashlips

`yarn build 5.70s user 0.29s system 99% cpu 5.999 total`

## N = 100

### nftgen

`NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen -n 100 180.19s user 2.77s system 832% cpu 21.988 total`

### Hashlips

`yarn build 64.58s user 1.94s system 100% cpu 1:06.15 total`

## N = 1000

### nftgen

`NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen -n 1000 1768.36s user 20.95s system 887% cpu 3:21.53 total`

### Hashlips

`yarn build 635.19s user 17.88s system 100% cpu 10:47.88 total`

## N = 10,000

### nftgen

`NFTGEN_CONFIG_PATH=./config RUST_LOG=info target/release/nftgen 17644.56s user 199.67s system 540% cpu 55:02.88 total`

### Hashlips

`yarn build 6432.55s user 192.58s system 102% cpu 1:47:40.75 total`
