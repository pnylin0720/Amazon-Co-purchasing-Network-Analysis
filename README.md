# Amazon Co-purchasing Network Analysis
A Rust-based network analysis tool for analyzing Amazon product co-purchasing relationships and detecting communities within the product network.
Overview
This project analyzes the Amazon Product Co-purchasing network dataset to understand relationships between products and their similarities. The analysis focuses on community detection using graph algorithms and measuring product similarities through various metrics.
Features

##### Community Detection: Uses Tarjan's Strongly Connected Components (SCC) algorithm to identify communities in the co-purchasing network
## Product Similarity Analysis: Calculates Jaccard similarity coefficients between products based on common neighbors
## Friend-of-Friend Analysis: Analyzes relationships between products within detected communities
## Scalable Processing: Handles large datasets efficiently using Rust's performance capabilities


### Key Components
  1. Community Detection (community_detection.rs)    
  - Implements Tarjan's SCC algorithm for undirected graphs   
  - Uses the petgraph crate's UnGraph and tarjan_scc algorithms    
  - Identifies strongly connected components through depth-first search    

2. Friend Analysis (friend_analysis.rs)
Contains three main functions:

subgraph: Creates undirected graphs from edge vectors and community node indices
calculate_product_similarity: Computes Jaccard similarity between products based on common neighbors
analyze_friend_of_friend: Analyzes relationships and counts common customers between product pairs

3. File Reading (read_file.rs)

Reads dataset containing product pairs separated by whitespace
Returns data as vectors of tuples for further processing
