use bio::alignment::sparse::{find_kmer_matches_seq1_hashed, lcskpp};
use bio::alignment::sparse::{hash_kmers, HashMapFx};

/// Container for a centroid and the indexes of them.
#[derive(Default)]
pub struct Cluster<'c> {
    /// each sequence in the cluster
    members: Vec<&'c str>,
    /// sequence that forms the centroid
    centroid: &'c str,
    /// hashed centromer to perform search and similarity
    pub hashed_centroid: HashMapFx<&'c [u8], Vec<u32>>,
}

impl<'c> Cluster<'c> {
    /// Initialize a cluster with a sequence as the centroid
    pub fn new(seq: &'c str, k: usize) -> Self {
        Cluster {
            members: Vec::new(),
            centroid: seq,
            hashed_centroid: hash_kmers(seq.as_bytes(), k),
        }
    }
    /// Traverse the push to the inner sequences
    pub fn push(&mut self, seq: &'c str ) {
        self.members.push(seq)
    }
}

impl<'a> std::fmt::Debug for Cluster<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.centroid, self.members)
    }
}

/// Cluster by similarity. The elements of each cluster
/// have s similarity > `similarity_threshold` with the centroid
pub struct BucketCluster<'a> {
    /// Clusters with a similarity threshold around the centroid
    pub clusters: Vec<Cluster<'a>>,
    /// Size of k-mers to perform the search
    k: usize,
    /// Minimum similarity with the centroid to become part of a cluster
    similarity_threshold: u32,
}

impl<'c> BucketCluster<'c> {
    /// Initialize empty bucket of clusters
    pub fn new(k: usize, similarity_threshold: u32) -> Self {
        BucketCluster {
            clusters: Vec::new(),
            k,
            similarity_threshold,
        }
    }

    fn get_best_cluster(&self, seq: &str) -> Option<usize> {
        let query = seq.as_bytes();
        let mut max_score = 0;
        let mut best_idx: Option<usize> = None;
        for (i, center) in self.clusters.iter().enumerate() {
            let matches = find_kmer_matches_seq1_hashed(&center.hashed_centroid, query, self.k);
            let score = lcskpp(&matches, self.k).score;
            if score > self.similarity_threshold && max_score < score {
                best_idx = Some(i);
                max_score = score;
            }
        }
        best_idx
    }

    /// Add the sequence to the best matched cluster or create a new cluster
    /// if the sequence is not inside the threshold of any centroid
    pub fn push(&mut self, seq: &'c str) {
        if let Some(idx) = self.get_best_cluster(seq) {
            self.clusters[idx].push(seq);
        } else {
            self.clusters.push(Cluster::new(seq, self.k))
        }
    }
}