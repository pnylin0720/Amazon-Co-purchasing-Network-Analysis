use petgraph::graph::{Graph, UnGraph};
use petgraph::prelude::{NodeIndex};
use petgraph::EdgeType;
use std::collections::HashMap;
use petgraph::stable_graph::DefaultIx;

pub fn subgraph(edges: &Vec<(usize, usize)>, community: &Vec<usize>) -> UnGraph<usize, ()> {
    let mut graph = UnGraph::<usize, (), DefaultIx>::from_edges(edges.iter().map(|&(src, tgt)| (NodeIndex::new(src), NodeIndex::new(tgt))));

    // Convert community indices to NodeIndex
    let community_nodes: Vec<NodeIndex> = community.iter().map(|&idx| NodeIndex::new(idx)).collect();

    graph.retain_nodes(|_, node| community_nodes.contains(&node));

    graph
}


pub fn calculate_product_similarity(
    edges: &Vec<(usize, usize)>,
    community: &Vec<usize>,
) -> HashMap<(usize, usize), f64> {
    let subgraph = subgraph(edges, community);
    let mut product_similarity: HashMap<(usize, usize), f64> = HashMap::new();

    for &node1 in community.iter() {
        for &node2 in community.iter() {
            if node1 != node2 {
                let common_customers_count = subgraph
                    .neighbors(NodeIndex::new(node1))
                    .filter(|&customer| subgraph.neighbors(NodeIndex::new(node2)).any(|c| c == customer))
                    .count();
                let total_customers_count =
                    subgraph.neighbors(NodeIndex::new(node1)).count()
                        + subgraph.neighbors(NodeIndex::new(node2)).count();
                let jaccard_similarity =
                    common_customers_count as f64 / total_customers_count as f64;

                product_similarity.insert((node1, node2), jaccard_similarity);
            }
        }
    }
    product_similarity
}

pub fn analyze_friend_of_friend(
    edges: &Vec<(usize, usize)>,
    community: &Vec<usize>,
    product_similarity: &HashMap<(usize, usize), f64>,
) {
    let subgraph = subgraph(edges, community);

    let mut common_customers: HashMap<(usize, usize), usize> = HashMap::new();

    for node1 in community.iter().flat_map(|&node| {
        subgraph.neighbors(NodeIndex::new(node)).collect::<Vec<_>>()
    }) {
        for node2 in community.iter().flat_map(|&node| subgraph.neighbors(NodeIndex::new(node))) {
            let common_customers_count = subgraph
                .neighbors(NodeIndex::new(node1.index()))
                .filter(|&n| n == NodeIndex::new(node2.index()))
                .count();
            common_customers.insert((node1.index(), node2.index()), common_customers_count);
        }
    }

    let mut sorted_pairs: Vec<((usize, usize), usize)> = common_customers.into_iter().collect();
    sorted_pairs.sort_by(|(_, count1), (_, count2)| count2.cmp(count1));

    println!("\nFriend-of-Friend Analysis in Community {}: ", community[0]);

    for ((node1, node2), common_customers_count) in sorted_pairs.iter().take(5) {
        let jaccard_similarity =
            *common_customers_count as f64 / (subgraph.neighbors(NodeIndex::new(*node1)).count()
                + subgraph.neighbors(NodeIndex::new(*node2)).count()) as f64;


        println!(
            "Pair: ({}, {}), Common Customers: {}, Jaccard Similarity: {:.2}",
            node1, node2, common_customers_count, jaccard_similarity
        );
    }
}