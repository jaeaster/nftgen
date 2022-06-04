# nftgen

A CLI tool and library for generating NFT images and metadata from layers of PNGs

## Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustup-init

# Clone the project
git clone git@github.com:diligentcodoor/nftgen.git
cd nftgen

# Build the executable
cargo build --release

ls ./target/release/nftgen
```

## Usage

```bash
USAGE:
    nftgen [OPTIONS] --num <NUM> --layers-path <LAYERS_PATH> --output-path <OUTPUT_PATH> --collection-name <COLLECTION_NAME> --description <DESCRIPTION> --base-uri <BASE_URI>

OPTIONS:
    -b, --base-uri <BASE_URI>                  Base URI for assets in the collection
    -c, --collection-name <COLLECTION_NAME>    Name of the collection
    -d, --description <DESCRIPTION>            Description for the collection
    -h, --help                                 Print help information
    -l, --layers-path <LAYERS_PATH>            path to root directory of NFT layers
        --layers-order <LAYERS_ORDER>...       Order of NFT layers from back to front
    -n, --num <NUM>                            Number of NFTs to generate
    -o, --output-path <OUTPUT_PATH>            path to root directory of NFT layers
    -V, --version                              Print version information
```

## Example

```bash
nftgen
  --num=10000
  --layers-path=layers
  --output-path=output
  --layers-order=Background,Face,Nose
  --collection-name=The best collection
  --description=A very descriptive text of the best collection
  --base-uri=ipfs://123234
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
--base-uri=ipfs://123
```
