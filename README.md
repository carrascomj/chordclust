# BCLUST
Bclust implements similarity clustering using [rust-bio](https://github.com/rust-bio/rust-bio).

## Algorithm
1. Sort by sequence length (bigger is first).
2. For each sequence, compare it with the database of centromeres:
  * If identity with best match > T: add to cluster of best match.
  * Else: form a new cluster.
