# h3o-zip — H3 compression algorithm

[![Crates.io](https://img.shields.io/crates/v/h3o-zip.svg)](https://crates.io/crates/h3o-zip)
[![Docs.rs](https://docs.rs/h3o-zip/badge.svg)](https://docs.rs/h3o-zip)
[![CI Status](https://github.com/HydroniumLabs/h3o-zip/actions/workflows/ci.yml/badge.svg)](https://github.com/HydroniumLabs/h3o-zip/actions)
[![Coverage](https://img.shields.io/codecov/c/github/HydroniumLabs/h3o-zip)](https://app.codecov.io/gh/HydroniumLabs/h3o-zip)
[![License](https://img.shields.io/badge/license-BSD-green)](https://opensource.org/licenses/BSD-3-Clause)

This library allows to compress an H3 cell set into a compacted space-efficient
representation.

It provides the Compressed H3 Tree codec (CHT).

This is especially useful for on-disk storage or on-wire transmission.

## Compression results

### Dense set

Test data: the 54,812 H3 cells at resolution 11 that covers Paris (contiguous shape, no
holes).

| Format         | size (in bytes) | bits/index  |
| :------------- | --------------: | ----------: |
| Raw            |         438 496 |       64.00 |
| CHT            |          16 330 |        2.38 |
| Compact        |          16 192 |        2.36 |
| Compact+CHT    |             933 |        0.14 |

Dense set composed of contiguous indexes are the easier to compress. We can see
that the `compact` routine from H3 is really efficient and performs as well as
CHT by encoding an index on ~2 bits.

Now, the interesting part is that by first compacting the indexes set and then
applying CHT we can reduce the size even more (the resulting payload is ~470x
smaller than the original one)!

This is due to the fact that H3 compaction, unlike CHT, doesn't really compress
the data (an index is always encoded on 64-bit) but reduces the number of
indexes by replace groups of children by their ancestors.

The denser the set is, the higher is the compression: mainland France at
resolution 11 (267,532,208 indexes, 1.99 Gio) is compacted and compressed down
to 100.93Kio!

### Sparse set

Test data: the 690,451 H3 cells at resolution 11 that covers cycle lanes in
mainland France.

| Format         | size (in bytes) | bits/index  |
| :------------- | --------------: | ----------: |
| Raw            |       5 523 608 |       64.00 |
| Compact        |       5 519 768 |       63.95 |
| CHT            |         539 405 |        6.25 |
| Compact+CHT    |         539 265 |        6.25 |

Sparse sets like this one are harder to compress because there is less
redundancy to exploit.

H3 compaction completely falls apart here (only shaving 480 indexes from the
600k), since it cannot find enough complete groups of children to replace them
by their ancestors.

CHT, on the other hand, still produces remarkable results (though not as good as
the one on the dense sets), producing a payload 10x  smaller than the original
one and using ~6.25 bits/index.

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install h3o-zip`

## Usage

Load the shape of a city, compute the H3 coverage at resolution 10 and save the
compressed result on disk:

```rust
use geojson::GeoJson;
use h3o::{
    geom::{Geometry, ToCells},
    CellIndex, Resolution,
};
use std::{fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = BufReader::new(File::open("city.geojson")?);
    let geojson = GeoJson::from_reader(file)?;
    let geometry = Geometry::try_from(&geojson)?;

    let mut file = File::create("coverage.cht")?;
    h3o_zip::compress(
        &mut file,
        CellIndex::compact(geometry.to_cells(Resolution::Ten))?,
    )?;

    Ok(())
}
```

## License

[BSD 3-Clause](./LICENSE)
