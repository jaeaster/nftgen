# nftgen

## Overview

A CLI tool for generating NFTs and hosting them for free w/ IPFS and [NFT.Storage](https://nft.storage/)

nftgen provides two key utilities:

- Generate NFT images and metadata by layering PNGs (traits) with provided rarities
- Upload this data to IPFS and store permanently for free via [NFT.Storage](https://nft.storage/), a service provided by [Protocol Labs](https://protocol.ai/)

## Dependencies

nftgen is written entirely in Rust, except for functionality related to IPFS and packaging data into Content Archives.
This may change in the future once the [Rust IPFS implementation](https://github.com/rs-ipfs/rust-ipfs) matures.

As a result, if you want to use the `nftgen upload` subcommand, you must install an official [ipfs command line implementation](https://docs.ipfs.io/install/command-line/#official-distributions) and ensure it is in your path.

You will also need to create an account and generate an API key for [NFT.Storage](https://nft.storage/) if you want to use `nftgen upload`

## Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustup-init

# Clone the project
git clone git@github.com:diligentcodoor/nftgen.git
cd nftgen

# Install ipfs CLI - See above

# Build the executable
cargo build --release

./target/release/nftgen --help
```

## Usage

```bash
nftgen 0.1.0
Generate images and metadata for NFTs by layering PNGs together

USAGE:
    nftgen <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    generate    Generate nft images and metadata [aliases: g]
    help        Print this message or the help of the given subcommand(s)
    upload      Upload nft images and metadata to IPFS [aliases: u]
```

## Example

```bash
nftgen generate
  --num=10000
  --layers-path=layers
  --output-path=output
  --layers-order=Background,Face,Nose
  --collection-name=The best collection
  --description=A very descriptive text of the best collection
```

```bash
nftgen upload
  --api-key=SuperSecretNftStorageKey
  --output-path=output
```

## Config File

Instead of passing arguments via the command line, you can use a configuration file by setting the `NFTGEN_CONFIG_PATH` environment variable. Arguments passed via the command line will overwrite args specified in the configuration file. See the [config file example](./config.example)

```bash
NFTGEN_CONFIG_PATH=./config nftgen
```

Setting up a configuration file is simple. nftgen will not look in any predetermined directory for a config file automatically. Instead, you need to set the `NFTGEN_CONFIG_PATH` environment variable to the file path of your config file. Once the environment variable is set, open the file and just type in the flags you want set automatically. There are only two rules for describing the format of the config file:

1. Every line is a shell argument, after trimming whitespace.
2. Lines starting with \# (optionally preceded by any amount of whitespace) are ignored.

In particular, there is no escaping. Each line is given to nftgen as a single command line argument verbatim.

Here's an example of a configuration file, which demonstrates some of the formatting peculiarities:

```bash
# Example
# Can separate options that take a value with a newline
--num
10

# Extra whitespace is allowed

# Can separate options that take a value with an equals sign "="
--layers-path=./layers

# Be careful, separating by a space is not valid, e.g.
# --output-path output

--output-path
output

--layers-order=Background,Face,Nose
--collection-name=Dope Collection Name
--description=A very cool collection
```
