//! Chordclust implements similarity clustering using [rust-bio](https://github.com/rust-bio/rust-bio).
//!
//! ## Algorithm
//! The algorithm is a greedy search, similar to what is explained in
//! https://www.drive5.com/usearch/manual/uclust_algo.html. It uses similarity
//! instead of identity (for now)
//!
//! 1. Sort by sequence length (bigger is first).
//! 2. For each sequence, compare it with the database of centroids:
//!   * If identity with best match > T: add to cluster of best match.
//!   * Else: form a new cluster.
pub mod cluster;
use cluster::BucketCluster;

mod hierarchical;
pub use hierarchical::cluster_hierarchical;

use bio::io::fasta;
use std::io::Read;

/// Read the sequences inside a buffer in FASTA format and store it in a sorted
/// `Vec<String>` by length. Based on the greedy nature of the algorithm, the
/// first sequence that is seen will form a cluster,
///
/// # Examples
/// ```
/// use chordclust::read_fasta_sorted;
///
/// const FASTA_FILE: &[u8] = b">id desc
/// AAAA
/// > id2 another|desc
/// AAAATTT
/// > id3 final|desc
/// AAUUT
/// ";
/// let vec = read_fasta_sorted(std::io::Cursor::new(FASTA_FILE));
/// println!("{:#?}", vec);
/// assert!(vec[..(vec.len())]
///     .iter()
///     .zip(vec[1..].iter())
///     .all(|(a, b)| a.len() > b.len()));
/// ```
pub fn read_fasta_sorted<Buf: Read>(buf: Buf) -> Vec<String> {
    let records = fasta::Reader::new(buf).records();
    let mut records: Vec<_> = records
        .filter_map(|record| match record {
            Ok(r) => Some(String::from_utf8(r.seq().to_ascii_uppercase()).unwrap()),
            _ => None,
        })
        .collect();
    // Sort input sequences by length
    records.sort_unstable_by_key(|r| -(r.len() as i32));
    records
}

/// Cluster a buffer by similarity. This is to be used in examples but it is
/// not bery useful.
pub fn cluster_similarity<Buf: Read>(buf: Buf, k: usize, similarity_threshold: u32) {
    let sequences = read_fasta_sorted(buf);
    let cluster_db = cluster_slice(&sequences, k, similarity_threshold);
    println!("{:#?}", cluster_db.clusters)
}

/// Cluster a slice of `String`s by similarity. The elements of each cluster
/// have s similarity > `similarity_threshold` with the centroid. `k` is the
/// size of the k-mers used to perform the search.
///
/// # Examples
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use chordclust::{read_fasta_sorted, cluster_slice};
///
/// let f = File::open("examples/UP000000425_122586_DNA_sample.fasta").unwrap();
/// let reader = BufReader::new(f);
/// let sequences = read_fasta_sorted(reader);
/// let cluster_db = cluster_slice(&sequences, 8, 85);
/// let n_clusters = cluster_db.clusters.len();
/// assert!(1 < n_clusters);
/// assert!(n_clusters < sequences.len());
/// ```
pub fn cluster_slice(sequences: &[String], k: usize, similarity_threshold: u32) -> BucketCluster {
    let mut cluster_db = BucketCluster::new(k, similarity_threshold);
    for seq in sequences.iter() {
        cluster_db.push(seq);
    }
    cluster_db
}
