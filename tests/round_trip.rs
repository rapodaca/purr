use std::{ fs, path::PathBuf };

use pretty_assertions::assert_eq;

use purr::read::read;
use purr::graph::from_tree;
use purr::tree::{ from_graph, Writer };

#[test]
fn round_trip() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("./tests/round_trip.smi");

    let file = fs::read_to_string(&path).expect("test data");
    let entries = file.split("\n").collect::<Vec<_>>();

    for entry in entries {
        let graph = from_tree(read(entry).unwrap().root);
        let tree = from_graph(graph).unwrap();

        assert_eq!(Writer::write(&tree), entry)
    }
}