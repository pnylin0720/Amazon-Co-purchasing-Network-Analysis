use petgraph::algo::tarjan_scc;
use petgraph::prelude::{NodeIndex, UnGraph};
use petgraph::stable_graph::DefaultIx;

pub fn find_communities(graph_data: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut graph = UnGraph::<usize, (), DefaultIx>::from_edges(
        graph_data.iter().map(|&(src, tgt)| (NodeIndex::new(src), NodeIndex::new(tgt))),
    );

    let mut node_to_component = vec![0; graph.node_count()];
    let mut stack = Vec::new();
    let mut index = 0;
    let mut components = Vec::new();

    for node in graph.node_indices() {
        if node_to_component[node.index()] == 0 {
            let mut lowlink = index;
            let mut current = Some((node, 0));

            while let Some((n, i)) = current {
                if node_to_component[n.index()] == 0 {
                    node_to_component[n.index()] = 1;
                    stack.push((n, i));
                    index += 1;

                    for neighbor in graph.neighbors(n) {
                        if node_to_component[neighbor.index()] == 0 {
                            current = Some((neighbor, i));
                            break;
                        } else if node_to_component[neighbor.index()] == 1 {
                            lowlink = lowlink.min(i);
                        }
                    }
                } else {
                    current = None;
                    lowlink = lowlink.min(i);
                }
            }

            if let Some((start, _)) = stack.pop() {
                while let Some((n, i)) = stack.pop() {
                    if n == start {
                        break;
                    }
                    lowlink = lowlink.min(i);
                }
            }

            let mut component = Vec::new();
            current = Some((node, 0));

            while let Some((n, i)) = current {
                if node_to_component[n.index()] == 1 {
                    node_to_component[n.index()] = 2;

                    for neighbor in graph.neighbors(n) {
                        if node_to_component[neighbor.index()] == 1 {
                            current = Some((neighbor, i));
                            break;
                        }
                    }

                    component.push(n.index());
                } else {
                    current = None;
                }
            }

            components.push(component);
        }
    }

    components
}

