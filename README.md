[![Crates.io](https://img.shields.io/crates/v/chordclust.svg)](https://crates.io/crates/chordclust)
[![Documentation](https://docs.rs/chordclust/badge.svg)](https://docs.rs/chordclust/)
[![Build](https://github.com/carrascomj/chordclust/workflows/build/badge.svg)](https://github.com/carrascomj/chordclust)
[![Codecov](https://codecov.io/github/carrascomj/chordclust/coverage.svg?branch=trunk)](https://codecov.io/gh/carrascomj/chordclust)

# chordclust

## BCLUST
Bclust implements similarity clustering using [rust-bio](https://github.com/rust-bio/rust-bio).

### Algorithm
The algorithm is a greedy search, similar to what is explained in
https://www.drive5.com/usearch/manual/uclust_algo.html. It uses similarity
instead of identity (for now)

1. Sort by sequence length (bigger is first).
2. For each sequence, compare it with the database of centroids:
  * If identity with best match > T: add to cluster of best match.
  * Else: form a new cluster.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

> README.md is automatically generated on CI using [cargo-readme](https://github.com/livioribeiro/cargo-readme). Please, modify README.tpl or lib.rs instead (check [the github worflow](https://github.com/carrascomj/rust_sbml/blob/trunk/.github/workflows/readme.yml) for more details).
