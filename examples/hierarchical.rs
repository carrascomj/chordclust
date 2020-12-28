use chordclust::{cluster_hierarchical, read_fasta_sorted};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("examples/UP000000425_122586_DNA_sample.fasta").unwrap();
    let reader = BufReader::new(f);
    let sequences = read_fasta_sorted(reader);
    let similarity_thresholds = &[100, 90, 85, 65, 30, 20];
    let cluster_db = cluster_hierarchical(&sequences, 8, similarity_thresholds);
    println!("{:#?}", cluster_db.clusters)
}
