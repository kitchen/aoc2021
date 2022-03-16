#![feature(stdin_forwarders)]

use day15::parse_map;
use std::io::{self, Read};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, node_graph) = parse_map(buf);
    
    let src = (0,0);
    let dst = (99,99);
    
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(State { cost: 0, position: src });
    while let Some(State { cost: cost, position: position }) = queue.pop() {
        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);
        if position == dst {
            println!("got to the end! total risk level was {}", cost);
            break;
        }
        
        let neighbor_candidates = vec![
            (position.0, position.1 - 1), // up
            (position.0 - 1, position.1), // left
            (position.0 + 1, position.1), // right
            (position.0, position.1 + 1), // down
        ];
        for neighbor in neighbor_candidates {
            if visited.contains(&neighbor) {
                continue;
            }
            match node_graph.get(&neighbor) {
                Some(next_cost) => {
                    println!("adding neighbor {}, {} with cost {} to queue", neighbor.0, neighbor.1, cost + next_cost);
                    queue.push(State { cost: cost + next_cost, position: neighbor });
                },
                None => {},
            }
        }
    }
    
    println!("we did it!");



}
