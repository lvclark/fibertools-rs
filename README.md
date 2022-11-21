# `fibertools-rs`
[![Actions Status](https://github.com/mrvollger/fibertools-rs/workflows/CI/badge.svg)](https://github.com/mrvollger/rustybam/actions)
[![Conda (channel only)](https://img.shields.io/conda/vn/bioconda/fibertools-rs?color=green)](https://anaconda.org/bioconda/fibertools-rs)
 [![Downloads](https://img.shields.io/conda/dn/bioconda/fibertools-rs?color=green)](https://anaconda.org/bioconda/fibertools-rs)
[![crates.io version](https://img.shields.io/crates/v/fibertools-rs)](https://crates.io/crates/fibertools-rs)
[![crates.io downloads](https://img.shields.io/crates/d/fibertools-rs?color=orange&label=downloads)](https://crates.io/crates/fibertools-rs)
[![DOI](https://zenodo.org/badge/517338593.svg)](https://zenodo.org/badge/latestdoi/517338593)

`fibertools-rs` a CLI tool for interacting with fiberseq bam files.

# Install
Installation from `cargo` requires a recent version of `gcc` and `cmake`. I have tested and recommend these versions of `gcc` and `cmake`, though other versions may work:
```bash
module load gcc/10.2.0 cmake/3.21.1
```
## From [![crates.io version](https://img.shields.io/crates/v/fibertools-rs)](https://crates.io/crates/fibertools-rs) (recommended)
```
cargo install fibertools-rs
```
[How to install `cargo`.](https://doc.rust-lang.org/cargo/getting-started/installation.html)
### Install with support for CNN m6A prediction
```bash
cargo install fibertools-rs --features cnn
```


## From [![Conda (channel only)](https://img.shields.io/conda/vn/bioconda/fibertools-rs?color=green)](https://anaconda.org/bioconda/fibertools-rs) (latest version not always available)
```
mamba install -c bioconda fibertools-rs
```
## From `github` (active development)
```
cargo install --git https://github.com/mrvollger/fibertools-rs
```
# `fibertools-rs`

[Help page for fibertools](/docs/ft--help.md)

# `ft extract`
[Help page for extract](/docs/ft-extract-help.md). Extracts fiberseq data from a bam file into plain text.


### `ft-extract --all`
The extract all option is a special option that tries to extract all the fiberseq data into a tabular format. The following is an image of the output. Note that the [column names](/docs/ft-all-columns.md) will be preserved across different software versions (unless otherwise noted); however, the order may change and new columns may be added. Therefore, when loading the data (with `pandas` e.g.) be sure to use the column names as opposed to indexes for manipulation.
![ft-extract all](/images/ft-extract-all.png)


# `ft-center`
[Help page for center](/docs/ft-center-help.md). Center fiberseq reads (bam) around reference position(s).

![Center](/images/center.png)

# `ft-predict-m6a`
[Help page for predict-m6a](/docs/ft-predict-m6a-help.md). Predict m6A positions using HiFi kinetics data and encode the results in the MM and ML bam tags.

# TODO
- [ ] Add `rustybam` stats to ft `all`
- [x] Add CNN to CI
- [x] Option to keep full float info in predict-m6a
- [x] Add option for all format but without sequence
- [x] handle broken pipe errors in `ft-extract`
- [x] add rq to all
