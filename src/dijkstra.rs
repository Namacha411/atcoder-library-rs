use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    to: usize,
    cost: usize,
}

impl Edge {
    pub fn new(to: usize, cost: usize) -> Edge {
        Edge { to, cost }
    }
}

pub type Graph = [Vec<Edge>];
pub fn dijkstra(graph: &Graph, start: usize) -> Vec<usize> {
    let inf = 1 << 60;
    let len = graph.len();
    let mut dist = vec![inf; len];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((dist[start], start)));
    while let Some(Reverse((d, v))) = heap.pop() {
        if dist[v] < d {
            continue;
        }
        for &Edge { to, cost } in graph[v].iter() {
            if dist[to] > dist[v] + cost {
                dist[to] = dist[v] + cost;
                heap.push(Reverse((dist[to], to)));
            }
        }
    }

    dist
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dijkstra_test() {
        let start = 0;
        let mut graph = vec![vec![]; 5];
        graph[0].push(Edge::new(1, 2));
        graph[1].push(Edge::new(4, 5));
        graph[1].push(Edge::new(2, 4));
        graph[0].push(Edge::new(3, 1));
        graph[3].push(Edge::new(2, 3));
        graph[2].push(Edge::new(4, 1));

        let dist = dijkstra(&graph, start);
        assert_eq!(dist[4], 5);
    }
}