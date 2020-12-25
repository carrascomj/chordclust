use chordclust::cluster_similarity;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("examples/UP000000425_122586_DNA.fasta").unwrap();
    let reader = BufReader::new(f);
    let cluster_db = cluster_similarity(reader, 8, 85);
    println!("{:#?}", cluster_db)

}

