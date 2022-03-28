use std::sync::Mutex;
use node_key::NodeKey;

use super::*;

#[test]
fn get_nodes_and_keys_test() {
    let test_vec_node = Vec::from([NodeKey{node: "1".to_string(), key: "1".to_string()}, 
                                                NodeKey{node: "2".to_string(), key: "2".to_string()}, 
                                                NodeKey{node: "3".to_string(), key: "3".to_string()}]);
    let nodes = Arc::new(Mutex::new(test_vec_node));

    let nodes_and_keys = get_nodes_and_keys(nodes.lock().unwrap());
    let node_key1 = "node: 1, key: 1".to_string();
    let node_key2 = "node: 2, key: 2".to_string();
    let node_key3 = "node: 3, key: 3".to_string();

    assert!(nodes_and_keys.contains(&node_key1) && nodes_and_keys.contains(&node_key2) && nodes_and_keys.contains(&node_key3));
}

#[test]
fn get_random_path_test() {
    let index = get_random_path(5);
    let all_index:[usize;5] = [0,1,2,3,4];

    for i in index {
        assert!(all_index.contains(&i));
    }
}