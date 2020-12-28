use super::cluster::BucketCluster;

/// Cluster a slice of `String`s by similarity in a groups inside a similarity
/// threshold around the centroids. The centroids are then passed to a second
/// clustering process as sequences to be clustered with a lower treshold.
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
pub fn cluster_hierarchical<'a>(
    sequences: &'a [String],
    k: usize,
    similarity_thresholds: &'a [u32],
) -> BucketCluster<'a> {
    let mut cluster_db = BucketCluster::new(k, similarity_thresholds[0]);
    for seq in sequences.iter() {
        cluster_db.push(seq);
    }
    let mut cluster_dbs: Vec<BucketCluster> = Vec::with_capacity(similarity_thresholds.len() - 1);
    cluster_dbs.push(cluster_db);
    for (i, &threshold) in similarity_thresholds.iter().enumerate() {
        let mut cluster_db = BucketCluster::new(k, threshold);
        cluster_dbs[i]
            .clusters
            .iter()
            .map(|(_, cluster)| cluster.centroid)
            .for_each(|seq| cluster_db.push(seq));
        cluster_dbs.push(cluster_db);
    }
    cluster_dbs.reverse();
    let mut clust_above = cluster_dbs.pop().unwrap();
    while let Some(mut bucket) = cluster_dbs.pop() {
        bucket.clusters.iter_mut().for_each(|(_, cluster)| {
            let ll = cluster.members.clone();
            let cluster_new = ll
                .iter()
                .flat_map(|seq| clust_above.clusters[seq].members.clone());
            cluster.members.extend(cluster_new);
        });
        clust_above = bucket;
    }

    clust_above
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_fasta_sorted;
    use std::fs::File;
    use std::io::BufReader;
    #[test]
    fn cluster_hierarchical_works() {
        let f = File::open("examples/UP000000425_122586_DNA_sample.fasta").unwrap();
        let reader = BufReader::new(f);
        let sequences = read_fasta_sorted(reader);
        let thresholds = [90, 85, 70];
        let cluster_db = cluster_hierarchical(&sequences, 8, &thresholds);
        let n_clusters = cluster_db.clusters.len();
        assert!(1 < n_clusters);
        assert!(n_clusters < sequences.len());
    }
}