mod community_detection;
mod friend_analysis;
mod read_file;

use community_detection::find_communities;
use friend_analysis::{calculate_product_similarity, analyze_friend_of_friend};
use read_file::read_file;
use std::thread;

fn print_community_details(component: usize, community: &[usize]) {
    println!("Component {}: {:?}", component, community);
}


fn main() {
    let file_path = "amazon0601.txt";

    let edges = match read_file(file_path) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Error reading data from file: {}", err);
            return;
        }
    };
    let subset_size = 1000;
    let edges_subset: Vec<_> = edges.iter().cloned().take(subset_size).collect();
    let communities = find_communities(edges.clone());

    for (component, community) in communities.iter().enumerate() {
        print_community_details(component, community);
        let product_similarity = calculate_product_similarity(&edges, community);
        analyze_friend_of_friend(&edges_subset, community, &product_similarity);
    }

}

#[test]
fn test_social_network_analysis() {
    let edges = vec![(1, 2), (2, 3), (3, 1), (4, 5)];
    let communities = find_communities(edges.clone());

    let community = communities.first().unwrap();
    let jaccard_similarity = calculate_product_similarity(&edges, community);

    let file_path = "amazon0601.txt";
    let edges_from_file = read_file(file_path);

    assert_eq!(communities.len(), 3);
    assert_eq!(jaccard_similarity.len(), 0);
    assert_eq!(edges_from_file.unwrap().len(), 3387388);
}