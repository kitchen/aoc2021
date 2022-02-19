#![feature(stdin_forwarders)]

use day12::parse;
use std::collections::{
    HashMap,
    HashSet,
};
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, nodes) = parse(&buf).expect("unable to parse input");
    
    
    let current = "start";
    let path = Vec::new();
    dfs(&nodes, current, &path, &None);

}

fn dfs(nodes: &HashMap<&str, HashSet<&str>>, current: &str, path: &Vec<&str>, doubled: &Option<&str>) {
    let mut new_path = Vec::new();
    new_path.extend(path);
    new_path.push(current);

    if current == "end" {
        println!("path: {:?}", new_path);
        return;
    }
    
    let neighbors = nodes.get(current).unwrap();

    for neighbor in neighbors {
        let mut new_doubled: Option<&str> = *doubled;
        if neighbor.to_lowercase().to_string() == **neighbor {
            if path.contains(neighbor) && doubled.is_some() {
                continue;
            }
            
            if *neighbor == "start" {
                continue;
            }
            
            if path.contains(neighbor) {
                new_doubled = Some(*neighbor);
            }
        }
        
        dfs(&nodes, neighbor, &new_path, &new_doubled);
        
    }
}
