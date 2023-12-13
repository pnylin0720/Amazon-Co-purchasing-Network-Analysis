use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_file(path: &str) -> Result<Vec<(usize, usize)>, io::Error>{
    let file = File::open(path)?;
    let mut edges = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let Some(src_str) = parts.next(){
            if let Some(tgt_str) = parts.next(){
                if let (Ok(src), Ok(tgt)) = (src_str.parse::<usize>(), tgt_str.parse::<usize>()){
                    edges.push((src, tgt));
                }
            }
        }
    }

    Ok(edges)
}
