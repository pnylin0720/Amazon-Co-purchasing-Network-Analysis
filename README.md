# Amazon Co-purchasing Network Analysis

A Rust-based network analysis tool for exploring Amazon product co-purchasing relationships and detecting communities within the product network.


## Overview

This project analyzes the Amazon Product Co-purchasing Network dataset to uncover relationships between products and measure their similarities. It focuses on:

- Community detection via graph algorithms
- Calculating product similarities (e.g. Jaccard similarity)
- Analyzing friend-of-friend relationships within detected communities

A detailed report is provided in the PDF file: [Amazon Co-purchasing network analysis Report.pdf](https://github.com/pnylin0720/Amazon-Co-purchasing-Network-Analysis/blob/main/Amazon%20Co-purchasing%20network%20analysis%20Report.pdf)


##  Features

 **Community Detection**  
- Implements Tarjan’s Strongly Connected Components (SCC) algorithm to identify communities.

 **Product Similarity Analysis**  
- Calculates Jaccard similarity between products based on shared neighbors.

 **Friend-of-Friend Analysis**  
- Examines connections between products within detected communities.

 **Scalable Processing**  
- Efficiently processes large datasets using Rust’s performance and memory safety.


##  Project Structure

- `main.rs` – Main application entry point.
- `community_detection.rs` – Tarjan’s SCC implementation for community detection.
- `friend_analysis.rs` – Functions for computing product similarity and friend-of-friend analysis.
- `read_file.rs` – Functions for reading the co-purchasing dataset.
- `amazon0601.txt.zip` – Compressed dataset file.
- `Amazon Co-purchasing network analysis Report.pdf` – Project report.
- `Cargo.lock` – Rust dependency lock file.
